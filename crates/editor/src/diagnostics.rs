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

#[derive(Default)]
pub(super) struct InlineDiagnostics {
    entries: Vec<(Anchor, InlineDiagnostic)>,
    best: RangeMinimum,
    best_primary: RangeMinimum,
    groups: HashMap<usize, Vec<usize>>,
}

impl InlineDiagnostics {
    pub(super) fn new(
        mut entries: Vec<(Anchor, InlineDiagnostic)>,
        buffer: &MultiBufferSnapshot,
    ) -> Self {
        entries.sort_by(|(left, _), (right, _)| left.cmp(right, buffer));
        let precedes = |left: usize, right: usize| precedes(&entries, left, right);
        let best = RangeMinimum::new(entries.len(), |_| true, &precedes);
        let best_primary = RangeMinimum::new(
            entries.len(),
            |index| entries[index].1.is_primary,
            &precedes,
        );
        let mut groups = HashMap::<usize, Vec<usize>>::default();
        for (index, (_, diagnostic)) in entries.iter().enumerate() {
            groups.entry(diagnostic.group_id).or_default().push(index);
        }
        Self {
            entries,
            best,
            best_primary,
            groups,
        }
    }

    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(super) fn clear(&mut self) {
        *self = Self::default();
    }

    pub(super) fn per_row(
        &self,
        snapshot: &DisplaySnapshot,
        rows: Range<DisplayRow>,
        max_severity: lsp::DiagnosticSeverity,
        excluded_group: Option<usize>,
    ) -> Vec<(DisplayRow, &InlineDiagnostic)> {
        let len = self.entries.len();
        let row = |index: usize| self.entries[index].0.to_display_point(snapshot).row();
        let mut start = self
            .entries
            .partition_point(|(anchor, _)| anchor.to_display_point(snapshot).row() < rows.start);
        let mut diagnostics = Vec::new();
        while start < len {
            let display_row = row(start);
            if display_row >= rows.end {
                break;
            }
            let mut bound = 1;
            while start + bound < len && row(start + bound) == display_row {
                bound *= 2;
            }
            let low = start + bound / 2 + 1;
            let high = (start + bound).min(len);
            let end = low
                + self.entries[low..high].partition_point(|(anchor, _)| {
                    anchor.to_display_point(snapshot).row() <= display_row
                });
            if let Some(index) = self.winner(start..end, max_severity, excluded_group) {
                diagnostics.push((display_row, &self.entries[index].1));
            }
            start = end;
        }
        diagnostics
    }

    fn winner(
        &self,
        range: Range<usize>,
        max_severity: lsp::DiagnosticSeverity,
        excluded_group: Option<usize>,
    ) -> Option<usize> {
        let excluded = excluded_group
            .and_then(|group| self.groups.get(&group))
            .map_or(&[][..], |indices| {
                let start = indices.partition_point(|index| *index < range.start);
                let end = indices.partition_point(|index| *index < range.end);
                &indices[start..end]
            });
        let precedes = |left: usize, right: usize| precedes(&self.entries, left, right);
        let best_in = |tree: &RangeMinimum| {
            let mut best = None;
            let mut start = range.start;
            for &end in excluded.iter().chain([&range.end]) {
                best = RangeMinimum::better(best, tree.min(start..end, &precedes), &precedes);
                start = end + 1;
            }
            best
        };
        let passes = |index: &usize| self.entries[*index].1.severity <= max_severity;
        best_in(&self.best_primary)
            .filter(passes)
            .or_else(|| best_in(&self.best).filter(passes))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ActiveDiagnosticGroup {
    active_range: Range<Anchor>,
    active_message: language::DiagnosticMessage,
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

        self.go_to_diagnostic_at_cursor(Direction::Next, action.severity, window, cx);
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

        self.go_to_diagnostic_at_cursor(Direction::Prev, action.severity, window, cx);
    }

    fn diagnostics_before_cursor<'a>(
        buffer: &'a MultiBufferSnapshot,
        cursor: MultiBufferOffset,
        severity: GoToDiagnosticSeverityFilter,
    ) -> impl Iterator<Item = DiagnosticEntryRef<'a, MultiBufferOffset>> {
        buffer
            .diagnostics_in_range(MultiBufferOffset(0)..cursor)
            .filter(move |entry| entry.range.start <= cursor)
            .filter(move |entry| severity.matches(entry.diagnostic.severity))
            .filter(|entry| entry.range.start != entry.range.end)
            .filter(|entry| !entry.diagnostic.is_unnecessary)
    }

    fn diagnostics_after_cursor<'a>(
        buffer: &'a MultiBufferSnapshot,
        cursor: MultiBufferOffset,
        severity: GoToDiagnosticSeverityFilter,
    ) -> impl Iterator<Item = DiagnosticEntryRef<'a, MultiBufferOffset>> {
        buffer
            .diagnostics_in_range(cursor..buffer.len())
            .filter(move |entry| entry.range.start >= cursor)
            .filter(move |entry| severity.matches(entry.diagnostic.severity))
            .filter(|entry| entry.range.start != entry.range.end)
            .filter(|entry| !entry.diagnostic.is_unnecessary)
    }

    /// Attempts to expand the diagnostic at the current cursor position,
    /// updating the cursor position to the diagnostic's start point.
    ///
    /// In case there's no diagnostic at the current cursor position, this will
    /// fallback to finding the next or previous diagnostic instead, depending
    /// on the provided `direction`.
    pub fn go_to_diagnostic_at_cursor(
        &mut self,
        direction: Direction,
        severity: GoToDiagnosticSeverityFilter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.defer_input(
            move |editor, window, cx| {
                editor.go_to_diagnostic_at_cursor(direction, severity, window, cx)
            },
            window,
            cx,
        ) {
            return;
        }
        let buffer = self.buffer.read(cx).snapshot(cx);
        let selection = self
            .selections
            .newest::<MultiBufferOffset>(&self.display_snapshot(cx));

        let before = Self::diagnostics_before_cursor(&buffer, selection.start, severity);
        let after = Self::diagnostics_after_cursor(&buffer, selection.start, severity);
        let active_group_id = match &self.active_diagnostics {
            ActiveDiagnostic::Group(group) => Some(group.group_id),
            _ => None,
        };

        let mut cursor_on_active = false;
        let mut target = None;

        for diagnostic in after.chain(before) {
            let contains_cursor = diagnostic.range.contains(&selection.start)
                || diagnostic.range.end == selection.head();

            if !contains_cursor {
                continue;
            }

            if active_group_id == Some(diagnostic.diagnostic.group_id) {
                cursor_on_active = true;
            } else if target.is_none() {
                target = Some(diagnostic);
            }
        }

        match (target, cursor_on_active) {
            (Some(diagnostic), false) => self.activate_diagnostic(&buffer, diagnostic, window, cx),
            _ => self.go_to_diagnostic_in_direction(
                &buffer, &selection, direction, severity, window, cx,
            ),
        }
    }

    fn activate_diagnostic(
        &mut self,
        buffer: &MultiBufferSnapshot,
        diagnostic: DiagnosticEntryRef<MultiBufferOffset>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let diagnostic_start = buffer.anchor_after(diagnostic.range.start);
        let Some((buffer_anchor, _)) = buffer.anchor_to_buffer_anchor(diagnostic_start) else {
            return;
        };
        let buffer_id = buffer_anchor.buffer_id;
        let snapshot = self.snapshot(window, cx);
        if snapshot.intersects_fold(diagnostic.range.start) {
            self.unfold_ranges(std::slice::from_ref(&diagnostic.range), true, false, cx);
        }
        self.change_selections(Default::default(), window, cx, |s| {
            s.select_ranges(vec![diagnostic.range.start..diagnostic.range.start])
        });
        self.activate_diagnostics(buffer_id, diagnostic, window, cx);
        self.refresh_edit_prediction(
            true,
            false,
            EditPredictionRequestTrigger::DiagnosticNavigation,
            window,
            cx,
        );
    }

    pub fn go_to_diagnostic_in_direction(
        &mut self,
        buffer: &MultiBufferSnapshot,
        selection: &Selection<MultiBufferOffset>,
        direction: Direction,
        severity: GoToDiagnosticSeverityFilter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.input_is_deferred(cx) {
            let selection = selection.clone().map(|offset| buffer.anchor_before(offset));
            self.defer_input(
                move |editor, window, cx| {
                    let snapshot = editor.buffer.read(cx).snapshot(cx);
                    let selection = selection.map(|anchor| anchor.to_offset(&snapshot));
                    editor.go_to_diagnostic_in_direction(
                        &snapshot, &selection, direction, severity, window, cx,
                    );
                },
                window,
                cx,
            );
            return;
        }
        let mut active_group_id = None;
        if let ActiveDiagnostic::Group(active_group) = &self.active_diagnostics
            && active_group.active_range.start.to_offset(&buffer) == selection.start
        {
            active_group_id = Some(active_group.group_id);
        }

        let before = Self::diagnostics_before_cursor(&buffer, selection.start, severity);
        let after = Self::diagnostics_after_cursor(&buffer, selection.start, severity);

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

        self.activate_diagnostic(&buffer, next_diagnostic, window, cx);
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
            self.refresh_inline_diagnostics(false, window, cx);
        } else {
            self.inline_diagnostics_update = Task::ready(());
            self.inline_diagnostics.clear();
            cx.notify();
        }
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
                        && entry
                            .diagnostic
                            .message
                            .rendered_eq(&active_diagnostics.active_message)
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
        let buffer = self.buffer.read(cx).snapshot(cx);

        let blocks = if let Some(renderer) = GlobalDiagnosticRenderer::global(cx) {
            let snapshot = self.snapshot(window, cx);
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

            self.display_map.update(cx, |display_map, cx| {
                display_map.insert_blocks(blocks, cx).into_iter().collect()
            })
        } else {
            // Ensure that, even if there's no global renderer set, we still use
            // an empty set of blocks, such that we can record the active group
            // below instead of bailing out.
            HashSet::default()
        };

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
            cx.notify();
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
                            .as_str()
                            .split_once('\n')
                            .map(|(line, _)| line)
                            .map(SharedString::new)
                            .unwrap_or_else(|| {
                                diagnostic_entry
                                    .diagnostic
                                    .message
                                    .as_shared_string()
                                    .clone()
                            });
                        let start_anchor = snapshot.anchor_before(diagnostic_entry.range.start);
                        inline_diagnostics.push((
                            start_anchor,
                            InlineDiagnostic {
                                message,
                                group_id: diagnostic_entry.diagnostic.group_id,
                                start: diagnostic_entry.range.start.to_point(&snapshot),
                                is_primary: diagnostic_entry.diagnostic.is_primary,
                                severity: diagnostic_entry.diagnostic.severity,
                            },
                        ));
                    }
                    InlineDiagnostics::new(inline_diagnostics, &snapshot)
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

    pub fn any_active_diagnostics(&self) -> bool {
        match &self.active_diagnostics {
            ActiveDiagnostic::None => false,
            ActiveDiagnostic::All => true,
            ActiveDiagnostic::Group(_) => true,
        }
    }
}

#[derive(Default)]
struct RangeMinimum {
    leaves: usize,
    nodes: Vec<Option<u32>>,
}

impl RangeMinimum {
    fn new(
        len: usize,
        include: impl Fn(usize) -> bool,
        precedes: &impl Fn(usize, usize) -> bool,
    ) -> Self {
        let leaves = len.next_power_of_two();
        let mut nodes = vec![None; 2 * leaves];
        for index in (0..len).filter(|index| include(*index)) {
            nodes[leaves + index] = u32::try_from(index).ok();
        }
        for node in (1..leaves).rev() {
            let left = nodes[2 * node].map(|index| index as usize);
            let right = nodes[2 * node + 1].map(|index| index as usize);
            nodes[node] =
                Self::better(left, right, precedes).and_then(|index| u32::try_from(index).ok());
        }
        Self { leaves, nodes }
    }

    fn min(&self, range: Range<usize>, precedes: &impl Fn(usize, usize) -> bool) -> Option<usize> {
        let node = |node: usize| {
            self.nodes
                .get(node)
                .copied()
                .flatten()
                .map(|index| index as usize)
        };
        let mut best = None;
        let mut left = range.start + self.leaves;
        let mut right = range.end + self.leaves;
        while left < right {
            if left % 2 == 1 {
                best = Self::better(best, node(left), precedes);
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                best = Self::better(best, node(right), precedes);
            }
            left /= 2;
            right /= 2;
        }
        best
    }

    fn better(
        left: Option<usize>,
        right: Option<usize>,
        precedes: &impl Fn(usize, usize) -> bool,
    ) -> Option<usize> {
        match (left, right) {
            (Some(left), Some(right)) => Some(if precedes(right, left) { right } else { left }),
            (left, right) => left.or(right),
        }
    }
}

fn precedes(entries: &[(Anchor, InlineDiagnostic)], left: usize, right: usize) -> bool {
    let key = |index: usize| {
        let diagnostic = &entries[index].1;
        (
            diagnostic.severity,
            diagnostic.start.row,
            diagnostic.start.column,
            index,
        )
    };
    key(left) < key(right)
}

#[cfg(test)]
mod tests {
    use super::{InlineDiagnostic, InlineDiagnostics};
    use crate::{
        actions::{
            ToggleCodeLens, ToggleDiagnostics, ToggleInlayHints, ToggleInlineDiagnostics,
            ToggleSemanticHighlights,
        },
        editor_tests::init_test,
        test::editor_test_context::EditorTestContext,
    };
    use gpui::SharedString;
    use gpui::{Action, TestAppContext, UpdateGlobal};
    use indoc::indoc;
    use language::DiagnosticSourceKind;
    use language::Point;
    use lsp::LanguageServerId;
    use multi_buffer::{Anchor, MultiBuffer};
    use rand::prelude::*;
    use settings::{DelayMs, SettingsStore};
    use std::cmp::Reverse;
    use std::sync::{
        Arc,
        atomic::{self, AtomicUsize},
    };
    use util::path;

    fn setup_inline_diagnostics(cx: &mut EditorTestContext) {
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    let inline = settings
                        .diagnostics
                        .get_or_insert_default()
                        .inline
                        .get_or_insert_default();
                    inline.enabled = Some(true);
                    inline.update_debounce_ms = Some(DelayMs(0));
                });
            });
        });

        cx.set_state(indoc! {"
            fn func(abc dˇef: i32) -> u32 {
            }
        "});

        let lsp_store =
            cx.update_editor(|editor, _, cx| editor.project().unwrap().read(cx).lsp_store());
        cx.update(|_, cx| {
            lsp_store.update(cx, |lsp_store, cx| {
                lsp_store
                    .update_diagnostics(
                        LanguageServerId(0),
                        lsp::PublishDiagnosticsParams {
                            uri: lsp::Uri::from_file_path(path!("/root/file")).unwrap(),
                            version: None,
                            diagnostics: vec![lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 12),
                                    lsp::Position::new(0, 15),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                message: lsp::DiagnosticMessage::from("cannot find value `def`"),
                                ..Default::default()
                            }],
                        },
                        None,
                        DiagnosticSourceKind::Pushed,
                        &[],
                        cx,
                    )
                    .unwrap()
            });
        });
        cx.run_until_parked();

        cx.update_editor(|editor, _, _| {
            assert_eq!(
                editor.inline_diagnostics.len(),
                1,
                "inline diagnostics should appear after the language server publishes them"
            );
        });
    }

    #[gpui::test]
    async fn test_captured_input_direct_diagnostic_navigation(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        cx.set_state("aˇb\ncd\ntarget\ntail");
        let lsp_store = cx.update_editor(|editor, _, cx| {
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
            editor.project().expect("project").read(cx).lsp_store()
        });
        cx.update(|_, cx| {
            lsp_store.update(cx, |store, cx| {
                store
                    .update_diagnostics(
                        LanguageServerId(0),
                        lsp::PublishDiagnosticsParams {
                            uri: lsp::Uri::from_file_path(path!("/root/file")).expect("file URI"),
                            version: None,
                            diagnostics: vec![lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(2, 0),
                                    lsp::Position::new(2, 6),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                message: lsp::DiagnosticMessage::from("target"),
                                ..lsp::Diagnostic::default()
                            }],
                        },
                        None,
                        DiagnosticSourceKind::Pushed,
                        &[],
                        cx,
                    )
                    .expect("publish diagnostic");
            });
        });
        cx.run_until_parked();
        let completion = cx.update_editor(|editor, window, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                buffer.edit(
                    [(Point::new(1, 0)..Point::new(1, 2), "x".repeat(2_000))],
                    None,
                    cx,
                );
            });
            editor.move_down(&crate::MoveDown, window, cx);
            assert!(editor.input_is_deferred(cx));
            editor.go_to_diagnostic_at_cursor(
                crate::Direction::Next,
                crate::GoToDiagnosticSeverityFilter::default(),
                window,
                cx,
            );
            assert_eq!(
                editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .head(),
                Point::new(0, 1)
            );
            editor.input_completion(cx)
        });
        completion.await.expect("diagnostic input completed");
        cx.editor(|editor, _, cx| {
            assert_eq!(
                editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .head(),
                Point::new(2, 0)
            );
            assert_eq!(editor.active_diagnostic_message(), Some("target"));
        });
    }

    #[gpui::test]
    async fn test_toggle_diagnostics_refreshes_inline_diagnostics(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        setup_inline_diagnostics(&mut cx);

        cx.update_editor(|editor, window, cx| {
            editor.toggle_diagnostics(&ToggleDiagnostics, window, cx);
        });
        cx.run_until_parked();
        cx.update_editor(|editor, _, _| {
            assert_eq!(
                editor.inline_diagnostics.len(),
                0,
                "inline diagnostics should be cleared after disabling diagnostics"
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.toggle_diagnostics(&ToggleDiagnostics, window, cx);
        });
        cx.run_until_parked();
        cx.update_editor(|editor, _, _| {
            assert_eq!(
                editor.inline_diagnostics.len(),
                1,
                "inline diagnostics should reappear after re-enabling diagnostics, without further editor events"
            );
        });
    }

    #[gpui::test]
    async fn test_toggle_inline_diagnostics_notifies_on_hide(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        setup_inline_diagnostics(&mut cx);

        let notify_count = Arc::new(AtomicUsize::new(0));
        let editor = cx.editor.clone();
        let _subscription = cx.update({
            let notify_count = notify_count.clone();
            move |_, cx| {
                cx.observe(&editor, move |_, _| {
                    notify_count.fetch_add(1, atomic::Ordering::SeqCst);
                })
            }
        });

        cx.update_editor(|editor, window, cx| {
            editor.toggle_inline_diagnostics(&ToggleInlineDiagnostics, window, cx);
        });
        cx.update_editor(|editor, _, _| {
            assert_eq!(
                editor.inline_diagnostics.len(),
                0,
                "inline diagnostics should be cleared after toggling them off"
            );
        });
        assert_eq!(
            notify_count.load(atomic::Ordering::SeqCst),
            1,
            "toggling inline diagnostics off should notify to repaint the editor"
        );

        cx.update_editor(|editor, window, cx| {
            editor.toggle_inline_diagnostics(&ToggleInlineDiagnostics, window, cx);
        });
        cx.run_until_parked();
        cx.update_editor(|editor, _, _| {
            assert_eq!(
                editor.inline_diagnostics.len(),
                1,
                "inline diagnostics should reappear after toggling them back on"
            );
        });
    }

    #[gpui::test]
    async fn test_actions_gated_by_lsp_data(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let lsp_data_actions: [&dyn Action; 4] = [
            &ToggleDiagnostics,
            &ToggleInlayHints,
            &ToggleCodeLens,
            &ToggleSemanticHighlights,
        ];

        cx.update_editor(|editor, _, cx| {
            editor.enable_lsp_data = false;
            cx.notify();
        });
        cx.update(|window, cx| {
            for action in lsp_data_actions {
                assert!(
                    !window.is_action_available(action, cx),
                    "{} should not be available when LSP data is disabled",
                    action.name()
                );
            }
        });

        cx.update_editor(|editor, _, cx| {
            editor.enable_lsp_data = true;
            cx.notify();
        });
        cx.update(|window, cx| {
            for action in lsp_data_actions {
                assert!(
                    window.is_action_available(action, cx),
                    "{} should be available again after re-enabling LSP data",
                    action.name()
                );
            }
        });
    }

    #[gpui::test]
    async fn test_toggle_diagnostics_remains_available_after_disabling_diagnostics(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;

        cx.dispatch_action(ToggleDiagnostics);
        cx.update_editor(|editor, window, cx| {
            assert!(
                !editor.diagnostics_enabled(),
                "diagnostics should be disabled after dispatching ToggleDiagnostics"
            );
            assert!(
                window.is_action_available(&ToggleDiagnostics, cx),
                "ToggleDiagnostics should still be available after disabling diagnostics, \
                 so the user can re-enable it"
            );
        });

        cx.dispatch_action(ToggleDiagnostics);
        cx.update_editor(|editor, window, cx| {
            assert!(
                editor.diagnostics_enabled(),
                "diagnostics should be re-enabled after a second dispatch of ToggleDiagnostics"
            );
            assert!(
                window.is_action_available(&ToggleDiagnostics, cx),
                "ToggleDiagnostics should be available again after re-enabling diagnostics"
            );
        });
    }

    #[gpui::test]
    async fn test_toggle_inline_diagnostics_availability(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;

        cx.dispatch_action(ToggleDiagnostics);
        cx.update(|window, cx| {
            assert!(
                !window.is_action_available(&ToggleInlineDiagnostics, cx),
                "ToggleInlineDiagnostics should not be available when diagnostics are disabled"
            );
        });

        cx.dispatch_action(ToggleDiagnostics);
        cx.update(|window, cx| {
            assert!(
                window.is_action_available(&ToggleInlineDiagnostics, cx),
                "ToggleInlineDiagnostics should be available again after re-enabling diagnostics"
            );
        });

        let initial_show = cx.update_editor(|editor, _, _| editor.show_inline_diagnostics());
        cx.dispatch_action(ToggleInlineDiagnostics);
        cx.update_editor(|editor, window, cx| {
            assert_eq!(
                editor.show_inline_diagnostics(),
                !initial_show,
                "inline diagnostics visibility should flip after dispatching ToggleInlineDiagnostics"
            );
            assert!(
                window.is_action_available(&ToggleInlineDiagnostics, cx),
                "ToggleInlineDiagnostics should still be available after toggling it, \
                 so the user can toggle it back"
            );
        });

        cx.dispatch_action(ToggleInlineDiagnostics);
        cx.update_editor(|editor, window, cx| {
            assert_eq!(
                editor.show_inline_diagnostics(),
                initial_show,
                "inline diagnostics visibility should flip back after a second dispatch of ToggleInlineDiagnostics"
            );
            assert!(
                window.is_action_available(&ToggleInlineDiagnostics, cx),
                "ToggleInlineDiagnostics should remain available after toggling it back"
            );
        });

        cx.update_editor(|editor, _, cx| {
            editor.disable_inline_diagnostics();
            cx.notify();
        });
        cx.update(|window, cx| {
            assert!(
                !window.is_action_available(&ToggleInlineDiagnostics, cx),
                "ToggleInlineDiagnostics should not be available in editors with inline diagnostics permanently disabled"
            );
        });
    }

    #[gpui::test(iterations = 20)]
    fn test_inline_diagnostic_winner_matches_sorted_selection(cx: &mut gpui::App, mut rng: StdRng) {
        let buffer = MultiBuffer::build_simple("", cx).read(cx).snapshot(cx);
        let severities = [
            lsp::DiagnosticSeverity::ERROR,
            lsp::DiagnosticSeverity::WARNING,
            lsp::DiagnosticSeverity::INFORMATION,
            lsp::DiagnosticSeverity::HINT,
        ];
        let entries = (0..rng.random_range(0..64))
            .map(|_| {
                (
                    Anchor::Min,
                    InlineDiagnostic {
                        message: SharedString::default(),
                        group_id: rng.random_range(0..4),
                        is_primary: rng.random(),
                        start: Point::new(0, rng.random_range(0..8)),
                        severity: severities[rng.random_range(0..severities.len())],
                    },
                )
            })
            .collect::<Vec<_>>();
        let diagnostics = InlineDiagnostics::new(entries.clone(), &buffer);
        for _ in 0..100 {
            let start = rng.random_range(0..=entries.len());
            let end = rng.random_range(start..=entries.len());
            let max_severity = severities[rng.random_range(0..severities.len())];
            let excluded_group = rng.random_bool(0.5).then(|| rng.random_range(0..4));

            let mut candidates = (start..end)
                .filter(|index| {
                    let diagnostic = &entries[*index].1;
                    diagnostic.severity <= max_severity
                        && Some(diagnostic.group_id) != excluded_group
                })
                .collect::<Vec<_>>();
            candidates.sort_by_key(|index| {
                let diagnostic = &entries[*index].1;
                (
                    diagnostic.severity,
                    Reverse(diagnostic.is_primary),
                    diagnostic.start.row,
                    diagnostic.start.column,
                )
            });
            let expected = candidates
                .iter()
                .find(|index| entries[**index].1.is_primary)
                .or_else(|| candidates.first())
                .copied();

            assert_eq!(
                diagnostics.winner(start..end, max_severity, excluded_group),
                expected
            );
        }
    }
}
