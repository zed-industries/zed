use super::*;

impl Editor {
    pub fn set_mark(&mut self, _: &actions::SetMark, window: &mut Window, cx: &mut Context<Self>) {
        if self.selection_mark_mode {
            self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.move_with(&mut |_, sel| {
                    sel.collapse_to(sel.head(), SelectionGoal::None);
                });
            })
        }
        self.selection_mark_mode = true;
        cx.notify();
    }

    pub fn swap_selection_ends(
        &mut self,
        _: &actions::SwapSelectionEnds,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.move_with(&mut |_, sel| {
                if sel.start != sel.end {
                    sel.reversed = !sel.reversed
                }
            });
        });
        self.request_autoscroll(Autoscroll::newest(), cx);
        cx.notify();
    }

    pub fn toggle_focus(
        workspace: &mut Workspace,
        _: &actions::ToggleFocus,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(item) = workspace.recent_active_item_by_type::<Self>(cx) else {
            return;
        };
        workspace.activate_item(&item, true, true, window, cx);
    }

    pub fn toggle_fold(
        &mut self,
        _: &actions::ToggleFold,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.buffer_kind(cx) == ItemBufferKind::Singleton {
            let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
            let selection = self.selections.newest::<Point>(&display_map);

            let range = if selection.is_empty() {
                let point = selection.head().to_display_point(&display_map);
                let start = DisplayPoint::new(point.row(), 0).to_point(&display_map);
                let end = DisplayPoint::new(point.row(), display_map.line_len(point.row()))
                    .to_point(&display_map);
                start..end
            } else {
                selection.range()
            };
            if display_map.folds_in_range(range).next().is_some() {
                self.unfold_lines(&Default::default(), window, cx)
            } else {
                self.fold(&Default::default(), window, cx)
            }
        } else {
            let multi_buffer_snapshot = self.buffer.read(cx).snapshot(cx);
            let buffer_ids: HashSet<_> = self
                .selections
                .disjoint_anchor_ranges()
                .flat_map(|range| multi_buffer_snapshot.buffer_ids_for_range(range))
                .collect();

            let should_unfold = buffer_ids
                .iter()
                .any(|buffer_id| self.is_buffer_folded(*buffer_id, cx));

            for buffer_id in buffer_ids {
                if should_unfold {
                    self.unfold_buffer(buffer_id, cx);
                } else {
                    self.fold_buffer(buffer_id, cx);
                }
            }
        }
    }

    pub fn toggle_fold_recursive(
        &mut self,
        _: &actions::ToggleFoldRecursive,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selection = self.selections.newest::<Point>(&self.display_snapshot(cx));

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let range = if selection.is_empty() {
            let point = selection.head().to_display_point(&display_map);
            let start = DisplayPoint::new(point.row(), 0).to_point(&display_map);
            let end = DisplayPoint::new(point.row(), display_map.line_len(point.row()))
                .to_point(&display_map);
            start..end
        } else {
            selection.range()
        };
        if display_map.folds_in_range(range).next().is_some() {
            self.unfold_recursive(&Default::default(), window, cx)
        } else {
            self.fold_recursive(&Default::default(), window, cx)
        }
    }

    pub fn fold(&mut self, _: &actions::Fold, window: &mut Window, cx: &mut Context<Self>) {
        if self.buffer_kind(cx) == ItemBufferKind::Singleton {
            let mut to_fold = Vec::new();
            let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
            let selections = self.selections.all_adjusted(&display_map);

            for selection in selections {
                let range = selection.range().sorted();
                let buffer_start_row = range.start.row;

                if range.start.row != range.end.row {
                    let mut found = false;
                    let mut row = range.start.row;
                    while row <= range.end.row {
                        if let Some(crease) = display_map.crease_for_buffer_row(MultiBufferRow(row))
                        {
                            found = true;
                            row = crease.range().end.row + 1;
                            to_fold.push(crease);
                        } else {
                            row += 1
                        }
                    }
                    if found {
                        continue;
                    }
                }

                for row in (0..=range.start.row).rev() {
                    if let Some(crease) = display_map.crease_for_buffer_row(MultiBufferRow(row))
                        && crease.range().end.row >= buffer_start_row
                    {
                        to_fold.push(crease);
                        if row <= range.start.row {
                            break;
                        }
                    }
                }
            }

            self.fold_creases(to_fold, true, window, cx);
        } else {
            let multi_buffer_snapshot = self.buffer.read(cx).snapshot(cx);
            let buffer_ids = self
                .selections
                .disjoint_anchor_ranges()
                .flat_map(|range| multi_buffer_snapshot.buffer_ids_for_range(range))
                .collect::<HashSet<_>>();
            for buffer_id in buffer_ids {
                self.fold_buffer(buffer_id, cx);
            }
        }
    }

    pub fn toggle_fold_all(
        &mut self,
        _: &actions::ToggleFoldAll,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let has_folds = if self.buffer.read(cx).is_singleton() {
            let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
            let has_folds = display_map
                .folds_in_range(MultiBufferOffset(0)..display_map.buffer_snapshot().len())
                .next()
                .is_some();
            has_folds
        } else {
            let snapshot = self.buffer.read(cx).snapshot(cx);
            let has_folds = snapshot
                .all_buffer_ids()
                .any(|buffer_id| self.is_buffer_folded(buffer_id, cx));
            has_folds
        };

        if has_folds {
            self.unfold_all(&actions::UnfoldAll, window, cx);
        } else {
            self.fold_all(&actions::FoldAll, window, cx);
        }
    }

    pub(super) fn fold_at_level(
        &mut self,
        fold_at: &FoldAtLevel,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.buffer.read(cx).is_singleton() {
            return;
        }

        let fold_at_level = fold_at.0;
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let mut to_fold = Vec::new();
        let mut stack = vec![(0, snapshot.max_row().0, 1)];

        let row_ranges_to_keep: Vec<Range<u32>> = self
            .selections
            .all::<Point>(&self.display_snapshot(cx))
            .into_iter()
            .map(|sel| sel.start.row..sel.end.row)
            .collect();

        while let Some((mut start_row, end_row, current_level)) = stack.pop() {
            while start_row < end_row {
                match self
                    .snapshot(window, cx)
                    .crease_for_buffer_row(MultiBufferRow(start_row))
                {
                    Some(crease) => {
                        let nested_start_row = crease.range().start.row + 1;
                        let nested_end_row = crease.range().end.row;

                        if current_level < fold_at_level {
                            stack.push((nested_start_row, nested_end_row, current_level + 1));
                        } else if current_level == fold_at_level {
                            // Fold iff there is no selection completely contained within the fold region
                            if !row_ranges_to_keep.iter().any(|selection| {
                                selection.end >= nested_start_row
                                    && selection.start <= nested_end_row
                            }) {
                                to_fold.push(crease);
                            }
                        }

                        start_row = nested_end_row + 1;
                    }
                    None => start_row += 1,
                }
            }
        }

        self.fold_creases(to_fold, true, window, cx);
    }

    pub fn fold_at_level_1(
        &mut self,
        _: &actions::FoldAtLevel1,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.fold_at_level(&actions::FoldAtLevel(1), window, cx);
    }

    pub fn fold_at_level_2(
        &mut self,
        _: &actions::FoldAtLevel2,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.fold_at_level(&actions::FoldAtLevel(2), window, cx);
    }

    pub fn fold_at_level_3(
        &mut self,
        _: &actions::FoldAtLevel3,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.fold_at_level(&actions::FoldAtLevel(3), window, cx);
    }

    pub fn fold_at_level_4(
        &mut self,
        _: &actions::FoldAtLevel4,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.fold_at_level(&actions::FoldAtLevel(4), window, cx);
    }

    pub fn fold_at_level_5(
        &mut self,
        _: &actions::FoldAtLevel5,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.fold_at_level(&actions::FoldAtLevel(5), window, cx);
    }

    pub fn fold_at_level_6(
        &mut self,
        _: &actions::FoldAtLevel6,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.fold_at_level(&actions::FoldAtLevel(6), window, cx);
    }

    pub fn fold_at_level_7(
        &mut self,
        _: &actions::FoldAtLevel7,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.fold_at_level(&actions::FoldAtLevel(7), window, cx);
    }

    pub fn fold_at_level_8(
        &mut self,
        _: &actions::FoldAtLevel8,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.fold_at_level(&actions::FoldAtLevel(8), window, cx);
    }

    pub fn fold_at_level_9(
        &mut self,
        _: &actions::FoldAtLevel9,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.fold_at_level(&actions::FoldAtLevel(9), window, cx);
    }

    pub fn fold_all(&mut self, _: &actions::FoldAll, window: &mut Window, cx: &mut Context<Self>) {
        if self.buffer.read(cx).is_singleton() {
            let mut fold_ranges = Vec::new();
            let snapshot = self.buffer.read(cx).snapshot(cx);

            for row in 0..snapshot.max_row().0 {
                if let Some(foldable_range) = self
                    .snapshot(window, cx)
                    .crease_for_buffer_row(MultiBufferRow(row))
                {
                    fold_ranges.push(foldable_range);
                }
            }

            self.fold_creases(fold_ranges, true, window, cx);
        } else {
            self.toggle_fold_multiple_buffers = cx.spawn_in(window, async move |editor, cx| {
                editor
                    .update_in(cx, |editor, _, cx| {
                        let snapshot = editor.buffer.read(cx).snapshot(cx);
                        for buffer_id in snapshot.all_buffer_ids() {
                            editor.fold_buffer(buffer_id, cx);
                        }
                    })
                    .ok();
            });
        }
    }

    pub fn fold_function_bodies(
        &mut self,
        _: &actions::FoldFunctionBodies,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);

        let ranges = snapshot
            .text_object_ranges(
                MultiBufferOffset(0)..snapshot.len(),
                TreeSitterOptions::default(),
            )
            .filter_map(|(range, obj)| (obj == TextObject::InsideFunction).then_some(range))
            .collect::<Vec<_>>();

        let creases = ranges
            .into_iter()
            .map(|range| Crease::simple(range, self.display_map.read(cx).fold_placeholder.clone()))
            .collect();

        self.fold_creases(creases, true, window, cx);
    }

    pub fn fold_recursive(
        &mut self,
        _: &actions::FoldRecursive,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut to_fold = Vec::new();
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all_adjusted(&display_map);

        for selection in selections {
            let range = selection.range().sorted();
            let buffer_start_row = range.start.row;

            if range.start.row != range.end.row {
                let mut found = false;
                for row in range.start.row..=range.end.row {
                    if let Some(crease) = display_map.crease_for_buffer_row(MultiBufferRow(row)) {
                        found = true;
                        to_fold.push(crease);
                    }
                }
                if found {
                    continue;
                }
            }

            for row in (0..=range.start.row).rev() {
                if let Some(crease) = display_map.crease_for_buffer_row(MultiBufferRow(row)) {
                    if crease.range().end.row >= buffer_start_row {
                        to_fold.push(crease);
                    } else {
                        break;
                    }
                }
            }
        }

        self.fold_creases(to_fold, true, window, cx);
    }

    pub fn fold_at(
        &mut self,
        buffer_row: MultiBufferRow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        if let Some(crease) = display_map.crease_for_buffer_row(buffer_row) {
            let autoscroll = self
                .selections
                .all::<Point>(&display_map)
                .iter()
                .any(|selection| crease.range().overlaps(&selection.range()));

            self.fold_creases(vec![crease], autoscroll, window, cx);
        }
    }

    pub fn unfold_lines(&mut self, _: &UnfoldLines, _window: &mut Window, cx: &mut Context<Self>) {
        if self.buffer_kind(cx) == ItemBufferKind::Singleton {
            let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
            let buffer = display_map.buffer_snapshot();
            let selections = self.selections.all::<Point>(&display_map);
            let ranges = selections
                .iter()
                .map(|s| {
                    let range = s.display_range(&display_map).sorted();
                    let mut start = range.start.to_point(&display_map);
                    let mut end = range.end.to_point(&display_map);
                    start.column = 0;
                    end.column = buffer.line_len(MultiBufferRow(end.row));
                    start..end
                })
                .collect::<Vec<_>>();

            self.unfold_ranges(&ranges, true, true, cx);
        } else {
            let multi_buffer_snapshot = self.buffer.read(cx).snapshot(cx);
            let buffer_ids = self
                .selections
                .disjoint_anchor_ranges()
                .flat_map(|range| multi_buffer_snapshot.buffer_ids_for_range(range))
                .collect::<HashSet<_>>();
            for buffer_id in buffer_ids {
                self.unfold_buffer(buffer_id, cx);
            }
        }
    }

    pub fn unfold_recursive(
        &mut self,
        _: &UnfoldRecursive,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(&display_map);
        let ranges = selections
            .iter()
            .map(|s| {
                let mut range = s.display_range(&display_map).sorted();
                *range.start.column_mut() = 0;
                *range.end.column_mut() = display_map.line_len(range.end.row());
                let start = range.start.to_point(&display_map);
                let end = range.end.to_point(&display_map);
                start..end
            })
            .collect::<Vec<_>>();

        self.unfold_ranges(&ranges, true, true, cx);
    }

    pub fn unfold_at(
        &mut self,
        buffer_row: MultiBufferRow,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));

        let intersection_range = Point::new(buffer_row.0, 0)
            ..Point::new(
                buffer_row.0,
                display_map.buffer_snapshot().line_len(buffer_row),
            );

        let autoscroll = self
            .selections
            .all::<Point>(&display_map)
            .iter()
            .any(|selection| RangeExt::overlaps(&selection.range(), &intersection_range));

        self.unfold_ranges(&[intersection_range], true, autoscroll, cx);
    }

    pub fn unfold_all(
        &mut self,
        _: &actions::UnfoldAll,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.buffer.read(cx).is_singleton() {
            let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
            self.unfold_ranges(
                &[MultiBufferOffset(0)..display_map.buffer_snapshot().len()],
                true,
                true,
                cx,
            );
        } else {
            self.toggle_fold_multiple_buffers = cx.spawn(async move |editor, cx| {
                editor
                    .update(cx, |editor, cx| {
                        let snapshot = editor.buffer.read(cx).snapshot(cx);
                        for buffer_id in snapshot.all_buffer_ids() {
                            editor.unfold_buffer(buffer_id, cx);
                        }
                    })
                    .ok();
            });
        }
    }

    pub fn fold_selected_ranges(
        &mut self,
        _: &FoldSelectedRanges,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all_adjusted(&display_map);
        let ranges = selections
            .into_iter()
            .map(|s| Crease::simple(s.range(), display_map.fold_placeholder.clone()))
            .collect::<Vec<_>>();
        self.fold_creases(ranges, true, window, cx);
    }

    pub fn fold_ranges<T: ToOffset + Clone>(
        &mut self,
        ranges: Vec<Range<T>>,
        auto_scroll: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let ranges = ranges
            .into_iter()
            .map(|r| Crease::simple(r, display_map.fold_placeholder.clone()))
            .collect::<Vec<_>>();
        self.fold_creases(ranges, auto_scroll, window, cx);
    }

    pub fn fold_creases<T: ToOffset + Clone>(
        &mut self,
        creases: Vec<Crease<T>>,
        auto_scroll: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if creases.is_empty() {
            return;
        }

        self.display_map.update(cx, |map, cx| map.fold(creases, cx));

        if auto_scroll {
            self.request_autoscroll(Autoscroll::fit(), cx);
        }

        cx.notify();

        self.scrollbar_marker_state.dirty = true;
        self.update_data_on_scroll(false, window, cx);
        self.folds_did_change(cx);
    }

    /// Removes any folds whose ranges intersect any of the given ranges.
    pub fn unfold_ranges<T: ToOffset + Clone>(
        &mut self,
        ranges: &[Range<T>],
        inclusive: bool,
        auto_scroll: bool,
        cx: &mut Context<Self>,
    ) {
        self.remove_folds_with(ranges, auto_scroll, cx, |map, cx| {
            map.unfold_intersecting(ranges.iter().cloned(), inclusive, cx);
        });
        self.folds_did_change(cx);
    }

    pub fn fold_buffer(&mut self, buffer_id: BufferId, cx: &mut Context<Self>) {
        self.fold_buffers([buffer_id], cx);
    }

    pub fn fold_buffers(
        &mut self,
        buffer_ids: impl IntoIterator<Item = BufferId>,
        cx: &mut Context<Self>,
    ) {
        if self.buffer().read(cx).is_singleton() {
            return;
        }

        let ids_to_fold: Vec<BufferId> = buffer_ids
            .into_iter()
            .filter(|id| !self.is_buffer_folded(*id, cx))
            .collect();

        if ids_to_fold.is_empty() {
            return;
        }

        self.display_map.update(cx, |display_map, cx| {
            display_map.fold_buffers(ids_to_fold.clone(), cx)
        });

        let snapshot = self.display_snapshot(cx);
        self.selections.change_with(&snapshot, |selections| {
            for buffer_id in ids_to_fold.iter().copied() {
                selections.remove_selections_from_buffer(buffer_id);
            }
        });

        cx.emit(EditorEvent::BufferFoldToggled {
            ids: ids_to_fold,
            folded: true,
        });
        cx.notify();
    }

    pub fn unfold_buffer(&mut self, buffer_id: BufferId, cx: &mut Context<Self>) {
        if self.buffer().read(cx).is_singleton() || !self.is_buffer_folded(buffer_id, cx) {
            return;
        }
        self.display_map.update(cx, |display_map, cx| {
            display_map.unfold_buffers([buffer_id], cx);
        });
        cx.emit(EditorEvent::BufferFoldToggled {
            ids: vec![buffer_id],
            folded: false,
        });
        cx.notify();
    }

    pub fn is_buffer_folded(&self, buffer: BufferId, cx: &App) -> bool {
        self.display_map.read(cx).is_buffer_folded(buffer)
    }

    pub fn has_any_buffer_folded(&self, cx: &App) -> bool {
        if self.buffer().read(cx).is_singleton() {
            return false;
        }
        !self.folded_buffers(cx).is_empty()
    }

    pub fn folded_buffers<'a>(&self, cx: &'a App) -> &'a HashSet<BufferId> {
        self.display_map.read(cx).folded_buffers()
    }

    pub fn disable_header_for_buffer(&mut self, buffer_id: BufferId, cx: &mut Context<Self>) {
        self.display_map.update(cx, |display_map, cx| {
            display_map.disable_header_for_buffer(buffer_id, cx);
        });
        cx.notify();
    }

    /// Removes any folds with the given ranges.
    pub fn remove_folds_with_type<T: ToOffset + Clone>(
        &mut self,
        ranges: &[Range<T>],
        type_id: TypeId,
        auto_scroll: bool,
        cx: &mut Context<Self>,
    ) {
        self.remove_folds_with(ranges, auto_scroll, cx, |map, cx| {
            map.remove_folds_with_type(ranges.iter().cloned(), type_id, cx)
        });
        self.folds_did_change(cx);
    }

    fn remove_folds_with<T: ToOffset + Clone>(
        &mut self,
        ranges: &[Range<T>],
        auto_scroll: bool,
        cx: &mut Context<Self>,
        update: impl FnOnce(&mut DisplayMap, &mut Context<DisplayMap>),
    ) {
        if ranges.is_empty() {
            return;
        }

        self.display_map.update(cx, update);

        if auto_scroll {
            self.request_autoscroll(Autoscroll::fit(), cx);
        }

        cx.notify();
        self.scrollbar_marker_state.dirty = true;
        self.active_indent_guides_state.dirty = true;
    }

    pub fn update_renderer_widths(
        &mut self,
        widths: impl IntoIterator<Item = (ChunkRendererId, Pixels)>,
        cx: &mut Context<Self>,
    ) -> bool {
        self.display_map
            .update(cx, |map, cx| map.update_fold_widths(widths, cx))
    }

    pub fn default_fold_placeholder(&self, cx: &App) -> FoldPlaceholder {
        self.display_map.read(cx).fold_placeholder.clone()
    }

    pub fn set_expand_all_diff_hunks(&mut self, cx: &mut App) {
        self.buffer.update(cx, |buffer, cx| {
            buffer.set_all_diff_hunks_expanded(cx);
        });
    }

    pub fn expand_all_diff_hunks(
        &mut self,
        _: &ExpandAllDiffHunks,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.buffer.update(cx, |buffer, cx| {
            buffer.expand_diff_hunks(vec![Anchor::Min..Anchor::Max], cx)
        });
    }

    pub fn collapse_all_diff_hunks(
        &mut self,
        _: &CollapseAllDiffHunks,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.buffer.update(cx, |buffer, cx| {
            buffer.collapse_diff_hunks(vec![Anchor::Min..Anchor::Max], cx)
        });
    }

    pub fn toggle_selected_diff_hunks(
        &mut self,
        _: &ToggleSelectedDiffHunks,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ranges: Vec<_> = self
            .selections
            .disjoint_anchors()
            .iter()
            .map(|s| s.range())
            .collect();
        self.toggle_diff_hunks_in_ranges(ranges, cx);
    }
}
