use crate::{
    DIAGNOSTICS_UPDATE_DEBOUNCE, IncludeWarnings, ToggleWarnings, context_range_for_entry,
    diagnostic_renderer::{DiagnosticBlock, DiagnosticRenderer},
    toolbar_controls::DiagnosticsToolbarEditor,
};
use anyhow::Result;
use collections::HashMap;
use editor::{
    Editor, EditorEvent, ExcerptRange, MultiBuffer, PathKey,
    display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
    multibuffer_context_lines,
};
use gpui::{
    AnyElement, App, AppContext, Context, Entity, EntityId, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled, Subscription,
    Task, WeakEntity, Window, actions, div,
};
use language::{Buffer, DiagnosticEntry, DiagnosticEntryRef, Point};
use project::{
    DiagnosticSummary, Event, Project, ProjectItem, ProjectPath,
    project_settings::{DiagnosticSeverity, ProjectSettings},
};
use settings::Settings;
use std::{
    any::{Any, TypeId},
    cmp::{self, Ordering},
    sync::Arc,
};
use text::{Anchor, BufferSnapshot, OffsetRangeExt};
use ui::{Button, ButtonStyle, Icon, IconName, Label, Tooltip, h_flex, prelude::*};
use workspace::{
    ItemHandle, ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, Item, ItemEvent, TabContentParams},
};

actions!(
    diagnostics,
    [
        /// Opens the project diagnostics view for the currently focused file.
        DeployCurrentFile,
    ]
);

/// The `BufferDiagnosticsEditor` is meant to be used when dealing specifically
/// with diagnostics for a single buffer, as only the excerpts of the buffer
/// where diagnostics are available are displayed.
pub(crate) struct BufferDiagnosticsEditor {
    pub project: Entity<Project>,
    focus_handle: FocusHandle,
    editor: Entity<Editor>,
    /// The current diagnostic entries in the `BufferDiagnosticsEditor`. Used to
    /// allow quick comparison of updated diagnostics, to confirm if anything
    /// has changed.
    pub(crate) diagnostics: Vec<DiagnosticEntry<Anchor>>,
    /// The blocks used to display the diagnostics' content in the editor, next
    /// to the excerpts where the diagnostic originated.
    blocks: Vec<CustomBlockId>,
    /// Multibuffer to contain all excerpts that contain diagnostics, which are
    /// to be rendered in the editor.
    multibuffer: Entity<MultiBuffer>,
    /// The buffer for which the editor is displaying diagnostics and excerpts
    /// for.
    buffer: Option<Entity<Buffer>>,
    /// The path for which the editor is displaying diagnostics for.
    project_path: ProjectPath,
    /// Summary of the number of warnings and errors for the path. Used to
    /// display the number of warnings and errors in the tab's content.
    summary: DiagnosticSummary,
    /// Whether to include warnings in the list of diagnostics shown in the
    /// editor.
    pub(crate) include_warnings: bool,
    /// Keeps track of whether there's a background task already running to
    /// update the excerpts, in order to avoid firing multiple tasks for this purpose.
    pub(crate) update_excerpts_task: Option<Task<Result<()>>>,
    /// The project's subscription, responsible for processing events related to
    /// diagnostics.
    _subscription: Subscription,
}

impl BufferDiagnosticsEditor {
    /// Creates new instance of the `BufferDiagnosticsEditor` which can then be
    /// displayed by adding it to a pane.
    pub fn new(
        project_path: ProjectPath,
        project_handle: Entity<Project>,
        buffer: Option<Entity<Buffer>>,
        include_warnings: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Subscribe to project events related to diagnostics so the
        // `BufferDiagnosticsEditor` can update its state accordingly.
        let project_event_subscription = cx.subscribe_in(
            &project_handle,
            window,
            |buffer_diagnostics_editor, _project, event, window, cx| match event {
                Event::DiskBasedDiagnosticsStarted { .. } => {
                    cx.notify();
                }
                Event::DiskBasedDiagnosticsFinished { .. } => {
                    buffer_diagnostics_editor.update_all_excerpts(window, cx);
                }
                Event::DiagnosticsUpdated {
                    paths,
                    language_server_id,
                } => {
                    // When diagnostics have been updated, the
                    // `BufferDiagnosticsEditor` should update its state only if
                    // one of the paths matches its `project_path`, otherwise
                    // the event should be ignored.
                    if paths.contains(&buffer_diagnostics_editor.project_path) {
                        buffer_diagnostics_editor.update_diagnostic_summary(cx);

                        if buffer_diagnostics_editor.editor.focus_handle(cx).contains_focused(window, cx) || buffer_diagnostics_editor.focus_handle.contains_focused(window, cx) {
                            log::debug!("diagnostics updated for server {language_server_id}. recording change");
                        } else {
                            log::debug!("diagnostics updated for server {language_server_id}. updating excerpts");
                            buffer_diagnostics_editor.update_all_excerpts(window, cx);
                        }
                    }
                }
                _ => {}
            },
        );

        let focus_handle = cx.focus_handle();

        cx.on_focus_in(
            &focus_handle,
            window,
            |buffer_diagnostics_editor, window, cx| buffer_diagnostics_editor.focus_in(window, cx),
        )
        .detach();

        cx.on_focus_out(
            &focus_handle,
            window,
            |buffer_diagnostics_editor, _event, window, cx| {
                buffer_diagnostics_editor.focus_out(window, cx)
            },
        )
        .detach();

        let summary = project_handle
            .read(cx)
            .diagnostic_summary_for_path(&project_path, cx);

        let multibuffer = cx.new(|cx| MultiBuffer::new(project_handle.read(cx).capability()));
        let max_severity = Self::max_diagnostics_severity(include_warnings);
        let editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(
                multibuffer.clone(),
                Some(project_handle.clone()),
                window,
                cx,
            );
            editor.set_vertical_scroll_margin(5, cx);
            editor.disable_inline_diagnostics();
            editor.set_max_diagnostics_severity(max_severity, cx);
            editor.set_all_diagnostics_active(cx);
            editor
        });

        // Subscribe to events triggered by the editor in order to correctly
        // update the buffer's excerpts.
        cx.subscribe_in(
            &editor,
            window,
            |buffer_diagnostics_editor, _editor, event: &EditorEvent, window, cx| {
                cx.emit(event.clone());

                match event {
                    // If the user tries to focus on the editor but there's actually
                    // no excerpts for the buffer, focus back on the
                    // `BufferDiagnosticsEditor` instance.
                    EditorEvent::Focused => {
                        if buffer_diagnostics_editor.multibuffer.read(cx).is_empty() {
                            window.focus(&buffer_diagnostics_editor.focus_handle);
                        }
                    }
                    EditorEvent::Blurred => {
                        buffer_diagnostics_editor.update_all_excerpts(window, cx)
                    }
                    _ => {}
                }
            },
        )
        .detach();

        let diagnostics = vec![];
        let update_excerpts_task = None;
        let mut buffer_diagnostics_editor = Self {
            project: project_handle,
            focus_handle,
            editor,
            diagnostics,
            blocks: Default::default(),
            multibuffer,
            buffer,
            project_path,
            summary,
            include_warnings,
            update_excerpts_task,
            _subscription: project_event_subscription,
        };

        buffer_diagnostics_editor.update_all_diagnostics(window, cx);
        buffer_diagnostics_editor
    }

    fn deploy(
        workspace: &mut Workspace,
        _: &DeployCurrentFile,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        // Determine the currently opened path by finding the active editor and
        // finding the project path for the buffer.
        // If there's no active editor with a project path, avoiding deploying
        // the buffer diagnostics view.
        if let Some(editor) = workspace.active_item_as::<Editor>(cx)
            && let Some(project_path) = editor.project_path(cx)
        {
            // Check if there's already a `BufferDiagnosticsEditor` tab for this
            // same path, and if so, focus on that one instead of creating a new
            // one.
            let existing_editor = workspace
                .items_of_type::<BufferDiagnosticsEditor>(cx)
                .find(|editor| editor.read(cx).project_path == project_path);

            if let Some(editor) = existing_editor {
                workspace.activate_item(&editor, true, true, window, cx);
            } else {
                let include_warnings = match cx.try_global::<IncludeWarnings>() {
                    Some(include_warnings) => include_warnings.0,
                    None => ProjectSettings::get_global(cx).diagnostics.include_warnings,
                };

                let item = cx.new(|cx| {
                    Self::new(
                        project_path,
                        workspace.project().clone(),
                        editor.read(cx).buffer().read(cx).as_singleton(),
                        include_warnings,
                        window,
                        cx,
                    )
                });

                workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
            }
        }
    }

    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(Self::deploy);
    }

    fn update_all_diagnostics(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.update_all_excerpts(window, cx);
    }

    fn update_diagnostic_summary(&mut self, cx: &mut Context<Self>) {
        let project = self.project.read(cx);

        self.summary = project.diagnostic_summary_for_path(&self.project_path, cx);
    }

    /// Enqueue an update to the excerpts and diagnostic blocks being shown in
    /// the editor.
    pub(crate) fn update_all_excerpts(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // If there's already a task updating the excerpts, early return and let
        // the other task finish.
        if self.update_excerpts_task.is_some() {
            return;
        }

        let buffer = self.buffer.clone();

        self.update_excerpts_task = Some(cx.spawn_in(window, async move |editor, cx| {
            cx.background_executor()
                .timer(DIAGNOSTICS_UPDATE_DEBOUNCE)
                .await;

            if let Some(buffer) = buffer {
                editor
                    .update_in(cx, |editor, window, cx| {
                        editor.update_excerpts(buffer, window, cx)
                    })?
                    .await?;
            };

            let _ = editor.update(cx, |editor, cx| {
                editor.update_excerpts_task = None;
                cx.notify();
            });

            Ok(())
        }));
    }

    /// Updates the excerpts in the `BufferDiagnosticsEditor` for a single
    /// buffer.
    fn update_excerpts(
        &mut self,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let was_empty = self.multibuffer.read(cx).is_empty();
        let multibuffer_context = multibuffer_context_lines(cx);
        let buffer_snapshot = buffer.read(cx).snapshot();
        let buffer_snapshot_max = buffer_snapshot.max_point();
        let max_severity = Self::max_diagnostics_severity(self.include_warnings)
            .into_lsp()
            .unwrap_or(lsp::DiagnosticSeverity::WARNING);

        cx.spawn_in(window, async move |buffer_diagnostics_editor, mut cx| {
            // Fetch the diagnostics for the whole of the buffer
            // (`Point::zero()..buffer_snapshot.max_point()`) so we can confirm
            // if the diagnostics changed, if it didn't, early return as there's
            // nothing to update.
            let diagnostics = buffer_snapshot
                .diagnostics_in_range::<_, Anchor>(Point::zero()..buffer_snapshot_max, false)
                .collect::<Vec<_>>();

            let unchanged =
                buffer_diagnostics_editor.update(cx, |buffer_diagnostics_editor, _cx| {
                    if buffer_diagnostics_editor
                        .diagnostics_are_unchanged(&diagnostics, &buffer_snapshot)
                    {
                        return true;
                    }

                    buffer_diagnostics_editor.set_diagnostics(&diagnostics);
                    return false;
                })?;

            if unchanged {
                return Ok(());
            }

            // Mapping between the Group ID and a vector of DiagnosticEntry.
            let mut grouped: HashMap<usize, Vec<_>> = HashMap::default();
            for entry in diagnostics {
                grouped
                    .entry(entry.diagnostic.group_id)
                    .or_default()
                    .push(DiagnosticEntryRef {
                        range: entry.range.to_point(&buffer_snapshot),
                        diagnostic: entry.diagnostic,
                    })
            }

            let mut blocks: Vec<DiagnosticBlock> = Vec::new();
            for (_, group) in grouped {
                // If the minimum severity of the group is higher than the
                // maximum severity, or it doesn't even have severity, skip this
                // group.
                if group
                    .iter()
                    .map(|d| d.diagnostic.severity)
                    .min()
                    .is_none_or(|severity| severity > max_severity)
                {
                    continue;
                }

                let languages = buffer_diagnostics_editor
                    .read_with(cx, |b, cx| b.project.read(cx).languages().clone())
                    .ok();

                let diagnostic_blocks = cx.update(|_window, cx| {
                    DiagnosticRenderer::diagnostic_blocks_for_group(
                        group,
                        buffer_snapshot.remote_id(),
                        Some(Arc::new(buffer_diagnostics_editor.clone())),
                        languages,
                        cx,
                    )
                })?;

                // For each of the diagnostic blocks to be displayed in the
                // editor, figure out its index in the list of blocks.
                //
                // The following rules are used to determine the order:
                // 1. Blocks with a lower start position should come first.
                // 2. If two blocks have the same start position, the one with
                // the higher end position should come first.
                for diagnostic_block in diagnostic_blocks {
                    let index = blocks.partition_point(|probe| {
                        match probe
                            .initial_range
                            .start
                            .cmp(&diagnostic_block.initial_range.start)
                        {
                            Ordering::Less => true,
                            Ordering::Greater => false,
                            Ordering::Equal => {
                                probe.initial_range.end > diagnostic_block.initial_range.end
                            }
                        }
                    });

                    blocks.insert(index, diagnostic_block);
                }
            }

            // Build the excerpt ranges for this specific buffer's diagnostics,
            // so those excerpts can later be used to update the excerpts shown
            // in the editor.
            // This is done by iterating over the list of diagnostic blocks and
            // determine what range does the diagnostic block span.
            let mut excerpt_ranges: Vec<ExcerptRange<_>> = Vec::new();

            for diagnostic_block in blocks.iter() {
                let excerpt_range = context_range_for_entry(
                    diagnostic_block.initial_range.clone(),
                    multibuffer_context,
                    buffer_snapshot.clone(),
                    &mut cx,
                )
                .await;
                let initial_range = buffer_snapshot
                    .anchor_after(diagnostic_block.initial_range.start)
                    ..buffer_snapshot.anchor_before(diagnostic_block.initial_range.end);

                let bin_search = |probe: &ExcerptRange<text::Anchor>| {
                    let context_start = || {
                        probe
                            .context
                            .start
                            .cmp(&excerpt_range.start, &buffer_snapshot)
                    };
                    let context_end =
                        || probe.context.end.cmp(&excerpt_range.end, &buffer_snapshot);
                    let primary_start = || {
                        probe
                            .primary
                            .start
                            .cmp(&initial_range.start, &buffer_snapshot)
                    };
                    let primary_end =
                        || probe.primary.end.cmp(&initial_range.end, &buffer_snapshot);
                    context_start()
                        .then_with(context_end)
                        .then_with(primary_start)
                        .then_with(primary_end)
                        .then(cmp::Ordering::Greater)
                };

                let index = excerpt_ranges
                    .binary_search_by(bin_search)
                    .unwrap_or_else(|i| i);

                excerpt_ranges.insert(
                    index,
                    ExcerptRange {
                        context: excerpt_range,
                        primary: initial_range,
                    },
                )
            }

            // Finally, update the editor's content with the new excerpt ranges
            // for this editor, as well as the diagnostic blocks.
            buffer_diagnostics_editor.update_in(cx, |buffer_diagnostics_editor, window, cx| {
                // Remove the list of `CustomBlockId` from the editor's display
                // map, ensuring that if any diagnostics have been solved, the
                // associated block stops being shown.
                let block_ids = buffer_diagnostics_editor.blocks.clone();

                buffer_diagnostics_editor.editor.update(cx, |editor, cx| {
                    editor.display_map.update(cx, |display_map, cx| {
                        display_map.remove_blocks(block_ids.into_iter().collect(), cx);
                    })
                });

                let (anchor_ranges, _) =
                    buffer_diagnostics_editor
                        .multibuffer
                        .update(cx, |multibuffer, cx| {
                            let excerpt_ranges = excerpt_ranges
                                .into_iter()
                                .map(|range| ExcerptRange {
                                    context: range.context.to_point(&buffer_snapshot),
                                    primary: range.primary.to_point(&buffer_snapshot),
                                })
                                .collect();
                            multibuffer.set_excerpt_ranges_for_path(
                                PathKey::for_buffer(&buffer, cx),
                                buffer.clone(),
                                &buffer_snapshot,
                                excerpt_ranges,
                                cx,
                            )
                        });

                if was_empty {
                    if let Some(anchor_range) = anchor_ranges.first() {
                        let range_to_select = anchor_range.start..anchor_range.start;

                        buffer_diagnostics_editor.editor.update(cx, |editor, cx| {
                            editor.change_selections(Default::default(), window, cx, |selection| {
                                selection.select_anchor_ranges([range_to_select])
                            })
                        });

                        // If the `BufferDiagnosticsEditor` is currently
                        // focused, move focus to its editor.
                        if buffer_diagnostics_editor.focus_handle.is_focused(window) {
                            buffer_diagnostics_editor
                                .editor
                                .read(cx)
                                .focus_handle(cx)
                                .focus(window);
                        }
                    }
                }

                // Cloning the blocks before moving ownership so these can later
                // be used to set the block contents for testing purposes.
                #[cfg(test)]
                let cloned_blocks = blocks.clone();

                // Build new diagnostic blocks to be added to the editor's
                // display map for the new diagnostics. Update the `blocks`
                // property before finishing, to ensure the blocks are removed
                // on the next execution.
                let editor_blocks =
                    anchor_ranges
                        .into_iter()
                        .zip(blocks.into_iter())
                        .map(|(anchor, block)| {
                            let editor = buffer_diagnostics_editor.editor.downgrade();

                            BlockProperties {
                                placement: BlockPlacement::Near(anchor.start),
                                height: Some(1),
                                style: BlockStyle::Flex,
                                render: Arc::new(move |block_context| {
                                    block.render_block(editor.clone(), block_context)
                                }),
                                priority: 1,
                            }
                        });

                let block_ids = buffer_diagnostics_editor.editor.update(cx, |editor, cx| {
                    editor.display_map.update(cx, |display_map, cx| {
                        display_map.insert_blocks(editor_blocks, cx)
                    })
                });

                // In order to be able to verify which diagnostic blocks are
                // rendered in the editor, the `set_block_content_for_tests`
                // function must be used, so that the
                // `editor::test::editor_content_with_blocks` function can then
                // be called to fetch these blocks.
                #[cfg(test)]
                {
                    for (block_id, block) in block_ids.iter().zip(cloned_blocks.iter()) {
                        let markdown = block.markdown.clone();
                        editor::test::set_block_content_for_tests(
                            &buffer_diagnostics_editor.editor,
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

                buffer_diagnostics_editor.blocks = block_ids;
                cx.notify()
            })
        })
    }

    fn set_diagnostics(&mut self, diagnostics: &[DiagnosticEntryRef<'_, Anchor>]) {
        self.diagnostics = diagnostics
            .iter()
            .map(DiagnosticEntryRef::to_owned)
            .collect();
    }

    fn diagnostics_are_unchanged(
        &self,
        diagnostics: &Vec<DiagnosticEntryRef<'_, Anchor>>,
        snapshot: &BufferSnapshot,
    ) -> bool {
        if self.diagnostics.len() != diagnostics.len() {
            return false;
        }

        self.diagnostics
            .iter()
            .zip(diagnostics.iter())
            .all(|(existing, new)| {
                existing.diagnostic.message == new.diagnostic.message
                    && existing.diagnostic.severity == new.diagnostic.severity
                    && existing.diagnostic.is_primary == new.diagnostic.is_primary
                    && existing.range.to_offset(snapshot) == new.range.to_offset(snapshot)
            })
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // If the `BufferDiagnosticsEditor` is focused and the multibuffer is
        // not empty, focus on the editor instead, which will allow the user to
        // start interacting and editing the buffer's contents.
        if self.focus_handle.is_focused(window) && !self.multibuffer.read(cx).is_empty() {
            self.editor.focus_handle(cx).focus(window)
        }
    }

    fn focus_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.is_focused(window) && !self.editor.focus_handle(cx).is_focused(window)
        {
            self.update_all_excerpts(window, cx);
        }
    }

    pub fn toggle_warnings(
        &mut self,
        _: &ToggleWarnings,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let include_warnings = !self.include_warnings;
        let max_severity = Self::max_diagnostics_severity(include_warnings);

        self.editor.update(cx, |editor, cx| {
            editor.set_max_diagnostics_severity(max_severity, cx);
        });

        self.include_warnings = include_warnings;
        self.diagnostics.clear();
        self.update_all_diagnostics(window, cx);
    }

    fn max_diagnostics_severity(include_warnings: bool) -> DiagnosticSeverity {
        match include_warnings {
            true => DiagnosticSeverity::Warning,
            false => DiagnosticSeverity::Error,
        }
    }

    #[cfg(test)]
    pub fn editor(&self) -> &Entity<Editor> {
        &self.editor
    }

    #[cfg(test)]
    pub fn summary(&self) -> &DiagnosticSummary {
        &self.summary
    }
}

impl Focusable for BufferDiagnosticsEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<EditorEvent> for BufferDiagnosticsEditor {}

impl Item for BufferDiagnosticsEditor {
    type Event = EditorEvent;

    fn act_as_type<'a>(
        &'a self,
        type_id: std::any::TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.clone().into())
        } else {
            None
        }
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

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
    }

    fn can_save(&self, _cx: &App) -> bool {
        true
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
            BufferDiagnosticsEditor::new(
                self.project_path.clone(),
                self.project.clone(),
                self.buffer.clone(),
                self.include_warnings,
                window,
                cx,
            )
        })))
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn for_each_project_item(&self, cx: &App, f: &mut dyn FnMut(EntityId, &dyn ProjectItem)) {
        self.editor.for_each_project_item(cx, f);
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).has_conflict(cx)
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).has_deleted_file(cx)
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).is_dirty(cx)
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

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.reload(project, window, cx)
    }

    fn save(
        &mut self,
        options: workspace::item::SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.save(options, project, window, cx)
    }

    fn save_as(
        &mut self,
        _project: Entity<Project>,
        _path: ProjectPath,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unreachable!()
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        })
    }

    // Builds the content to be displayed in the tab.
    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        let path_style = self.project.read(cx).path_style(cx);
        let error_count = self.summary.error_count;
        let warning_count = self.summary.warning_count;
        let label = Label::new(
            self.project_path
                .path
                .file_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| self.project_path.path.display(path_style).to_string()),
        );

        h_flex()
            .gap_1()
            .child(label)
            .when(error_count == 0 && warning_count == 0, |parent| {
                parent.child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success)),
                )
            })
            .when(error_count > 0, |parent| {
                parent.child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::XCircle).color(Color::Error))
                        .child(Label::new(error_count.to_string()).color(params.text_color())),
                )
            })
            .when(warning_count > 0, |parent| {
                parent.child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Warning).color(Color::Warning))
                        .child(Label::new(warning_count.to_string()).color(params.text_color())),
                )
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, _app: &App) -> SharedString {
        "Buffer Diagnostics".into()
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let path_style = self.project.read(cx).path_style(cx);
        Some(
            format!(
                "Buffer Diagnostics - {}",
                self.project_path.path.display(path_style)
            )
            .into(),
        )
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Buffer Diagnostics Opened")
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }
}

impl Render for BufferDiagnosticsEditor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let path_style = self.project.read(cx).path_style(cx);
        let filename = self.project_path.path.display(path_style).to_string();
        let error_count = self.summary.error_count;
        let warning_count = match self.include_warnings {
            true => self.summary.warning_count,
            false => 0,
        };

        let child = if error_count + warning_count == 0 {
            let label = match warning_count {
                0 => "No problems in",
                _ => "No errors in",
            };

            v_flex()
                .key_context("EmptyPane")
                .size_full()
                .gap_1()
                .justify_center()
                .items_center()
                .text_center()
                .bg(cx.theme().colors().editor_background)
                .child(
                    div()
                        .h_flex()
                        .child(Label::new(label).color(Color::Muted))
                        .child(
                            Button::new("open-file", filename)
                                .style(ButtonStyle::Transparent)
                                .tooltip(Tooltip::text("Open File"))
                                .on_click(cx.listener(|buffer_diagnostics, _, window, cx| {
                                    if let Some(workspace) = window.root::<Workspace>().flatten() {
                                        workspace.update(cx, |workspace, cx| {
                                            workspace
                                                .open_path(
                                                    buffer_diagnostics.project_path.clone(),
                                                    None,
                                                    true,
                                                    window,
                                                    cx,
                                                )
                                                .detach_and_log_err(cx);
                                        })
                                    }
                                })),
                        ),
                )
                .when(self.summary.warning_count > 0, |div| {
                    let label = match self.summary.warning_count {
                        1 => "Show 1 warning".into(),
                        warning_count => format!("Show {} warnings", warning_count),
                    };

                    div.child(
                        Button::new("diagnostics-show-warning-label", label).on_click(cx.listener(
                            |buffer_diagnostics_editor, _, window, cx| {
                                buffer_diagnostics_editor.toggle_warnings(
                                    &Default::default(),
                                    window,
                                    cx,
                                );
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
            .child(child)
    }
}

impl DiagnosticsToolbarEditor for WeakEntity<BufferDiagnosticsEditor> {
    fn include_warnings(&self, cx: &App) -> bool {
        self.read_with(cx, |buffer_diagnostics_editor, _cx| {
            buffer_diagnostics_editor.include_warnings
        })
        .unwrap_or(false)
    }

    fn is_updating(&self, cx: &App) -> bool {
        self.read_with(cx, |buffer_diagnostics_editor, cx| {
            buffer_diagnostics_editor.update_excerpts_task.is_some()
                || buffer_diagnostics_editor
                    .project
                    .read(cx)
                    .language_servers_running_disk_based_diagnostics(cx)
                    .next()
                    .is_some()
        })
        .unwrap_or(false)
    }

    fn stop_updating(&self, cx: &mut App) {
        let _ = self.update(cx, |buffer_diagnostics_editor, cx| {
            buffer_diagnostics_editor.update_excerpts_task = None;
            cx.notify();
        });
    }

    fn refresh_diagnostics(&self, window: &mut Window, cx: &mut App) {
        let _ = self.update(cx, |buffer_diagnostics_editor, cx| {
            buffer_diagnostics_editor.update_all_excerpts(window, cx);
        });
    }

    fn toggle_warnings(&self, window: &mut Window, cx: &mut App) {
        let _ = self.update(cx, |buffer_diagnostics_editor, cx| {
            buffer_diagnostics_editor.toggle_warnings(&Default::default(), window, cx);
        });
    }

    fn get_diagnostics_for_buffer(
        &self,
        _buffer_id: text::BufferId,
        cx: &App,
    ) -> Vec<language::DiagnosticEntry<text::Anchor>> {
        self.read_with(cx, |buffer_diagnostics_editor, _cx| {
            buffer_diagnostics_editor.diagnostics.clone()
        })
        .unwrap_or_default()
    }
}
