pub mod items;
mod toolbar_controls;

mod diagnostic_renderer;

#[cfg(test)]
mod diagnostics_tests;

use anyhow::Result;
use collections::{BTreeSet, HashMap, HashSet};
use diagnostic_renderer::DiagnosticBlock;
use editor::{
    AnchorRangeExt, DEFAULT_MULTIBUFFER_CONTEXT, DiagnosticRenderer, Editor, EditorEvent,
    ExcerptId, ExcerptRange, MultiBuffer, PathKey, RangeToAnchorExt, ToOffset, ToPoint,
    display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId, RenderBlock},
    scroll::Autoscroll,
};
use gpui::{
    AnyElement, AnyView, App, AsyncApp, Context, Entity, EventEmitter, FocusHandle, Focusable,
    Global, HighlightStyle, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    Styled, StyledText, Subscription, Task, WeakEntity, Window, actions, div, svg,
};
use language::{
    Bias, Buffer, BufferRow, BufferSnapshot, Diagnostic, DiagnosticEntry, DiagnosticSeverity,
    OffsetRangeExt, Point, Selection, SelectionGoal, ToTreeSitterPoint,
};
use lsp::LanguageServerId;
use project::{DiagnosticSummary, Project, ProjectPath, project_settings::ProjectSettings};
use settings::Settings;
use std::{
    any::{Any, TypeId},
    cmp,
    cmp::Ordering,
    mem,
    ops::{Range, RangeInclusive},
    sync::Arc,
    time::Duration,
};
use theme::ActiveTheme;
pub use toolbar_controls::ToolbarControls;
use ui::{Icon, IconName, Label, h_flex, prelude::*};
use util::ResultExt;
use workspace::{
    ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle, TabContentParams},
    searchable::SearchableItemHandle,
};

actions!(diagnostics, [Deploy, ToggleWarnings]);

struct IncludeWarnings(bool);
impl Global for IncludeWarnings {}

pub fn init(cx: &mut App) {
    editor::set_diagnostic_renderer(diagnostic_renderer::DiagnosticRenderer {}, cx);
    cx.observe_new(ProjectDiagnosticsEditor::register).detach();
}

struct ProjectDiagnosticsEditor {
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    editor: Entity<Editor>,
    summary: DiagnosticSummary,
    excerpts: Entity<MultiBuffer>,
    path_states: Vec<PathState>,
    paths_to_update: BTreeSet<(ProjectPath, Option<LanguageServerId>)>,
    include_warnings: bool,
    context: u32,
    update_excerpts_task: Option<Task<Result<()>>>,
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
    blocks: HashSet<CustomBlockId>,
    block_count: usize,
}

impl EventEmitter<EditorEvent> for ProjectDiagnosticsEditor {}

const DIAGNOSTICS_UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

impl Render for ProjectDiagnosticsEditor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let warning_count = if self.include_warnings {
            self.summary.warning_count
        } else {
            0
        };

        let child = if warning_count + self.summary.error_count == 0 {
            let label = if self.summary.warning_count == 0 {
                SharedString::new_static("No problems in workspace")
            } else {
                SharedString::new_static("No errors in workspace")
            };
            v_flex()
                .key_context("EmptyPane")
                .size_full()
                .gap_1()
                .justify_center()
                .items_center()
                .text_center()
                .bg(cx.theme().colors().editor_background)
                .child(Label::new(label).color(Color::Muted))
                .when(self.summary.warning_count > 0, |this| {
                    let plural_suffix = if self.summary.warning_count > 1 {
                        "s"
                    } else {
                        ""
                    };
                    let label = format!(
                        "Show {} warning{}",
                        self.summary.warning_count, plural_suffix
                    );
                    this.child(
                        Button::new("diagnostics-show-warning-label", label).on_click(cx.listener(
                            |this, _, window, cx| {
                                this.toggle_warnings(&Default::default(), window, cx);
                                cx.notify();
                            },
                        )),
                    )
                })
        } else {
            div().size_full().child(self.editor.clone())
        };

        div()
            .key_context("Diagnostics")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .on_action(cx.listener(Self::toggle_warnings))
            .child(child)
    }
}

impl ProjectDiagnosticsEditor {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(Self::deploy);
    }

    fn new_with_context(
        context: u32,
        include_warnings: bool,
        project_handle: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project_event_subscription =
            cx.subscribe_in(&project_handle, window, |this, project, event, window, cx| match event {
                project::Event::DiskBasedDiagnosticsStarted { .. } => {
                    cx.notify();
                }
                project::Event::DiskBasedDiagnosticsFinished { language_server_id } => {
                    log::debug!("disk based diagnostics finished for server {language_server_id}");
                    this.update_stale_excerpts(window, cx);
                }
                project::Event::DiagnosticsUpdated {
                    language_server_id,
                    path,
                } => {
                    this.paths_to_update
                        .insert((path.clone(), Some(*language_server_id)));
                    this.summary = project.read(cx).diagnostic_summary(false, cx);
                    cx.emit(EditorEvent::TitleChanged);

                    if this.editor.focus_handle(cx).contains_focused(window, cx) || this.focus_handle.contains_focused(window, cx) {
                        log::debug!("diagnostics updated for server {language_server_id}, path {path:?}. recording change");
                    } else {
                        log::debug!("diagnostics updated for server {language_server_id}, path {path:?}. updating excerpts");
                        this.update_stale_excerpts(window, cx);
                    }
                }
                _ => {}
            });

        let focus_handle = cx.focus_handle();
        cx.on_focus_in(&focus_handle, window, |this, window, cx| {
            this.focus_in(window, cx)
        })
        .detach();
        cx.on_focus_out(&focus_handle, window, |this, _event, window, cx| {
            this.focus_out(window, cx)
        })
        .detach();

        let excerpts = cx.new(|cx| MultiBuffer::new(project_handle.read(cx).capability()));
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(excerpts.clone(), Some(project_handle.clone()), window, cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor.disable_inline_diagnostics();
            editor
        });
        cx.subscribe_in(
            &editor,
            window,
            |this, _editor, event: &EditorEvent, window, cx| {
                cx.emit(event.clone());
                match event {
                    EditorEvent::Focused => {
                        if this.excerpts.read(cx).is_empty() {
                            window.focus(&this.focus_handle);
                        }
                    }
                    EditorEvent::Blurred => this.update_stale_excerpts(window, cx),
                    _ => {}
                }
            },
        )
        .detach();
        cx.observe_global_in::<IncludeWarnings>(window, |this, window, cx| {
            this.include_warnings = cx.global::<IncludeWarnings>().0;
            this.update_all_excerpts(window, cx);
        })
        .detach();

        let project = project_handle.read(cx);
        let mut this = Self {
            project: project_handle.clone(),
            context,
            summary: project.diagnostic_summary(false, cx),
            include_warnings,
            workspace,
            excerpts,
            focus_handle,
            editor,
            path_states: Default::default(),
            paths_to_update: Default::default(),
            update_excerpts_task: None,
            _subscription: project_event_subscription,
        };
        this.update_all_excerpts(window, cx);
        this
    }

    fn update_stale_excerpts(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.update_excerpts_task.is_some() {
            return;
        }
        let project_handle = self.project.clone();
        self.update_excerpts_task = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor()
                .timer(DIAGNOSTICS_UPDATE_DEBOUNCE)
                .await;
            loop {
                let Some((path, language_server_id)) = this.update(cx, |this, _| {
                    let Some((path, language_server_id)) = this.paths_to_update.pop_first() else {
                        dbg!("done...");
                        this.update_excerpts_task.take();
                        return None;
                    };
                    Some((path, language_server_id))
                })?
                else {
                    break;
                };

                if let Some(buffer) = project_handle
                    .update(cx, |project, cx| project.open_buffer(path.clone(), cx))?
                    .await
                    .log_err()
                {
                    this.update_in(cx, |this, window, cx| {
                        this.update_excerpts(path, language_server_id, buffer, window, cx)
                    })?
                    .await?;
                }
            }
            Ok(())
        }));
    }

    fn new(
        project_handle: Entity<Project>,
        include_warnings: bool,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_with_context(
            editor::DEFAULT_MULTIBUFFER_CONTEXT,
            include_warnings,
            project_handle,
            workspace,
            window,
            cx,
        )
    }

    fn deploy(
        workspace: &mut Workspace,
        _: &Deploy,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if let Some(existing) = workspace.item_of_type::<ProjectDiagnosticsEditor>(cx) {
            let is_active = workspace
                .active_item(cx)
                .is_some_and(|item| item.item_id() == existing.item_id());
            workspace.activate_item(&existing, true, !is_active, window, cx);
        } else {
            let workspace_handle = cx.entity().downgrade();

            let include_warnings = match cx.try_global::<IncludeWarnings>() {
                Some(include_warnings) => include_warnings.0,
                None => ProjectSettings::get_global(cx).diagnostics.include_warnings,
            };

            let diagnostics = cx.new(|cx| {
                ProjectDiagnosticsEditor::new(
                    workspace.project().clone(),
                    include_warnings,
                    workspace_handle,
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(Box::new(diagnostics), None, true, window, cx);
        }
    }

    fn toggle_warnings(&mut self, _: &ToggleWarnings, window: &mut Window, cx: &mut Context<Self>) {
        self.include_warnings = !self.include_warnings;
        cx.set_global(IncludeWarnings(self.include_warnings));
        self.update_all_excerpts(window, cx);
        cx.notify();
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.focus_handle.is_focused(window) && !self.excerpts.read(cx).is_empty() {
            self.editor.focus_handle(cx).focus(window)
        }
    }

    fn focus_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.is_focused(window) && !self.editor.focus_handle(cx).is_focused(window)
        {
            self.update_stale_excerpts(window, cx);
        }
    }

    /// Enqueue an update of all excerpts. Updates all paths that either
    /// currently have diagnostics or are currently present in this view.
    fn update_all_excerpts(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.project.update(cx, |project, cx| {
            let mut paths = project
                .diagnostic_summaries(false, cx)
                .map(|(path, _, _)| (path, None))
                .collect::<BTreeSet<_>>();
            paths.extend(
                self.path_states
                    .iter()
                    .map(|state| (state.path.clone(), None)),
            );
            let paths_to_update = std::mem::take(&mut self.paths_to_update);
            paths.extend(paths_to_update.into_iter().map(|(path, _)| (path, None)));
            self.paths_to_update = paths;
        });
        self.update_stale_excerpts(window, cx);
    }

    fn update_excerpts(
        &mut self,
        path_to_update: ProjectPath,
        server_to_update: Option<LanguageServerId>,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        // let was_empty = self.path_states.is_empty();
        let buffer_snapshot = buffer.read(cx).snapshot();
        let editor = self.editor.downgrade();
        let editor_snapshot = self
            .editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));
        cx.spawn_in(window, async move |this, mut cx| {
            let diagnostics = buffer_snapshot
                .diagnostics_in_range::<_, text::Point>(
                    Point::zero()..buffer_snapshot.max_point(),
                    false,
                )
                .filter(|d| !(d.diagnostic.is_primary && d.diagnostic.is_unnecessary))
                .collect::<Vec<_>>();

            let mut grouped: HashMap<usize, Vec<_>> = HashMap::default();
            for entry in diagnostics {
                grouped
                    .entry(entry.diagnostic.group_id)
                    .or_default()
                    .push(entry)
            }
            let mut blocks: Vec<DiagnosticBlock> = Vec::new();

            for (group_id, group) in grouped {
                let more = cx.update(|_, cx| {
                    crate::diagnostic_renderer::DiagnosticRenderer::diagnostic_blocks_for_group(
                        group,
                        buffer_snapshot.remote_id(),
                        cx,
                    )
                })?;

                for item in more {
                    let insert_pos = blocks
                        .binary_search_by(|existing| {
                            match existing.initial_range.start.cmp(&item.initial_range.start) {
                                Ordering::Equal => item
                                    .initial_range
                                    .end
                                    .cmp(&existing.initial_range.end)
                                    .reverse(),
                                other => other,
                            }
                        })
                        .unwrap_or_else(|pos| pos);

                    blocks.insert(insert_pos, item);
                }
            }

            let mut excerpt_ranges: Vec<ExcerptRange<Point>> = Vec::new();
            for b in blocks.iter() {
                let excerpt_range = context_range_for_entry(
                    b.initial_range.clone(),
                    DEFAULT_MULTIBUFFER_CONTEXT,
                    buffer_snapshot.clone(),
                    &mut cx,
                )
                .await;
                debug_assert!(
                    excerpt_ranges.last().is_none()
                        || excerpt_ranges.last().unwrap().context.start < excerpt_range.start
                );
                excerpt_ranges.push(ExcerptRange {
                    context: excerpt_range,
                    primary: b.initial_range.clone(),
                })
            }

            let (anchor_ranges, _) = this.update(cx, |this, cx| {
                this.excerpts.update(cx, |multi_buffer, cx| {
                    multi_buffer.set_excerpt_ranges_for_path(
                        PathKey::for_buffer(&buffer, cx),
                        buffer.clone(),
                        &buffer_snapshot,
                        excerpt_ranges,
                        cx,
                    )
                })
            })?;

            let editor_blocks =
                anchor_ranges
                    .into_iter()
                    .zip(blocks.into_iter())
                    .map(|(anchor, block)| {
                        let editor = editor.clone();
                        BlockProperties {
                            placement: BlockPlacement::Near(anchor.start),
                            height: Some(1),
                            style: BlockStyle::Flex,
                            render: Arc::new(move |bcx| block.render_block(editor.clone(), bcx)),
                            priority: 1,
                        }
                    });

            editor.update(cx, |editor, cx| {
                editor.display_map.update(cx, |display_map, cx| {
                    display_map.insert_blocks(editor_blocks, cx)
                });
                cx.notify()
            })
        })
    }

    #[cfg(test)]
    fn check_invariants(&self, cx: &mut Context<Self>) {
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

impl Focusable for ProjectDiagnosticsEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ProjectDiagnosticsEditor {
    type Event = EditorEvent;

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn navigate(
        &mut self,
        data: Box<dyn Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some("Project Diagnostics".into())
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, _: &App) -> AnyElement {
        h_flex()
            .gap_1()
            .when(
                self.summary.error_count == 0 && self.summary.warning_count == 0,
                |then| {
                    then.child(
                        h_flex()
                            .gap_1()
                            .child(Icon::new(IconName::Check).color(Color::Success))
                            .child(Label::new("No problems").color(params.text_color())),
                    )
                },
            )
            .when(self.summary.error_count > 0, |then| {
                then.child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::XCircle).color(Color::Error))
                        .child(
                            Label::new(self.summary.error_count.to_string())
                                .color(params.text_color()),
                        ),
                )
            })
            .when(self.summary.warning_count > 0, |then| {
                then.child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Warning).color(Color::Warning))
                        .child(
                            Label::new(self.summary.warning_count.to_string())
                                .color(params.text_color()),
                        ),
                )
            })
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Project Diagnostics Opened")
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.for_each_project_item(cx, f)
    }

    fn is_singleton(&self, _: &App) -> bool {
        false
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| {
            ProjectDiagnosticsEditor::new(
                self.project.clone(),
                self.include_warnings,
                self.workspace.clone(),
                window,
                cx,
            )
        }))
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.excerpts.read(cx).is_dirty(cx)
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.excerpts.read(cx).has_deleted_file(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.excerpts.read(cx).has_conflict(cx)
    }

    fn can_save(&self, _: &App) -> bool {
        true
    }

    fn save(
        &mut self,
        format: bool,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.save(format, project, window, cx)
    }

    fn save_as(
        &mut self,
        _: Entity<Project>,
        _: ProjectPath,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unreachable!()
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.reload(project, window, cx)
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }
}

const DIAGNOSTIC_HEADER: &str = "diagnostic header";

// fn diagnostic_header_renderer(diagnostic: Diagnostic) -> RenderBlock {
//     let (message, code_ranges) = highlight_diagnostic_message(&diagnostic, None);
//     let message: SharedString = message;
//     Arc::new(move |cx| {
//         let color = cx.theme().colors();
//         let highlight_style: HighlightStyle = color.text_accent.into();

//         h_flex()
//             .id(DIAGNOSTIC_HEADER)
//             .block_mouse_down()
//             .h(2. * cx.window.line_height())
//             .w_full()
//             .px_9()
//             .justify_between()
//             .gap_2()
//             .child(
//                 h_flex()
//                     .gap_2()
//                     .px_1()
//                     .rounded_sm()
//                     .bg(color.surface_background.opacity(0.5))
//                     .map(|stack| {
//                         stack.child(
//                             svg()
//                                 .size(cx.window.text_style().font_size)
//                                 .flex_none()
//                                 .map(|icon| {
//                                     if diagnostic.severity == DiagnosticSeverity::ERROR {
//                                         icon.path(IconName::XCircle.path())
//                                             .text_color(Color::Error.color(cx))
//                                     } else {
//                                         icon.path(IconName::Warning.path())
//                                             .text_color(Color::Warning.color(cx))
//                                     }
//                                 }),
//                         )
//                     })
//                     .child(
//                         h_flex()
//                             .gap_1()
//                             .child(
//                                 StyledText::new(message.clone()).with_default_highlights(
//                                     &cx.window.text_style(),
//                                     code_ranges
//                                         .iter()
//                                         .map(|range| (range.clone(), highlight_style)),
//                                 ),
//                             )
//                             .when_some(diagnostic.code.as_ref(), |stack, code| {
//                                 stack.child(
//                                     div()
//                                         .child(SharedString::from(format!("({code:?})")))
//                                         .text_color(color.text_muted),
//                                 )
//                             }),
//                     ),
//             )
//             .when_some(diagnostic.source.as_ref(), |stack, source| {
//                 stack.child(
//                     div()
//                         .child(SharedString::from(source.clone()))
//                         .text_color(color.text_muted),
//                 )
//             })
//             .into_any_element()
//     })
// }

fn compare_diagnostics(
    old: &DiagnosticEntry<language::Anchor>,
    new: &DiagnosticEntry<language::Anchor>,
    snapshot: &language::BufferSnapshot,
) -> Ordering {
    use language::ToOffset;

    // The diagnostics may point to a previously open Buffer for this file.
    if !old.range.start.is_valid(snapshot) || !new.range.start.is_valid(snapshot) {
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

const DIAGNOSTIC_EXPANSION_ROW_LIMIT: u32 = 32;

async fn context_range_for_entry(
    range: Range<Point>,
    context: u32,
    snapshot: BufferSnapshot,
    cx: &mut AsyncApp,
) -> Range<Point> {
    if let Some(rows) = heuristic_syntactic_expand(
        range.clone(),
        DIAGNOSTIC_EXPANSION_ROW_LIMIT,
        snapshot.clone(),
        cx,
    )
    .await
    {
        return Range {
            start: Point::new(*rows.start(), 0),
            end: snapshot.clip_point(Point::new(*rows.end(), u32::MAX), Bias::Left),
        };
    }
    Range {
        start: Point::new(range.start.row.saturating_sub(context), 0),
        end: snapshot.clip_point(Point::new(range.end.row + context, u32::MAX), Bias::Left),
    }
}

/// Expands the input range using syntax information from TreeSitter. This expansion will be limited
/// to the specified `max_row_count`.
///
/// If there is a containing outline item that is less than `max_row_count`, it will be returned.
/// Otherwise fairly arbitrary heuristics are applied to attempt to return a logical block of code.
async fn heuristic_syntactic_expand(
    input_range: Range<Point>,
    max_row_count: u32,
    snapshot: BufferSnapshot,
    cx: &mut AsyncApp,
) -> Option<RangeInclusive<BufferRow>> {
    let input_row_count = input_range.end.row - input_range.start.row;
    if input_row_count > max_row_count {
        return None;
    }

    // If the outline node contains the diagnostic and is small enough, just use that.
    let outline_range = snapshot.outline_range_containing(input_range.clone());
    if let Some(outline_range) = outline_range.clone() {
        // Remove blank lines from start and end
        if let Some(start_row) = (outline_range.start.row..outline_range.end.row)
            .find(|row| !snapshot.line_indent_for_row(*row).is_line_blank())
        {
            if let Some(end_row) = (outline_range.start.row..outline_range.end.row + 1)
                .rev()
                .find(|row| !snapshot.line_indent_for_row(*row).is_line_blank())
            {
                let row_count = end_row.saturating_sub(start_row);
                if row_count <= max_row_count {
                    return Some(RangeInclusive::new(
                        outline_range.start.row,
                        outline_range.end.row,
                    ));
                }
            }
        }
    }

    let mut node = snapshot.syntax_ancestor(input_range.clone())?;

    loop {
        let node_start = Point::from_ts_point(node.start_position());
        let node_end = Point::from_ts_point(node.end_position());
        let node_range = node_start..node_end;
        let row_count = node_end.row - node_start.row + 1;
        let mut ancestor_range = None;
        let reached_outline_node = cx.background_executor().scoped({
                 let node_range = node_range.clone();
                 let outline_range = outline_range.clone();
                 let ancestor_range =  &mut ancestor_range;
                |scope| {scope.spawn(async move {
                    // Stop if we've exceeded the row count or reached an outline node. Then, find the interval
                    // of node children which contains the query range. For example, this allows just returning
                    // the header of a declaration rather than the entire declaration.
                    if row_count > max_row_count || outline_range == Some(node_range.clone()) {
                        let mut cursor = node.walk();
                        let mut included_child_start = None;
                        let mut included_child_end = None;
                        let mut previous_end = node_start;
                        if cursor.goto_first_child() {
                            loop {
                                let child_node = cursor.node();
                                let child_range = previous_end..Point::from_ts_point(child_node.end_position());
                                if included_child_start.is_none() && child_range.contains(&input_range.start) {
                                    included_child_start = Some(child_range.start);
                                }
                                if child_range.contains(&input_range.end) {
                                    included_child_end = Some(child_range.end);
                                }
                                previous_end = child_range.end;
                                if !cursor.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        let end = included_child_end.unwrap_or(node_range.end);
                        if let Some(start) = included_child_start {
                            let row_count = end.row - start.row;
                            if row_count < max_row_count {
                                *ancestor_range = Some(Some(RangeInclusive::new(start.row, end.row)));
                                return;
                            }
                        }

                        log::info!(
                            "Expanding to ancestor started on {} node exceeding row limit of {max_row_count}.",
                            node.grammar_name()
                        );
                        *ancestor_range = Some(None);
                    }
                })
            }});
        reached_outline_node.await;
        if let Some(node) = ancestor_range {
            return node;
        }

        let node_name = node.grammar_name();
        let node_row_range = RangeInclusive::new(node_range.start.row, node_range.end.row);
        if node_name.ends_with("block") {
            return Some(node_row_range);
        } else if node_name.ends_with("statement") || node_name.ends_with("declaration") {
            // Expand to the nearest dedent or blank line for statements and declarations.
            let tab_size = cx
                .update(|cx| snapshot.settings_at(node_range.start, cx).tab_size.get())
                .ok()?;
            let indent_level = snapshot
                .line_indent_for_row(node_range.start.row)
                .len(tab_size);
            let rows_remaining = max_row_count.saturating_sub(row_count);
            let Some(start_row) = (node_range.start.row.saturating_sub(rows_remaining)
                ..node_range.start.row)
                .rev()
                .find(|row| {
                    is_line_blank_or_indented_less(indent_level, *row, tab_size, &snapshot.clone())
                })
            else {
                return Some(node_row_range);
            };
            let rows_remaining = max_row_count.saturating_sub(node_range.end.row - start_row);
            let Some(end_row) = (node_range.end.row + 1
                ..cmp::min(
                    node_range.end.row + rows_remaining + 1,
                    snapshot.row_count(),
                ))
                .find(|row| {
                    is_line_blank_or_indented_less(indent_level, *row, tab_size, &snapshot.clone())
                })
            else {
                return Some(node_row_range);
            };
            return Some(RangeInclusive::new(start_row, end_row));
        }

        // TODO: doing this instead of walking a cursor as that doesn't work - why?
        let Some(parent) = node.parent() else {
            log::info!(
                "Expanding to ancestor reached the top node, so using default context line count.",
            );
            return None;
        };
        node = parent;
    }
}

fn is_line_blank_or_indented_less(
    indent_level: u32,
    row: u32,
    tab_size: u32,
    snapshot: &BufferSnapshot,
) -> bool {
    let line_indent = snapshot.line_indent_for_row(row);
    line_indent.is_line_blank() || line_indent.len(tab_size) < indent_level
}
