use super::*;

impl GutterDimensions {
    /// The width of the space reserved for the fold indicators,
    /// use alongside 'justify_end' and `gutter_width` to
    /// right align content with the line numbers
    pub fn fold_area_width(&self) -> Pixels {
        self.margin + self.right_padding
    }
}

impl EditorSnapshot {
    pub fn render_crease_toggle(
        &self,
        buffer_row: MultiBufferRow,
        row_contains_cursor: bool,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let folded = self.is_line_folded(buffer_row);
        let mut is_foldable = false;

        if let Some(crease) = self
            .crease_snapshot
            .query_row(buffer_row, self.buffer_snapshot())
        {
            is_foldable = true;
            match crease {
                Crease::Inline { render_toggle, .. } | Crease::Block { render_toggle, .. } => {
                    if let Some(render_toggle) = render_toggle {
                        let toggle_callback =
                            Arc::new(move |folded, window: &mut Window, cx: &mut App| {
                                if folded {
                                    editor.update(cx, |editor, cx| {
                                        editor.fold_at(buffer_row, window, cx)
                                    });
                                } else {
                                    editor.update(cx, |editor, cx| {
                                        editor.unfold_at(buffer_row, window, cx)
                                    });
                                }
                            });
                        return Some((render_toggle)(
                            buffer_row,
                            folded,
                            toggle_callback,
                            window,
                            cx,
                        ));
                    }
                }
            }
        }

        is_foldable |= !self.use_lsp_folding_ranges && self.starts_indent(buffer_row);

        if folded || (is_foldable && (row_contains_cursor || self.gutter_hovered)) {
            Some(
                Disclosure::new(("gutter_crease", buffer_row.0), !folded)
                    .toggle_state(folded)
                    .on_click(window.listener_for(&editor, move |this, _e, window, cx| {
                        if folded {
                            this.unfold_at(buffer_row, window, cx);
                        } else {
                            this.fold_at(buffer_row, window, cx);
                        }
                    }))
                    .into_any_element(),
            )
        } else {
            None
        }
    }

    pub fn render_crease_trailer(
        &self,
        buffer_row: MultiBufferRow,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let folded = self.is_line_folded(buffer_row);
        if let Crease::Inline { render_trailer, .. } = self
            .crease_snapshot
            .query_row(buffer_row, self.buffer_snapshot())?
        {
            let render_trailer = render_trailer.as_ref()?;
            Some(render_trailer(buffer_row, folded, window, cx))
        } else {
            None
        }
    }
}

impl Editor {
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

    pub fn insert_creases(
        &mut self,
        creases: impl IntoIterator<Item = Crease<Anchor>>,
        cx: &mut Context<Self>,
    ) -> Vec<CreaseId> {
        self.display_map
            .update(cx, |map, cx| map.insert_creases(creases, cx))
    }

    pub fn remove_creases(
        &mut self,
        ids: impl IntoIterator<Item = CreaseId>,
        cx: &mut Context<Self>,
    ) -> Vec<(CreaseId, Range<Anchor>)> {
        self.display_map
            .update(cx, |map, cx| map.remove_creases(ids, cx))
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

    pub(super) fn unfold_buffers_with_selections(&mut self, cx: &mut Context<Self>) {
        if self.buffer().read(cx).is_singleton() {
            return;
        }
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let buffer_ids: HashSet<BufferId> = self
            .selections
            .disjoint_anchor_ranges()
            .flat_map(|range| snapshot.buffer_ids_for_range(range))
            .collect();
        for buffer_id in buffer_ids {
            self.unfold_buffer(buffer_id, cx);
        }
    }

    pub(super) fn folds_did_change(&mut self, cx: &mut Context<Self>) {
        use text::ToOffset as _;

        if self.mode.is_minimap()
            || WorkspaceSettings::get(None, cx).restore_on_startup
                == RestoreOnStartupBehavior::EmptyTab
        {
            return;
        }

        let display_snapshot = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let Some(buffer_snapshot) = display_snapshot.buffer_snapshot().as_singleton() else {
            return;
        };
        let inmemory_folds = display_snapshot
            .folds_in_range(MultiBufferOffset(0)..display_snapshot.buffer_snapshot().len())
            .map(|fold| {
                let start = fold.range.start.text_anchor_in(buffer_snapshot);
                let end = fold.range.end.text_anchor_in(buffer_snapshot);
                (start..end).to_point(buffer_snapshot)
            })
            .collect();
        self.update_restoration_data(cx, |data| {
            data.folds = inmemory_folds;
        });

        let Some(workspace_id) = self.workspace_serialization_id(cx) else {
            return;
        };

        // Get file path for path-based fold storage (survives tab close)
        let Some(file_path) = self.buffer().read(cx).as_singleton().and_then(|buffer| {
            project::File::from_dyn(buffer.read(cx).file())
                .map(|file| Arc::<Path>::from(file.abs_path(cx)))
        }) else {
            return;
        };

        let background_executor = cx.background_executor().clone();
        const FINGERPRINT_LEN: usize = 32;
        let db_folds = display_snapshot
            .folds_in_range(MultiBufferOffset(0)..display_snapshot.buffer_snapshot().len())
            .map(|fold| {
                let start = fold
                    .range
                    .start
                    .text_anchor_in(buffer_snapshot)
                    .to_offset(buffer_snapshot);
                let end = fold
                    .range
                    .end
                    .text_anchor_in(buffer_snapshot)
                    .to_offset(buffer_snapshot);

                // Extract fingerprints - content at fold boundaries for validation on restore
                // Both fingerprints must be INSIDE the fold to avoid capturing surrounding
                // content that might change independently.
                // start_fp: first min(32, fold_len) bytes of fold content
                // end_fp: last min(32, fold_len) bytes of fold content
                // Clip to character boundaries to handle multibyte UTF-8 characters.
                let fold_len = end - start;
                let start_fp_end = buffer_snapshot
                    .clip_offset(start + std::cmp::min(FINGERPRINT_LEN, fold_len), Bias::Left);
                let start_fp: String = buffer_snapshot
                    .text_for_range(start..start_fp_end)
                    .collect();
                let end_fp_start = buffer_snapshot
                    .clip_offset(end.saturating_sub(FINGERPRINT_LEN).max(start), Bias::Right);
                let end_fp: String = buffer_snapshot.text_for_range(end_fp_start..end).collect();

                (start, end, start_fp, end_fp)
            })
            .collect::<Vec<_>>();
        let db = EditorDb::global(cx);
        self.serialize_folds = cx.background_spawn(async move {
            background_executor.timer(SERIALIZATION_THROTTLE_TIME).await;
            if db_folds.is_empty() {
                // No folds - delete any persisted folds for this file
                db.delete_file_folds(workspace_id, file_path)
                    .await
                    .with_context(|| format!("deleting file folds for workspace {workspace_id:?}"))
                    .log_err();
            } else {
                db.save_file_folds(workspace_id, file_path, db_folds)
                    .await
                    .with_context(|| {
                        format!("persisting file folds for workspace {workspace_id:?}")
                    })
                    .log_err();
            }
        });
    }

    pub(super) fn refresh_single_line_folds(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        struct NewlineFold;
        let type_id = std::any::TypeId::of::<NewlineFold>();
        if !self.mode.is_single_line() {
            return;
        }
        let snapshot = self.snapshot(window, cx);
        if snapshot.buffer_snapshot().max_point().row == 0 {
            return;
        }
        let task = cx.background_spawn(async move {
            let new_newlines = snapshot
                .buffer_chars_at(MultiBufferOffset(0))
                .filter_map(|(c, i)| {
                    if c == '\n' {
                        Some(
                            snapshot.buffer_snapshot().anchor_after(i)
                                ..snapshot.buffer_snapshot().anchor_before(i + 1usize),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let existing_newlines = snapshot
                .folds_in_range(MultiBufferOffset(0)..snapshot.buffer_snapshot().len())
                .filter_map(|fold| {
                    if fold.placeholder.type_tag == Some(type_id) {
                        Some(fold.range.start..fold.range.end)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            (new_newlines, existing_newlines)
        });
        self.folding_newlines = cx.spawn(async move |this, cx| {
            let (new_newlines, existing_newlines) = task.await;
            if new_newlines == existing_newlines {
                return;
            }
            let placeholder = FoldPlaceholder {
                render: Arc::new(move |_, _, cx| {
                    div()
                        .bg(cx.theme().status().hint_background)
                        .border_b_1()
                        .size_full()
                        .font(ThemeSettings::get_global(cx).buffer_font.clone())
                        .border_color(cx.theme().status().hint)
                        .child("\\n")
                        .into_any()
                }),
                constrain_width: false,
                merge_adjacent: false,
                type_tag: Some(type_id),
                collapsed_text: None,
            };
            let creases = new_newlines
                .into_iter()
                .map(|range| Crease::simple(range, placeholder.clone()))
                .collect();
            this.update(cx, |this, cx| {
                this.display_map.update(cx, |display_map, cx| {
                    display_map.remove_folds_with_type(existing_newlines, type_id, cx);
                    display_map.fold(creases, cx);
                });
            })
            .ok();
        });
    }

    /// Load folds from the file_folds database table by file path.
    /// Used when manually opening a file that was previously closed.
    pub(super) fn load_folds_from_db(
        &mut self,
        workspace_id: WorkspaceId,
        file_path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if self.mode.is_minimap()
            || WorkspaceSettings::get(None, cx).restore_on_startup
                == RestoreOnStartupBehavior::EmptyTab
        {
            return;
        }

        let Some(folds) = EditorDb::global(cx)
            .get_file_folds(workspace_id, &file_path)
            .log_err()
        else {
            return;
        };
        if folds.is_empty() {
            return;
        }

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let snapshot_len = snapshot.len().0;

        // Helper: search for fingerprint in buffer, return offset if found
        let find_fingerprint = |fingerprint: &str, search_start: usize| -> Option<usize> {
            let search_start = snapshot
                .clip_offset(MultiBufferOffset(search_start), Bias::Left)
                .0;
            let search_end = snapshot_len.saturating_sub(fingerprint.len());

            let mut byte_offset = search_start;
            for ch in snapshot.chars_at(MultiBufferOffset(search_start)) {
                if byte_offset > search_end {
                    break;
                }
                if snapshot.contains_str_at(MultiBufferOffset(byte_offset), fingerprint) {
                    return Some(byte_offset);
                }
                byte_offset += ch.len_utf8();
            }
            None
        };

        let mut search_start = 0usize;

        let valid_folds: Vec<_> = folds
            .into_iter()
            .filter_map(|(stored_start, stored_end, start_fp, end_fp)| {
                let sfp = start_fp?;
                let efp = end_fp?;
                let efp_len = efp.len();

                let start_matches = stored_start < snapshot_len
                    && snapshot.contains_str_at(MultiBufferOffset(stored_start), &sfp);
                let efp_check_pos = stored_end.saturating_sub(efp_len);
                let end_matches = efp_check_pos >= stored_start
                    && stored_end <= snapshot_len
                    && snapshot.contains_str_at(MultiBufferOffset(efp_check_pos), &efp);

                let (new_start, new_end) = if start_matches && end_matches {
                    (stored_start, stored_end)
                } else if sfp == efp {
                    let new_start = find_fingerprint(&sfp, search_start)?;
                    let fold_len = stored_end - stored_start;
                    let new_end = new_start + fold_len;
                    (new_start, new_end)
                } else {
                    let new_start = find_fingerprint(&sfp, search_start)?;
                    let efp_pos = find_fingerprint(&efp, new_start + sfp.len())?;
                    let new_end = efp_pos + efp_len;
                    (new_start, new_end)
                };

                search_start = new_end;

                if new_end <= new_start {
                    return None;
                }

                Some(
                    snapshot.clip_offset(MultiBufferOffset(new_start), Bias::Left)
                        ..snapshot.clip_offset(MultiBufferOffset(new_end), Bias::Right),
                )
            })
            .collect();

        if !valid_folds.is_empty() {
            self.fold_ranges(valid_folds, false, window, cx);
        }
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
}
