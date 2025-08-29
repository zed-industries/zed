pub mod items;
mod toolbar_controls;

mod diagnostic_renderer;

#[cfg(test)]
mod diagnostics_tests;

use anyhow::Result;
use collections::{BTreeSet, HashMap};
use diagnostic_renderer::DiagnosticBlock;
use editor::{
    Editor, EditorEvent, ExcerptRange, MultiBuffer, PathKey,
    display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
    multibuffer_context_lines,
};
use gpui::{
    AnyElement, AnyView, App, AsyncApp, Context, Entity, EventEmitter, FocusHandle, Focusable,
    Global, InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Task, WeakEntity, Window, actions, div,
};
use language::{
    Bias, Buffer, BufferRow, BufferSnapshot, DiagnosticEntry, Point, ToTreeSitterPoint,
};
use project::{
    DiagnosticSummary, Project, ProjectPath,
    project_settings::{DiagnosticSeverity, ProjectSettings},
};
use settings::Settings;
use std::{
    any::{Any, TypeId},
    cmp::{self, Ordering},
    ops::{Range, RangeInclusive},
    sync::Arc,
    time::Duration,
};
use text::{BufferId, OffsetRangeExt};
use theme::ActiveTheme;
pub use toolbar_controls::ToolbarControls;
use ui::{Icon, IconName, Label, h_flex, prelude::*};
use util::ResultExt;
use workspace::{
    ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};

actions!(
    diagnostics,
    [
        /// Opens the project diagnostics view.
        Deploy,
        /// Toggles the display of warning-level diagnostics.
        ToggleWarnings,
        /// Toggles automatic refresh of diagnostics.
        ToggleDiagnosticsRefresh
    ]
);

#[derive(Default)]
pub(crate) struct IncludeWarnings(bool);
impl Global for IncludeWarnings {}

pub fn init(cx: &mut App) {
    editor::set_diagnostic_renderer(diagnostic_renderer::DiagnosticRenderer {}, cx);
    cx.observe_new(ProjectDiagnosticsEditor::register).detach();
}

pub(crate) struct ProjectDiagnosticsEditor {
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    editor: Entity<Editor>,
    diagnostics: HashMap<BufferId, Vec<DiagnosticEntry<text::Anchor>>>,
    blocks: HashMap<BufferId, Vec<CustomBlockId>>,
    summary: DiagnosticSummary,
    multibuffer: Entity<MultiBuffer>,
    paths_to_update: BTreeSet<ProjectPath>,
    include_warnings: bool,
    update_excerpts_task: Option<Task<Result<()>>>,
    diagnostic_summary_update: Task<()>,
    _subscription: Subscription,
}

impl EventEmitter<EditorEvent> for ProjectDiagnosticsEditor {}

const DIAGNOSTICS_UPDATE_DELAY: Duration = Duration::from_millis(50);

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
            .on_action(cx.listener(Self::toggle_diagnostics_refresh))
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

    fn new(
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
                    paths,
                } => {
                    this.paths_to_update.extend(paths.clone());
                    let project = project.clone();
                    this.diagnostic_summary_update = cx.spawn(async move |this, cx| {
                        cx.background_executor()
                            .timer(Duration::from_millis(30))
                            .await;
                        this.update(cx, |this, cx| {
                            this.summary = project.read(cx).diagnostic_summary(false, cx);
                        })
                        .log_err();
                    });
                    cx.emit(EditorEvent::TitleChanged);

                    if this.editor.focus_handle(cx).contains_focused(window, cx) || this.focus_handle.contains_focused(window, cx) {
                        log::debug!("diagnostics updated for server {language_server_id}, paths {paths:?}. recording change");
                    } else {
                        log::debug!("diagnostics updated for server {language_server_id}, paths {paths:?}. updating excerpts");
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
            editor.set_max_diagnostics_severity(
                if include_warnings {
                    DiagnosticSeverity::Warning
                } else {
                    DiagnosticSeverity::Error
                },
                cx,
            );
            editor.set_all_diagnostics_active(cx);
            editor
        });
        cx.subscribe_in(
            &editor,
            window,
            |this, _editor, event: &EditorEvent, window, cx| {
                cx.emit(event.clone());
                match event {
                    EditorEvent::Focused => {
                        if this.multibuffer.read(cx).is_empty() {
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
            let include_warnings = cx.global::<IncludeWarnings>().0;
            this.include_warnings = include_warnings;
            this.editor.update(cx, |editor, cx| {
                editor.set_max_diagnostics_severity(
                    if include_warnings {
                        DiagnosticSeverity::Warning
                    } else {
                        DiagnosticSeverity::Error
                    },
                    cx,
                )
            });
            this.diagnostics.clear();
            this.update_all_excerpts(window, cx);
        })
        .detach();

        let project = project_handle.read(cx);
        let mut this = Self {
            project: project_handle.clone(),
            summary: project.diagnostic_summary(false, cx),
            diagnostics: Default::default(),
            blocks: Default::default(),
            include_warnings,
            workspace,
            multibuffer: excerpts,
            focus_handle,
            editor,
            paths_to_update: Default::default(),
            update_excerpts_task: None,
            diagnostic_summary_update: Task::ready(()),
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
                .timer(DIAGNOSTICS_UPDATE_DELAY)
                .await;
            loop {
                let Some(path) = this.update(cx, |this, cx| {
                    let Some(path) = this.paths_to_update.pop_first() else {
                        this.update_excerpts_task = None;
                        cx.notify();
                        return None;
                    };
                    Some(path)
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
                        this.update_excerpts(buffer, window, cx)
                    })?
                    .await?;
                }
            }
            Ok(())
        }));
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
                    include_warnings,
                    workspace.project().clone(),
                    workspace_handle,
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(Box::new(diagnostics), None, true, window, cx);
        }
    }

    fn toggle_warnings(&mut self, _: &ToggleWarnings, _: &mut Window, cx: &mut Context<Self>) {
        cx.set_global(IncludeWarnings(!self.include_warnings));
    }

    fn toggle_diagnostics_refresh(
        &mut self,
        _: &ToggleDiagnosticsRefresh,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.update_excerpts_task.is_some() {
            self.update_excerpts_task = None;
        } else {
            self.update_all_excerpts(window, cx);
        }
        cx.notify();
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.focus_handle.is_focused(window) && !self.multibuffer.read(cx).is_empty() {
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
                .map(|(path, _, _)| path)
                .collect::<BTreeSet<_>>();
            self.multibuffer.update(cx, |multibuffer, cx| {
                for buffer in multibuffer.all_buffers() {
                    if let Some(file) = buffer.read(cx).file() {
                        paths.insert(ProjectPath {
                            path: file.path().clone(),
                            worktree_id: file.worktree_id(cx),
                        });
                    }
                }
            });
            self.paths_to_update = paths;
        });
        self.update_stale_excerpts(window, cx);
    }

    fn diagnostics_are_unchanged(
        &self,
        existing: &Vec<DiagnosticEntry<text::Anchor>>,
        new: &Vec<DiagnosticEntry<text::Anchor>>,
        snapshot: &BufferSnapshot,
    ) -> bool {
        if existing.len() != new.len() {
            return false;
        }
        existing.iter().zip(new.iter()).all(|(existing, new)| {
            existing.diagnostic.message == new.diagnostic.message
                && existing.diagnostic.severity == new.diagnostic.severity
                && existing.diagnostic.is_primary == new.diagnostic.is_primary
                && existing.range.to_offset(snapshot) == new.range.to_offset(snapshot)
        })
    }

    fn update_excerpts(
        &mut self,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let was_empty = self.multibuffer.read(cx).is_empty();
        let buffer_snapshot = buffer.read(cx).snapshot();
        let buffer_id = buffer_snapshot.remote_id();
        let max_severity = if self.include_warnings {
            lsp::DiagnosticSeverity::WARNING
        } else {
            lsp::DiagnosticSeverity::ERROR
        };

        cx.spawn_in(window, async move |this, cx| {
            let diagnostics = buffer_snapshot
                .diagnostics_in_range::<_, text::Anchor>(
                    Point::zero()..buffer_snapshot.max_point(),
                    false,
                )
                .collect::<Vec<_>>();
            let unchanged = this.update(cx, |this, _| {
                if this.diagnostics.get(&buffer_id).is_some_and(|existing| {
                    this.diagnostics_are_unchanged(existing, &diagnostics, &buffer_snapshot)
                }) {
                    return true;
                }
                this.diagnostics.insert(buffer_id, diagnostics.clone());
                false
            })?;
            if unchanged {
                return Ok(());
            }

            let mut grouped: HashMap<usize, Vec<_>> = HashMap::default();
            for entry in diagnostics {
                grouped
                    .entry(entry.diagnostic.group_id)
                    .or_default()
                    .push(DiagnosticEntry {
                        range: entry.range.to_point(&buffer_snapshot),
                        diagnostic: entry.diagnostic,
                    })
            }
            let mut blocks: Vec<DiagnosticBlock> = Vec::new();

            for (_, group) in grouped {
                let group_severity = group.iter().map(|d| d.diagnostic.severity).min();
                if group_severity.is_none_or(|s| s > max_severity) {
                    continue;
                }
                let more = cx.update(|_, cx| {
                    crate::diagnostic_renderer::DiagnosticRenderer::diagnostic_blocks_for_group(
                        group,
                        buffer_snapshot.remote_id(),
                        Some(this.clone()),
                        cx,
                    )
                })?;

                for item in more {
                    let i = blocks
                        .binary_search_by(|probe| {
                            probe
                                .initial_range
                                .start
                                .cmp(&item.initial_range.start)
                                .then(probe.initial_range.end.cmp(&item.initial_range.end))
                                .then(Ordering::Greater)
                        })
                        .unwrap_or_else(|i| i);
                    blocks.insert(i, item);
                }
            }

            let mut excerpt_ranges: Vec<ExcerptRange<Point>> = Vec::new();
            let context_lines = cx.update(|_, cx| multibuffer_context_lines(cx))?;
            for b in blocks.iter() {
                let excerpt_range = context_range_for_entry(
                    b.initial_range.clone(),
                    context_lines,
                    buffer_snapshot.clone(),
                    cx,
                )
                .await;
                let i = excerpt_ranges
                    .binary_search_by(|probe| {
                        probe
                            .context
                            .start
                            .cmp(&excerpt_range.start)
                            .then(probe.context.end.cmp(&excerpt_range.end))
                            .then(probe.primary.start.cmp(&b.initial_range.start))
                            .then(probe.primary.end.cmp(&b.initial_range.end))
                            .then(cmp::Ordering::Greater)
                    })
                    .unwrap_or_else(|i| i);
                excerpt_ranges.insert(
                    i,
                    ExcerptRange {
                        context: excerpt_range,
                        primary: b.initial_range.clone(),
                    },
                )
            }

            this.update_in(cx, |this, window, cx| {
                if let Some(block_ids) = this.blocks.remove(&buffer_id) {
                    this.editor.update(cx, |editor, cx| {
                        editor.display_map.update(cx, |display_map, cx| {
                            display_map.remove_blocks(block_ids.into_iter().collect(), cx)
                        });
                    })
                }
                let (anchor_ranges, _) = this.multibuffer.update(cx, |multi_buffer, cx| {
                    multi_buffer.set_excerpt_ranges_for_path(
                        PathKey::for_buffer(&buffer, cx),
                        buffer.clone(),
                        &buffer_snapshot,
                        excerpt_ranges,
                        cx,
                    )
                });
                #[cfg(test)]
                let cloned_blocks = blocks.clone();

                if was_empty && let Some(anchor_range) = anchor_ranges.first() {
                    let range_to_select = anchor_range.start..anchor_range.start;
                    this.editor.update(cx, |editor, cx| {
                        editor.change_selections(Default::default(), window, cx, |s| {
                            s.select_anchor_ranges([range_to_select]);
                        })
                    });
                    if this.focus_handle.is_focused(window) {
                        this.editor.read(cx).focus_handle(cx).focus(window);
                    }
                }

                let editor_blocks =
                    anchor_ranges
                        .into_iter()
                        .zip(blocks.into_iter())
                        .map(|(anchor, block)| {
                            let editor = this.editor.downgrade();
                            BlockProperties {
                                placement: BlockPlacement::Near(anchor.start),
                                height: Some(1),
                                style: BlockStyle::Flex,
                                render: Arc::new(move |bcx| {
                                    block.render_block(editor.clone(), bcx)
                                }),
                                priority: 1,
                            }
                        });
                let block_ids = this.editor.update(cx, |editor, cx| {
                    editor.display_map.update(cx, |display_map, cx| {
                        display_map.insert_blocks(editor_blocks, cx)
                    })
                });

                #[cfg(test)]
                {
                    for (block_id, block) in block_ids.iter().zip(cloned_blocks.iter()) {
                        let markdown = block.markdown.clone();
                        editor::test::set_block_content_for_tests(
                            &this.editor,
                            *block_id,
                            cx,
                            move |cx| {
                                markdown::MarkdownElement::rendered_text(
                                    markdown.clone(),
                                    cx,
                                    editor::hover_popover::diagnostics_markdown_style,
                                )
                            },
                        );
                    }
                }

                this.blocks.insert(buffer_id, block_ids);
                cx.notify()
            })
        })
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

    fn tab_content_text(&self, _detail: usize, _: &App) -> SharedString {
        "Diagnostics".into()
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
                self.include_warnings,
                self.project.clone(),
                self.workspace.clone(),
                window,
                cx,
            )
        }))
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).is_dirty(cx)
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).has_deleted_file(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).has_conflict(cx)
    }

    fn can_save(&self, _: &App) -> bool {
        true
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.save(options, project, window, cx)
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
            && let Some(end_row) = (outline_range.start.row..outline_range.end.row + 1)
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
