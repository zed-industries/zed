use super::*;

pub trait DiagnosticRenderer {
    fn render_group(
        &self,
        diagnostic_group: Vec<DiagnosticEntryRef<'_, Point>>,
        buffer_id: BufferId,
        snapshot: EditorSnapshot,
        editor: WeakEntity<Editor>,
        language_registry: Option<Arc<LanguageRegistry>>,
        cx: &mut App,
    ) -> Vec<BlockProperties<Anchor>>;

    fn render_hover(
        &self,
        diagnostic_group: Vec<DiagnosticEntryRef<'_, Point>>,
        range: Range<Point>,
        buffer_id: BufferId,
        language_registry: Option<Arc<LanguageRegistry>>,
        cx: &mut App,
    ) -> Option<Entity<markdown::Markdown>>;

    fn open_link(
        &self,
        editor: &mut Editor,
        link: SharedString,
        window: &mut Window,
        cx: &mut Context<Editor>,
    );
}

pub fn set_diagnostic_renderer(renderer: impl DiagnosticRenderer + 'static, cx: &mut App) {
    cx.set_global(GlobalDiagnosticRenderer(Arc::new(renderer)));
}

pub(super) struct GlobalDiagnosticRenderer(Arc<dyn DiagnosticRenderer>);

impl GlobalDiagnosticRenderer {
    pub(super) fn global(cx: &App) -> Option<Arc<dyn DiagnosticRenderer>> {
        cx.try_global::<Self>().map(|g| g.0.clone())
    }
}

impl gpui::Global for GlobalDiagnosticRenderer {}

#[derive(Debug, Clone)]
pub(super) struct InlineDiagnostic {
    pub(super) message: SharedString,
    pub(super) group_id: usize,
    pub(super) is_primary: bool,
    pub(super) start: Point,
    pub(super) severity: lsp::DiagnosticSeverity,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ActiveDiagnosticGroup {
    active_range: Range<Anchor>,
    active_message: String,
    group_id: usize,
    blocks: HashSet<CustomBlockId>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ActiveDiagnostic {
    None,
    All,
    Group(ActiveDiagnosticGroup),
}

impl Editor {
    pub fn go_to_diagnostic(
        &mut self,
        action: &GoToDiagnostic,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.diagnostics_enabled() {
            return;
        }
        self.go_to_diagnostic_impl(Direction::Next, action.severity, window, cx)
    }

    pub fn go_to_prev_diagnostic(
        &mut self,
        action: &GoToPreviousDiagnostic,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.diagnostics_enabled() {
            return;
        }
        self.go_to_diagnostic_impl(Direction::Prev, action.severity, window, cx)
    }

    pub fn go_to_diagnostic_impl(
        &mut self,
        direction: Direction,
        severity: GoToDiagnosticSeverityFilter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let selection = self
            .selections
            .newest::<MultiBufferOffset>(&self.display_snapshot(cx));

        let mut active_group_id = None;
        if let ActiveDiagnostic::Group(active_group) = &self.active_diagnostics
            && active_group.active_range.start.to_offset(&buffer) == selection.start
        {
            active_group_id = Some(active_group.group_id);
        }

        fn filtered<'a>(
            severity: GoToDiagnosticSeverityFilter,
            diagnostics: impl Iterator<Item = DiagnosticEntryRef<'a, MultiBufferOffset>>,
        ) -> impl Iterator<Item = DiagnosticEntryRef<'a, MultiBufferOffset>> {
            diagnostics
                .filter(move |entry| severity.matches(entry.diagnostic.severity))
                .filter(|entry| entry.range.start != entry.range.end)
                .filter(|entry| !entry.diagnostic.is_unnecessary)
        }

        let before = filtered(
            severity,
            buffer
                .diagnostics_in_range(MultiBufferOffset(0)..selection.start)
                .filter(|entry| entry.range.start <= selection.start),
        );
        let after = filtered(
            severity,
            buffer
                .diagnostics_in_range(selection.start..buffer.len())
                .filter(|entry| entry.range.start >= selection.start),
        );

        let mut found: Option<DiagnosticEntryRef<MultiBufferOffset>> = None;
        if direction == Direction::Prev {
            'outer: for prev_diagnostics in [before.collect::<Vec<_>>(), after.collect::<Vec<_>>()]
            {
                for diagnostic in prev_diagnostics.into_iter().rev() {
                    if diagnostic.range.start != selection.start
                        || active_group_id
                            .is_some_and(|active| diagnostic.diagnostic.group_id < active)
                    {
                        found = Some(diagnostic);
                        break 'outer;
                    }
                }
            }
        } else {
            for diagnostic in after.chain(before) {
                if diagnostic.range.start != selection.start
                    || active_group_id.is_some_and(|active| diagnostic.diagnostic.group_id > active)
                {
                    found = Some(diagnostic);
                    break;
                }
            }
        }
        let Some(next_diagnostic) = found else {
            return;
        };

        let next_diagnostic_start = buffer.anchor_after(next_diagnostic.range.start);
        let Some((buffer_anchor, _)) = buffer.anchor_to_buffer_anchor(next_diagnostic_start) else {
            return;
        };
        let buffer_id = buffer_anchor.buffer_id;
        let snapshot = self.snapshot(window, cx);
        if snapshot.intersects_fold(next_diagnostic.range.start) {
            self.unfold_ranges(
                std::slice::from_ref(&next_diagnostic.range),
                true,
                false,
                cx,
            );
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.select_ranges(vec![
                next_diagnostic.range.start..next_diagnostic.range.start,
            ])
        });
        self.activate_diagnostics(buffer_id, next_diagnostic, window, cx);
        self.refresh_edit_prediction(false, true, window, cx);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn active_diagnostic_message(&self) -> Option<&str> {
        match &self.active_diagnostics {
            ActiveDiagnostic::Group(group) => Some(group.active_message.as_str()),
            _ => None,
        }
    }

    pub fn set_all_diagnostics_active(&mut self, cx: &mut Context<Self>) {
        if !self.diagnostics_enabled() {
            return;
        }
        self.dismiss_diagnostics(cx);
        self.active_diagnostics = ActiveDiagnostic::All;
    }

    /// Disable inline diagnostics rendering for this editor.
    pub fn disable_inline_diagnostics(&mut self) {
        self.inline_diagnostics_enabled = false;
        self.inline_diagnostics_update = Task::ready(());
        self.inline_diagnostics.clear();
    }

    pub fn disable_diagnostics(&mut self, cx: &mut Context<Self>) {
        self.diagnostics_enabled = false;
        self.dismiss_diagnostics(cx);
        self.inline_diagnostics_update = Task::ready(());
        self.inline_diagnostics.clear();
    }

    pub fn diagnostics_enabled(&self) -> bool {
        self.diagnostics_enabled && self.lsp_data_enabled()
    }

    pub fn inline_diagnostics_enabled(&self) -> bool {
        self.inline_diagnostics_enabled && self.diagnostics_enabled()
    }

    pub fn show_inline_diagnostics(&self) -> bool {
        self.show_inline_diagnostics
    }

    pub fn toggle_inline_diagnostics(
        &mut self,
        _: &ToggleInlineDiagnostics,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        self.show_inline_diagnostics = !self.show_inline_diagnostics;
        self.refresh_inline_diagnostics(false, window, cx);
    }

    pub fn set_max_diagnostics_severity(&mut self, severity: DiagnosticSeverity, cx: &mut App) {
        self.diagnostics_max_severity = severity;
        self.display_map.update(cx, |display_map, _| {
            display_map.diagnostics_max_severity = self.diagnostics_max_severity;
        });
    }

    pub fn toggle_diagnostics(
        &mut self,
        _: &ToggleDiagnostics,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let diagnostics_enabled =
            self.diagnostics_enabled() && self.diagnostics_max_severity != DiagnosticSeverity::Off;
        self.diagnostics_enabled = !diagnostics_enabled;

        let new_severity = if self.diagnostics_enabled {
            EditorSettings::get_global(cx)
                .diagnostics_max_severity
                .filter(|severity| severity != &DiagnosticSeverity::Off)
                .unwrap_or(DiagnosticSeverity::Hint)
        } else {
            DiagnosticSeverity::Off
        };
        self.set_max_diagnostics_severity(new_severity, cx);
        if self.diagnostics_enabled {
            self.active_diagnostics = ActiveDiagnostic::None;
            self.inline_diagnostics_update = Task::ready(());
            self.inline_diagnostics.clear();
        } else {
            self.refresh_inline_diagnostics(false, window, cx);
        }

        cx.notify();
    }

    pub(super) fn all_diagnostics_active(&self) -> bool {
        self.active_diagnostics == ActiveDiagnostic::All
    }

    pub(super) fn active_diagnostic_group_id(&self) -> Option<usize> {
        match &self.active_diagnostics {
            ActiveDiagnostic::Group(group) => Some(group.group_id),
            _ => None,
        }
    }

    pub(super) fn has_active_diagnostic_group(&self) -> bool {
        matches!(self.active_diagnostics, ActiveDiagnostic::Group(_))
    }

    pub(super) fn refresh_active_diagnostics(&mut self, cx: &mut Context<Editor>) {
        if !self.diagnostics_enabled() {
            return;
        }

        if let ActiveDiagnostic::Group(active_diagnostics) = &mut self.active_diagnostics {
            let buffer = self.buffer.read(cx).snapshot(cx);
            let primary_range_start = active_diagnostics.active_range.start.to_offset(&buffer);
            let primary_range_end = active_diagnostics.active_range.end.to_offset(&buffer);
            let is_valid = buffer
                .diagnostics_in_range::<MultiBufferOffset>(primary_range_start..primary_range_end)
                .any(|entry| {
                    entry.diagnostic.is_primary
                        && !entry.range.is_empty()
                        && entry.range.start == primary_range_start
                        && entry.diagnostic.message == active_diagnostics.active_message
                });

            if !is_valid {
                self.dismiss_diagnostics(cx);
            }
        }
    }

    pub(super) fn activate_diagnostics(
        &mut self,
        buffer_id: BufferId,
        diagnostic: DiagnosticEntryRef<'_, MultiBufferOffset>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.diagnostics_enabled() || matches!(self.active_diagnostics, ActiveDiagnostic::All) {
            return;
        }
        self.dismiss_diagnostics(cx);
        let snapshot = self.snapshot(window, cx);
        let buffer = self.buffer.read(cx).snapshot(cx);
        let Some(renderer) = GlobalDiagnosticRenderer::global(cx) else {
            return;
        };

        let diagnostic_group = buffer
            .diagnostic_group(buffer_id, diagnostic.diagnostic.group_id)
            .collect::<Vec<_>>();

        let language_registry = self
            .project()
            .map(|project| project.read(cx).languages().clone());

        let blocks = renderer.render_group(
            diagnostic_group,
            buffer_id,
            snapshot,
            cx.weak_entity(),
            language_registry,
            cx,
        );

        let blocks = self.display_map.update(cx, |display_map, cx| {
            display_map.insert_blocks(blocks, cx).into_iter().collect()
        });
        self.active_diagnostics = ActiveDiagnostic::Group(ActiveDiagnosticGroup {
            active_range: buffer.anchor_before(diagnostic.range.start)
                ..buffer.anchor_after(diagnostic.range.end),
            active_message: diagnostic.diagnostic.message.clone(),
            group_id: diagnostic.diagnostic.group_id,
            blocks,
        });
        cx.notify();
    }

    pub(super) fn dismiss_diagnostics(&mut self, cx: &mut Context<Self>) {
        if matches!(self.active_diagnostics, ActiveDiagnostic::All) {
            return;
        };

        let prev = mem::replace(&mut self.active_diagnostics, ActiveDiagnostic::None);
        if let ActiveDiagnostic::Group(group) = prev {
            self.display_map.update(cx, |display_map, cx| {
                display_map.remove_blocks(group.blocks, cx);
            });
            cx.notify();
        }
    }

    pub(super) fn refresh_inline_diagnostics(
        &mut self,
        debounce: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let max_severity = ProjectSettings::get_global(cx)
            .diagnostics
            .inline
            .max_severity
            .unwrap_or(self.diagnostics_max_severity);

        if !self.inline_diagnostics_enabled()
            || !self.diagnostics_enabled()
            || !self.show_inline_diagnostics
            || max_severity == DiagnosticSeverity::Off
        {
            self.inline_diagnostics_update = Task::ready(());
            self.inline_diagnostics.clear();
            return;
        }

        let debounce_ms = ProjectSettings::get_global(cx)
            .diagnostics
            .inline
            .update_debounce_ms;
        let debounce = if debounce && debounce_ms > 0 {
            Some(Duration::from_millis(debounce_ms))
        } else {
            None
        };
        self.inline_diagnostics_update = cx.spawn_in(window, async move |editor, cx| {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }
            let Some(snapshot) = editor.upgrade().map(|editor| {
                editor.update(cx, |editor, cx| editor.buffer().read(cx).snapshot(cx))
            }) else {
                return;
            };

            let new_inline_diagnostics = cx
                .background_spawn(async move {
                    let mut inline_diagnostics = Vec::<(Anchor, InlineDiagnostic)>::new();
                    for diagnostic_entry in
                        snapshot.diagnostics_in_range(MultiBufferOffset(0)..snapshot.len())
                    {
                        let message = diagnostic_entry
                            .diagnostic
                            .message
                            .split_once('\n')
                            .map(|(line, _)| line)
                            .map(SharedString::new)
                            .unwrap_or_else(|| {
                                SharedString::new(&*diagnostic_entry.diagnostic.message)
                            });
                        let start_anchor = snapshot.anchor_before(diagnostic_entry.range.start);
                        let (Ok(i) | Err(i)) = inline_diagnostics
                            .binary_search_by(|(probe, _)| probe.cmp(&start_anchor, &snapshot));
                        inline_diagnostics.insert(
                            i,
                            (
                                start_anchor,
                                InlineDiagnostic {
                                    message,
                                    group_id: diagnostic_entry.diagnostic.group_id,
                                    start: diagnostic_entry.range.start.to_point(&snapshot),
                                    is_primary: diagnostic_entry.diagnostic.is_primary,
                                    severity: diagnostic_entry.diagnostic.severity,
                                },
                            ),
                        );
                    }
                    inline_diagnostics
                })
                .await;

            editor
                .update(cx, |editor, cx| {
                    editor.inline_diagnostics = new_inline_diagnostics;
                    cx.notify();
                })
                .ok();
        });
    }

    pub(super) fn pull_diagnostics(
        &mut self,
        buffer_id: BufferId,
        _window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        // `ActiveDiagnostic::All` is a special mode where editor's diagnostics are managed by the external view,
        // skip any LSP updates for it.

        if self.active_diagnostics == ActiveDiagnostic::All || !self.diagnostics_enabled() {
            return None;
        }
        let pull_diagnostics_settings = ProjectSettings::get_global(cx)
            .diagnostics
            .lsp_pull_diagnostics;
        if !pull_diagnostics_settings.enabled {
            return None;
        }
        let debounce = Duration::from_millis(pull_diagnostics_settings.debounce_ms);
        let project = self.project()?.downgrade();
        let buffer = self.buffer().read(cx).buffer(buffer_id)?;

        self.pull_diagnostics_task = cx.spawn(async move |_, cx| {
            cx.background_executor().timer(debounce).await;
            if let Ok(task) = project.update(cx, |project, cx| {
                project.lsp_store().update(cx, |lsp_store, cx| {
                    lsp_store.pull_diagnostics_for_buffer(buffer, cx)
                })
            }) {
                task.await.log_err();
            }
            project
                .update(cx, |project, cx| {
                    project.lsp_store().update(cx, |lsp_store, cx| {
                        lsp_store.pull_document_diagnostics_for_buffer_edit(buffer_id, cx);
                    })
                })
                .log_err();
        });

        Some(())
    }

    pub(super) fn update_diagnostics_state(
        &mut self,
        window: &mut Window,
        cx: &mut Context<'_, Editor>,
    ) {
        if !self.diagnostics_enabled() {
            return;
        }
        self.refresh_active_diagnostics(cx);
        self.refresh_inline_diagnostics(true, window, cx);
        self.scrollbar_marker_state.dirty = true;
        cx.notify();
    }
}
