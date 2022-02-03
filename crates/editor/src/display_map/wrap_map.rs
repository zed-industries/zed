use super::{
    fold_map,
    tab_map::{self, TabEdit, TabPoint, TabSnapshot},
};
use crate::{MultiBufferSnapshot, Point};
use gpui::{
    fonts::FontId, text_layout::LineWrapper, Entity, ModelContext, ModelHandle, MutableAppContext,
    Task,
};
use language::Chunk;
use lazy_static::lazy_static;
use smol::future::yield_now;
use std::{cmp, collections::VecDeque, mem, ops::Range, time::Duration};
use sum_tree::{Bias, Cursor, SumTree};
use text::Patch;

pub use super::tab_map::TextSummary;
pub type WrapEdit = text::Edit<u32>;

pub struct WrapMap {
    snapshot: WrapSnapshot,
    pending_edits: VecDeque<(TabSnapshot, Vec<TabEdit>)>,
    interpolated_edits: Patch<u32>,
    edits_since_sync: Patch<u32>,
    wrap_width: Option<f32>,
    background_task: Option<Task<()>>,
    font: (FontId, f32),
}

impl Entity for WrapMap {
    type Event = ();
}

#[derive(Clone)]
pub struct WrapSnapshot {
    tab_snapshot: TabSnapshot,
    transforms: SumTree<Transform>,
    interpolated: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Transform {
    summary: TransformSummary,
    display_text: Option<&'static str>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TransformSummary {
    input: TextSummary,
    output: TextSummary,
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct WrapPoint(pub super::Point);

pub struct WrapChunks<'a> {
    input_chunks: tab_map::TabChunks<'a>,
    input_chunk: Chunk<'a>,
    output_position: WrapPoint,
    max_output_row: u32,
    transforms: Cursor<'a, Transform, (WrapPoint, TabPoint)>,
}

pub struct WrapBufferRows<'a> {
    input_buffer_rows: fold_map::FoldBufferRows<'a>,
    input_buffer_row: Option<u32>,
    output_row: u32,
    soft_wrapped: bool,
    max_output_row: u32,
    transforms: Cursor<'a, Transform, (WrapPoint, TabPoint)>,
}

impl WrapMap {
    pub fn new(
        tab_snapshot: TabSnapshot,
        font_id: FontId,
        font_size: f32,
        wrap_width: Option<f32>,
        cx: &mut MutableAppContext,
    ) -> (ModelHandle<Self>, WrapSnapshot) {
        let handle = cx.add_model(|cx| {
            let mut this = Self {
                font: (font_id, font_size),
                wrap_width: None,
                pending_edits: Default::default(),
                interpolated_edits: Default::default(),
                edits_since_sync: Default::default(),
                snapshot: WrapSnapshot::new(tab_snapshot),
                background_task: None,
            };
            this.set_wrap_width(wrap_width, cx);
            mem::take(&mut this.edits_since_sync);
            this
        });
        let snapshot = handle.read(cx).snapshot.clone();
        (handle, snapshot)
    }

    #[cfg(test)]
    pub fn is_rewrapping(&self) -> bool {
        self.background_task.is_some()
    }

    pub fn sync(
        &mut self,
        tab_snapshot: TabSnapshot,
        edits: Vec<TabEdit>,
        cx: &mut ModelContext<Self>,
    ) -> (WrapSnapshot, Vec<WrapEdit>) {
        if self.wrap_width.is_some() {
            self.pending_edits.push_back((tab_snapshot, edits));
            self.flush_edits(cx);
        } else {
            self.edits_since_sync = self
                .edits_since_sync
                .compose(&self.snapshot.interpolate(tab_snapshot, &edits));
            self.snapshot.interpolated = false;
        }

        (
            self.snapshot.clone(),
            mem::take(&mut self.edits_since_sync).into_inner(),
        )
    }

    pub fn set_font(&mut self, font_id: FontId, font_size: f32, cx: &mut ModelContext<Self>) {
        if (font_id, font_size) != self.font {
            self.font = (font_id, font_size);
            self.rewrap(cx)
        }
    }

    pub fn set_wrap_width(&mut self, wrap_width: Option<f32>, cx: &mut ModelContext<Self>) -> bool {
        if wrap_width == self.wrap_width {
            return false;
        }

        self.wrap_width = wrap_width;
        self.rewrap(cx);
        true
    }

    fn rewrap(&mut self, cx: &mut ModelContext<Self>) {
        self.background_task.take();
        self.interpolated_edits.clear();
        self.pending_edits.clear();

        if let Some(wrap_width) = self.wrap_width {
            let mut new_snapshot = self.snapshot.clone();
            let font_cache = cx.font_cache().clone();
            let (font_id, font_size) = self.font;
            let task = cx.background().spawn(async move {
                let mut line_wrapper = font_cache.line_wrapper(font_id, font_size);
                let tab_snapshot = new_snapshot.tab_snapshot.clone();
                let range = TabPoint::zero()..tab_snapshot.max_point();
                let edits = new_snapshot
                    .update(
                        tab_snapshot,
                        &[TabEdit {
                            old: range.clone(),
                            new: range.clone(),
                        }],
                        wrap_width,
                        &mut line_wrapper,
                    )
                    .await;
                (new_snapshot, edits)
            });

            match cx
                .background()
                .block_with_timeout(Duration::from_millis(5), task)
            {
                Ok((snapshot, edits)) => {
                    self.snapshot = snapshot;
                    self.edits_since_sync = self.edits_since_sync.compose(&edits);
                    cx.notify();
                }
                Err(wrap_task) => {
                    self.background_task = Some(cx.spawn(|this, mut cx| async move {
                        let (snapshot, edits) = wrap_task.await;
                        this.update(&mut cx, |this, cx| {
                            this.snapshot = snapshot;
                            this.edits_since_sync = this
                                .edits_since_sync
                                .compose(mem::take(&mut this.interpolated_edits).invert())
                                .compose(&edits);
                            this.background_task = None;
                            this.flush_edits(cx);
                            cx.notify();
                        });
                    }));
                }
            }
        } else {
            let old_rows = self.snapshot.transforms.summary().output.lines.row + 1;
            self.snapshot.transforms = SumTree::new();
            let summary = self.snapshot.tab_snapshot.text_summary();
            if !summary.lines.is_zero() {
                self.snapshot
                    .transforms
                    .push(Transform::isomorphic(summary), &());
            }
            let new_rows = self.snapshot.transforms.summary().output.lines.row + 1;
            self.snapshot.interpolated = false;
            self.edits_since_sync = self.edits_since_sync.compose(&Patch::new(vec![WrapEdit {
                old: 0..old_rows,
                new: 0..new_rows,
            }]));
        }
    }

    fn flush_edits(&mut self, cx: &mut ModelContext<Self>) {
        if !self.snapshot.interpolated {
            let mut to_remove_len = 0;
            for (tab_snapshot, _) in &self.pending_edits {
                if tab_snapshot.version() <= self.snapshot.tab_snapshot.version() {
                    to_remove_len += 1;
                } else {
                    break;
                }
            }
            self.pending_edits.drain(..to_remove_len);
        }

        if self.pending_edits.is_empty() {
            return;
        }

        if let Some(wrap_width) = self.wrap_width {
            if self.background_task.is_none() {
                let pending_edits = self.pending_edits.clone();
                let mut snapshot = self.snapshot.clone();
                let font_cache = cx.font_cache().clone();
                let (font_id, font_size) = self.font;
                let update_task = cx.background().spawn(async move {
                    let mut line_wrapper = font_cache.line_wrapper(font_id, font_size);

                    let mut edits = Patch::default();
                    for (tab_snapshot, tab_edits) in pending_edits {
                        let wrap_edits = snapshot
                            .update(tab_snapshot, &tab_edits, wrap_width, &mut line_wrapper)
                            .await;
                        edits = edits.compose(&wrap_edits);
                    }
                    (snapshot, edits)
                });

                match cx
                    .background()
                    .block_with_timeout(Duration::from_millis(1), update_task)
                {
                    Ok((snapshot, output_edits)) => {
                        self.snapshot = snapshot;
                        self.edits_since_sync = self.edits_since_sync.compose(&output_edits);
                    }
                    Err(update_task) => {
                        self.background_task = Some(cx.spawn(|this, mut cx| async move {
                            let (snapshot, edits) = update_task.await;
                            this.update(&mut cx, |this, cx| {
                                this.snapshot = snapshot;
                                this.edits_since_sync = this
                                    .edits_since_sync
                                    .compose(mem::take(&mut this.interpolated_edits).invert())
                                    .compose(&edits);
                                this.background_task = None;
                                this.flush_edits(cx);
                                cx.notify();
                            });
                        }));
                    }
                }
            }
        }

        let was_interpolated = self.snapshot.interpolated;
        let mut to_remove_len = 0;
        for (tab_snapshot, edits) in &self.pending_edits {
            if tab_snapshot.version() <= self.snapshot.tab_snapshot.version() {
                to_remove_len += 1;
            } else {
                let interpolated_edits = self.snapshot.interpolate(tab_snapshot.clone(), &edits);
                self.edits_since_sync = self.edits_since_sync.compose(&interpolated_edits);
                self.interpolated_edits = self.interpolated_edits.compose(&interpolated_edits);
            }
        }

        if !was_interpolated {
            self.pending_edits.drain(..to_remove_len);
        }
    }
}

impl WrapSnapshot {
    fn new(tab_snapshot: TabSnapshot) -> Self {
        let mut transforms = SumTree::new();
        let extent = tab_snapshot.text_summary();
        if !extent.lines.is_zero() {
            transforms.push(Transform::isomorphic(extent), &());
        }
        Self {
            transforms,
            tab_snapshot,
            interpolated: true,
        }
    }

    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        self.tab_snapshot.buffer_snapshot()
    }

    fn interpolate(&mut self, new_tab_snapshot: TabSnapshot, tab_edits: &[TabEdit]) -> Patch<u32> {
        let mut new_transforms;
        if tab_edits.is_empty() {
            new_transforms = self.transforms.clone();
        } else {
            let mut old_cursor = self.transforms.cursor::<TabPoint>();

            let mut tab_edits_iter = tab_edits.iter().peekable();
            new_transforms =
                old_cursor.slice(&tab_edits_iter.peek().unwrap().old.start, Bias::Right, &());

            while let Some(edit) = tab_edits_iter.next() {
                if edit.new.start > TabPoint::from(new_transforms.summary().input.lines) {
                    let summary = new_tab_snapshot.text_summary_for_range(
                        TabPoint::from(new_transforms.summary().input.lines)..edit.new.start,
                    );
                    new_transforms.push_or_extend(Transform::isomorphic(summary));
                }

                if !edit.new.is_empty() {
                    new_transforms.push_or_extend(Transform::isomorphic(
                        new_tab_snapshot.text_summary_for_range(edit.new.clone()),
                    ));
                }

                old_cursor.seek_forward(&edit.old.end, Bias::Right, &());
                if let Some(next_edit) = tab_edits_iter.peek() {
                    if next_edit.old.start > old_cursor.end(&()) {
                        if old_cursor.end(&()) > edit.old.end {
                            let summary = self
                                .tab_snapshot
                                .text_summary_for_range(edit.old.end..old_cursor.end(&()));
                            new_transforms.push_or_extend(Transform::isomorphic(summary));
                        }

                        old_cursor.next(&());
                        new_transforms.push_tree(
                            old_cursor.slice(&next_edit.old.start, Bias::Right, &()),
                            &(),
                        );
                    }
                } else {
                    if old_cursor.end(&()) > edit.old.end {
                        let summary = self
                            .tab_snapshot
                            .text_summary_for_range(edit.old.end..old_cursor.end(&()));
                        new_transforms.push_or_extend(Transform::isomorphic(summary));
                    }
                    old_cursor.next(&());
                    new_transforms.push_tree(old_cursor.suffix(&()), &());
                }
            }
        }

        let old_snapshot = mem::replace(
            self,
            WrapSnapshot {
                tab_snapshot: new_tab_snapshot,
                transforms: new_transforms,
                interpolated: true,
            },
        );
        self.check_invariants();
        old_snapshot.compute_edits(tab_edits, self)
    }

    async fn update(
        &mut self,
        new_tab_snapshot: TabSnapshot,
        tab_edits: &[TabEdit],
        wrap_width: f32,
        line_wrapper: &mut LineWrapper,
    ) -> Patch<u32> {
        #[derive(Debug)]
        struct RowEdit {
            old_rows: Range<u32>,
            new_rows: Range<u32>,
        }

        let mut tab_edits_iter = tab_edits.into_iter().peekable();
        let mut row_edits = Vec::new();
        while let Some(edit) = tab_edits_iter.next() {
            let mut row_edit = RowEdit {
                old_rows: edit.old.start.row()..edit.old.end.row() + 1,
                new_rows: edit.new.start.row()..edit.new.end.row() + 1,
            };

            while let Some(next_edit) = tab_edits_iter.peek() {
                if next_edit.old.start.row() <= row_edit.old_rows.end {
                    row_edit.old_rows.end = next_edit.old.end.row() + 1;
                    row_edit.new_rows.end = next_edit.new.end.row() + 1;
                    tab_edits_iter.next();
                } else {
                    break;
                }
            }

            row_edits.push(row_edit);
        }

        let mut new_transforms;
        if row_edits.is_empty() {
            new_transforms = self.transforms.clone();
        } else {
            let mut row_edits = row_edits.into_iter().peekable();
            let mut old_cursor = self.transforms.cursor::<TabPoint>();

            new_transforms = old_cursor.slice(
                &TabPoint::new(row_edits.peek().unwrap().old_rows.start, 0),
                Bias::Right,
                &(),
            );

            while let Some(edit) = row_edits.next() {
                if edit.new_rows.start > new_transforms.summary().input.lines.row {
                    let summary = new_tab_snapshot.text_summary_for_range(
                        TabPoint(new_transforms.summary().input.lines)
                            ..TabPoint::new(edit.new_rows.start, 0),
                    );
                    new_transforms.push_or_extend(Transform::isomorphic(summary));
                }

                let mut line = String::new();
                let mut remaining = None;
                let mut chunks = new_tab_snapshot.chunks(
                    TabPoint::new(edit.new_rows.start, 0)..new_tab_snapshot.max_point(),
                    false,
                );
                let mut edit_transforms = Vec::<Transform>::new();
                for _ in edit.new_rows.start..edit.new_rows.end {
                    while let Some(chunk) =
                        remaining.take().or_else(|| chunks.next().map(|c| c.text))
                    {
                        if let Some(ix) = chunk.find('\n') {
                            line.push_str(&chunk[..ix + 1]);
                            remaining = Some(&chunk[ix + 1..]);
                            break;
                        } else {
                            line.push_str(chunk)
                        }
                    }

                    if line.is_empty() {
                        break;
                    }

                    let mut prev_boundary_ix = 0;
                    for boundary in line_wrapper.wrap_line(&line, wrap_width) {
                        let wrapped = &line[prev_boundary_ix..boundary.ix];
                        push_isomorphic(&mut edit_transforms, TextSummary::from(wrapped));
                        edit_transforms.push(Transform::wrap(boundary.next_indent));
                        prev_boundary_ix = boundary.ix;
                    }

                    if prev_boundary_ix < line.len() {
                        push_isomorphic(
                            &mut edit_transforms,
                            TextSummary::from(&line[prev_boundary_ix..]),
                        );
                    }

                    line.clear();
                    yield_now().await;
                }

                let mut edit_transforms = edit_transforms.into_iter();
                if let Some(transform) = edit_transforms.next() {
                    new_transforms.push_or_extend(transform);
                }
                new_transforms.extend(edit_transforms, &());

                old_cursor.seek_forward(&TabPoint::new(edit.old_rows.end, 0), Bias::Right, &());
                if let Some(next_edit) = row_edits.peek() {
                    if next_edit.old_rows.start > old_cursor.end(&()).row() {
                        if old_cursor.end(&()) > TabPoint::new(edit.old_rows.end, 0) {
                            let summary = self.tab_snapshot.text_summary_for_range(
                                TabPoint::new(edit.old_rows.end, 0)..old_cursor.end(&()),
                            );
                            new_transforms.push_or_extend(Transform::isomorphic(summary));
                        }
                        old_cursor.next(&());
                        new_transforms.push_tree(
                            old_cursor.slice(
                                &TabPoint::new(next_edit.old_rows.start, 0),
                                Bias::Right,
                                &(),
                            ),
                            &(),
                        );
                    }
                } else {
                    if old_cursor.end(&()) > TabPoint::new(edit.old_rows.end, 0) {
                        let summary = self.tab_snapshot.text_summary_for_range(
                            TabPoint::new(edit.old_rows.end, 0)..old_cursor.end(&()),
                        );
                        new_transforms.push_or_extend(Transform::isomorphic(summary));
                    }
                    old_cursor.next(&());
                    new_transforms.push_tree(old_cursor.suffix(&()), &());
                }
            }
        }

        let old_snapshot = mem::replace(
            self,
            WrapSnapshot {
                tab_snapshot: new_tab_snapshot,
                transforms: new_transforms,
                interpolated: false,
            },
        );
        self.check_invariants();
        old_snapshot.compute_edits(tab_edits, self)
    }

    fn compute_edits(&self, tab_edits: &[TabEdit], new_snapshot: &WrapSnapshot) -> Patch<u32> {
        let mut wrap_edits = Vec::new();
        let mut old_cursor = self.transforms.cursor::<TransformSummary>();
        let mut new_cursor = new_snapshot.transforms.cursor::<TransformSummary>();
        for mut tab_edit in tab_edits.iter().cloned() {
            tab_edit.old.start.0.column = 0;
            tab_edit.old.end.0 += Point::new(1, 0);
            tab_edit.new.start.0.column = 0;
            tab_edit.new.end.0 += Point::new(1, 0);

            old_cursor.seek(&tab_edit.old.start, Bias::Right, &());
            let mut old_start = old_cursor.start().output.lines;
            old_start += tab_edit.old.start.0 - old_cursor.start().input.lines;

            old_cursor.seek(&tab_edit.old.end, Bias::Right, &());
            let mut old_end = old_cursor.start().output.lines;
            old_end += tab_edit.old.end.0 - old_cursor.start().input.lines;

            new_cursor.seek(&tab_edit.new.start, Bias::Right, &());
            let mut new_start = new_cursor.start().output.lines;
            new_start += tab_edit.new.start.0 - new_cursor.start().input.lines;

            new_cursor.seek(&tab_edit.new.end, Bias::Right, &());
            let mut new_end = new_cursor.start().output.lines;
            new_end += tab_edit.new.end.0 - new_cursor.start().input.lines;

            wrap_edits.push(WrapEdit {
                old: old_start.row..old_end.row,
                new: new_start.row..new_end.row,
            });
        }

        consolidate_wrap_edits(&mut wrap_edits);
        Patch::new(wrap_edits)
    }

    pub fn text_chunks(&self, wrap_row: u32) -> impl Iterator<Item = &str> {
        self.chunks(wrap_row..self.max_point().row() + 1, false)
            .map(|h| h.text)
    }

    pub fn chunks<'a>(&'a self, rows: Range<u32>, language_aware: bool) -> WrapChunks<'a> {
        let output_start = WrapPoint::new(rows.start, 0);
        let output_end = WrapPoint::new(rows.end, 0);
        let mut transforms = self.transforms.cursor::<(WrapPoint, TabPoint)>();
        transforms.seek(&output_start, Bias::Right, &());
        let mut input_start = TabPoint(transforms.start().1 .0);
        if transforms.item().map_or(false, |t| t.is_isomorphic()) {
            input_start.0 += output_start.0 - transforms.start().0 .0;
        }
        let input_end = self
            .to_tab_point(output_end)
            .min(self.tab_snapshot.max_point());
        WrapChunks {
            input_chunks: self
                .tab_snapshot
                .chunks(input_start..input_end, language_aware),
            input_chunk: Default::default(),
            output_position: output_start,
            max_output_row: rows.end,
            transforms,
        }
    }

    pub fn text_summary(&self) -> TextSummary {
        self.transforms.summary().output
    }

    pub fn max_point(&self) -> WrapPoint {
        WrapPoint(self.transforms.summary().output.lines)
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let mut len = 0;
        for chunk in self.text_chunks(row) {
            if let Some(newline_ix) = chunk.find('\n') {
                len += newline_ix;
                break;
            } else {
                len += chunk.len();
            }
        }
        len as u32
    }

    pub fn soft_wrap_indent(&self, row: u32) -> Option<u32> {
        let mut cursor = self.transforms.cursor::<WrapPoint>();
        cursor.seek(&WrapPoint::new(row + 1, 0), Bias::Right, &());
        cursor.item().and_then(|transform| {
            if transform.is_isomorphic() {
                None
            } else {
                Some(transform.summary.output.lines.column)
            }
        })
    }

    pub fn longest_row(&self) -> u32 {
        self.transforms.summary().output.longest_row
    }

    pub fn buffer_rows(&self, start_row: u32) -> WrapBufferRows {
        let mut transforms = self.transforms.cursor::<(WrapPoint, TabPoint)>();
        transforms.seek(&WrapPoint::new(start_row, 0), Bias::Left, &());
        let mut input_row = transforms.start().1.row();
        if transforms.item().map_or(false, |t| t.is_isomorphic()) {
            input_row += start_row - transforms.start().0.row();
        }
        let soft_wrapped = transforms.item().map_or(false, |t| !t.is_isomorphic());
        let mut input_buffer_rows = self.tab_snapshot.buffer_rows(input_row);
        let input_buffer_row = input_buffer_rows.next().unwrap();
        WrapBufferRows {
            transforms,
            input_buffer_row,
            input_buffer_rows,
            output_row: start_row,
            soft_wrapped,
            max_output_row: self.max_point().row(),
        }
    }

    pub fn to_tab_point(&self, point: WrapPoint) -> TabPoint {
        let mut cursor = self.transforms.cursor::<(WrapPoint, TabPoint)>();
        cursor.seek(&point, Bias::Right, &());
        let mut tab_point = cursor.start().1 .0;
        if cursor.item().map_or(false, |t| t.is_isomorphic()) {
            tab_point += point.0 - cursor.start().0 .0;
        }
        TabPoint(tab_point)
    }

    pub fn to_point(&self, point: WrapPoint, bias: Bias) -> Point {
        self.tab_snapshot.to_point(self.to_tab_point(point), bias)
    }

    pub fn from_point(&self, point: Point, bias: Bias) -> WrapPoint {
        self.from_tab_point(self.tab_snapshot.from_point(point, bias))
    }

    pub fn from_tab_point(&self, point: TabPoint) -> WrapPoint {
        let mut cursor = self.transforms.cursor::<(TabPoint, WrapPoint)>();
        cursor.seek(&point, Bias::Right, &());
        WrapPoint(cursor.start().1 .0 + (point.0 - cursor.start().0 .0))
    }

    pub fn clip_point(&self, mut point: WrapPoint, bias: Bias) -> WrapPoint {
        if bias == Bias::Left {
            let mut cursor = self.transforms.cursor::<WrapPoint>();
            cursor.seek(&point, Bias::Right, &());
            if cursor.item().map_or(false, |t| !t.is_isomorphic()) {
                point = *cursor.start();
                *point.column_mut() -= 1;
            }
        }

        self.from_tab_point(self.tab_snapshot.clip_point(self.to_tab_point(point), bias))
    }

    pub fn prev_row_boundary(&self, mut point: WrapPoint) -> u32 {
        if self.transforms.is_empty() {
            return 0;
        }

        *point.column_mut() = 0;

        let mut cursor = self.transforms.cursor::<(WrapPoint, TabPoint)>();
        cursor.seek(&point, Bias::Right, &());
        if cursor.item().is_none() {
            cursor.prev(&());
        }

        while let Some(transform) = cursor.item() {
            if transform.is_isomorphic() && cursor.start().1.column() == 0 {
                return cmp::min(cursor.end(&()).0.row(), point.row());
            } else {
                cursor.prev(&());
            }
        }

        unreachable!()
    }

    pub fn next_row_boundary(&self, mut point: WrapPoint) -> Option<u32> {
        point.0 += Point::new(1, 0);

        let mut cursor = self.transforms.cursor::<(WrapPoint, TabPoint)>();
        cursor.seek(&point, Bias::Right, &());
        while let Some(transform) = cursor.item() {
            if transform.is_isomorphic() && cursor.start().1.column() == 0 {
                return Some(cmp::max(cursor.start().0.row(), point.row()));
            } else {
                cursor.next(&());
            }
        }

        None
    }

    fn check_invariants(&self) {
        #[cfg(test)]
        {
            assert_eq!(
                TabPoint::from(self.transforms.summary().input.lines),
                self.tab_snapshot.max_point()
            );

            {
                let mut transforms = self.transforms.cursor::<()>().peekable();
                while let Some(transform) = transforms.next() {
                    if let Some(next_transform) = transforms.peek() {
                        assert!(transform.is_isomorphic() != next_transform.is_isomorphic());
                    }
                }
            }

            let input_buffer_rows = self.buffer_snapshot().buffer_rows(0).collect::<Vec<_>>();
            let mut expected_buffer_rows = Vec::new();
            let mut prev_tab_row = 0;
            for display_row in 0..=self.max_point().row() {
                let tab_point = self.to_tab_point(WrapPoint::new(display_row, 0));
                if tab_point.row() == prev_tab_row && display_row != 0 {
                    expected_buffer_rows.push(None);
                } else {
                    let fold_point = self.tab_snapshot.to_fold_point(tab_point, Bias::Left).0;
                    let buffer_point = fold_point.to_buffer_point(&self.tab_snapshot.fold_snapshot);
                    expected_buffer_rows.push(input_buffer_rows[buffer_point.row as usize]);
                    prev_tab_row = tab_point.row();
                }
            }

            for start_display_row in 0..expected_buffer_rows.len() {
                assert_eq!(
                    self.buffer_rows(start_display_row as u32)
                        .collect::<Vec<_>>(),
                    &expected_buffer_rows[start_display_row..],
                    "invalid buffer_rows({}..)",
                    start_display_row
                );
            }
        }
    }
}

impl<'a> Iterator for WrapChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_position.row() >= self.max_output_row {
            return None;
        }

        let transform = self.transforms.item()?;
        if let Some(display_text) = transform.display_text {
            let mut start_ix = 0;
            let mut end_ix = display_text.len();
            let mut summary = transform.summary.output.lines;

            if self.output_position > self.transforms.start().0 {
                // Exclude newline starting prior to the desired row.
                start_ix = 1;
                summary.row = 0;
            } else if self.output_position.row() + 1 >= self.max_output_row {
                // Exclude soft indentation ending after the desired row.
                end_ix = 1;
                summary.column = 0;
            }

            self.output_position.0 += summary;
            self.transforms.next(&());
            return Some(Chunk {
                text: &display_text[start_ix..end_ix],
                ..self.input_chunk
            });
        }

        if self.input_chunk.text.is_empty() {
            self.input_chunk = self.input_chunks.next().unwrap();
        }

        let mut input_len = 0;
        let transform_end = self.transforms.end(&()).0;
        for c in self.input_chunk.text.chars() {
            let char_len = c.len_utf8();
            input_len += char_len;
            if c == '\n' {
                *self.output_position.row_mut() += 1;
                *self.output_position.column_mut() = 0;
            } else {
                *self.output_position.column_mut() += char_len as u32;
            }

            if self.output_position >= transform_end {
                self.transforms.next(&());
                break;
            }
        }

        let (prefix, suffix) = self.input_chunk.text.split_at(input_len);
        self.input_chunk.text = suffix;
        Some(Chunk {
            text: prefix,
            ..self.input_chunk
        })
    }
}

impl<'a> Iterator for WrapBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_row > self.max_output_row {
            return None;
        }

        let buffer_row = self.input_buffer_row;
        let soft_wrapped = self.soft_wrapped;

        self.output_row += 1;
        self.transforms
            .seek_forward(&WrapPoint::new(self.output_row, 0), Bias::Left, &());
        if self.transforms.item().map_or(false, |t| t.is_isomorphic()) {
            self.input_buffer_row = self.input_buffer_rows.next().unwrap();
            self.soft_wrapped = false;
        } else {
            self.soft_wrapped = true;
        }

        Some(if soft_wrapped { None } else { buffer_row })
    }
}

impl Transform {
    fn isomorphic(summary: TextSummary) -> Self {
        #[cfg(test)]
        assert!(!summary.lines.is_zero());

        Self {
            summary: TransformSummary {
                input: summary.clone(),
                output: summary,
            },
            display_text: None,
        }
    }

    fn wrap(indent: u32) -> Self {
        lazy_static! {
            static ref WRAP_TEXT: String = {
                let mut wrap_text = String::new();
                wrap_text.push('\n');
                wrap_text.extend((0..LineWrapper::MAX_INDENT as usize).map(|_| ' '));
                wrap_text
            };
        }

        Self {
            summary: TransformSummary {
                input: TextSummary::default(),
                output: TextSummary {
                    lines: Point::new(1, indent),
                    first_line_chars: 0,
                    last_line_chars: indent,
                    longest_row: 1,
                    longest_row_chars: indent,
                },
            },
            display_text: Some(&WRAP_TEXT[..1 + indent as usize]),
        }
    }

    fn is_isomorphic(&self) -> bool {
        self.display_text.is_none()
    }
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self) -> Self::Summary {
        self.summary.clone()
    }
}

fn push_isomorphic(transforms: &mut Vec<Transform>, summary: TextSummary) {
    if let Some(last_transform) = transforms.last_mut() {
        if last_transform.is_isomorphic() {
            last_transform.summary.input += &summary;
            last_transform.summary.output += &summary;
            return;
        }
    }
    transforms.push(Transform::isomorphic(summary));
}

trait SumTreeExt {
    fn push_or_extend(&mut self, transform: Transform);
}

impl SumTreeExt for SumTree<Transform> {
    fn push_or_extend(&mut self, transform: Transform) {
        let mut transform = Some(transform);
        self.update_last(
            |last_transform| {
                if last_transform.is_isomorphic() && transform.as_ref().unwrap().is_isomorphic() {
                    let transform = transform.take().unwrap();
                    last_transform.summary.input += &transform.summary.input;
                    last_transform.summary.output += &transform.summary.output;
                }
            },
            &(),
        );

        if let Some(transform) = transform {
            self.push(transform, &());
        }
    }
}

impl WrapPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(super::Point::new(row, column))
    }

    pub fn row(self) -> u32 {
        self.0.row
    }

    pub fn row_mut(&mut self) -> &mut u32 {
        &mut self.0.row
    }

    pub fn column(&self) -> u32 {
        self.0.column
    }

    pub fn column_mut(&mut self) -> &mut u32 {
        &mut self.0.column
    }
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.input += &other.input;
        self.output += &other.output;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for TabPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.input.lines;
    }
}

impl<'a> sum_tree::SeekTarget<'a, TransformSummary, TransformSummary> for TabPoint {
    fn cmp(&self, cursor_location: &TransformSummary, _: &()) -> std::cmp::Ordering {
        Ord::cmp(&self.0, &cursor_location.input.lines)
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for WrapPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.output.lines;
    }
}

fn consolidate_wrap_edits(edits: &mut Vec<WrapEdit>) {
    let mut i = 1;
    while i < edits.len() {
        let edit = edits[i].clone();
        let prev_edit = &mut edits[i - 1];
        if prev_edit.old.end >= edit.old.start {
            prev_edit.old.end = edit.old.end;
            prev_edit.new.end = edit.new.end;
            edits.remove(i);
            continue;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::{fold_map::FoldMap, tab_map::TabMap},
        MultiBuffer,
    };
    use gpui::test::observe;
    use language::RandomCharIter;
    use rand::prelude::*;
    use smol::stream::StreamExt;
    use std::{cmp, env};
    use text::Rope;

    #[gpui::test(iterations = 100)]
    async fn test_random_wraps(mut cx: gpui::TestAppContext, mut rng: StdRng) {
        cx.foreground().set_block_on_ticks(0..=50);
        cx.foreground().forbid_parking();
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().fonts();
        let mut wrap_width = if rng.gen_bool(0.1) {
            None
        } else {
            Some(rng.gen_range(0.0..=1000.0))
        };
        let tab_size = rng.gen_range(1..=4);
        let family_id = font_cache.load_family(&["Helvetica"]).unwrap();
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;

        log::info!("Tab size: {}", tab_size);
        log::info!("Wrap width: {:?}", wrap_width);

        let buffer = cx.update(|cx| {
            if rng.gen() {
                MultiBuffer::build_random(&mut rng, cx)
            } else {
                let len = rng.gen_range(0..10);
                let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
                MultiBuffer::build_simple(&text, cx)
            }
        });
        let mut buffer_snapshot = buffer.read_with(&cx, |buffer, cx| buffer.snapshot(cx));
        let (mut fold_map, folds_snapshot) = FoldMap::new(buffer_snapshot.clone());
        let (tab_map, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), tab_size);
        log::info!("Unwrapped text (no folds): {:?}", buffer_snapshot.text());
        log::info!(
            "Unwrapped text (unexpanded tabs): {:?}",
            folds_snapshot.text()
        );
        log::info!("Unwrapped text (expanded tabs): {:?}", tabs_snapshot.text());

        let mut line_wrapper = LineWrapper::new(font_id, font_size, font_system);
        let unwrapped_text = tabs_snapshot.text();
        let expected_text = wrap_text(&unwrapped_text, wrap_width, &mut line_wrapper);

        let (wrap_map, _) =
            cx.update(|cx| WrapMap::new(tabs_snapshot.clone(), font_id, font_size, wrap_width, cx));
        let mut notifications = observe(&wrap_map, &mut cx);

        if wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) {
            notifications.next().await.unwrap();
        }

        let (initial_snapshot, _) = wrap_map.update(&mut cx, |map, cx| {
            assert!(!map.is_rewrapping());
            map.sync(tabs_snapshot.clone(), Vec::new(), cx)
        });

        let actual_text = initial_snapshot.text();
        assert_eq!(
            actual_text, expected_text,
            "unwrapped text is: {:?}",
            unwrapped_text
        );
        log::info!("Wrapped text: {:?}", actual_text);

        let mut edits = Vec::new();
        for _i in 0..operations {
            log::info!("{} ==============================================", _i);

            let mut buffer_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=19 => {
                    wrap_width = if rng.gen_bool(0.2) {
                        None
                    } else {
                        Some(rng.gen_range(0.0..=1000.0))
                    };
                    log::info!("Setting wrap width to {:?}", wrap_width);
                    wrap_map.update(&mut cx, |map, cx| map.set_wrap_width(wrap_width, cx));
                }
                20..=39 => {
                    for (folds_snapshot, fold_edits) in fold_map.randomly_mutate(&mut rng) {
                        let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
                        let (mut snapshot, wrap_edits) = wrap_map
                            .update(&mut cx, |map, cx| map.sync(tabs_snapshot, tab_edits, cx));
                        snapshot.check_invariants();
                        snapshot.verify_chunks(&mut rng);
                        edits.push((snapshot, wrap_edits));
                    }
                }
                _ => {
                    buffer.update(&mut cx, |buffer, cx| {
                        let subscription = buffer.subscribe();
                        let edit_count = rng.gen_range(1..=5);
                        buffer.randomly_edit(&mut rng, edit_count, cx);
                        buffer_snapshot = buffer.snapshot(cx);
                        buffer_edits.extend(subscription.consume());
                    });
                }
            }

            log::info!("Unwrapped text (no folds): {:?}", buffer_snapshot.text());
            let (folds_snapshot, fold_edits) = fold_map.read(buffer_snapshot.clone(), buffer_edits);
            log::info!(
                "Unwrapped text (unexpanded tabs): {:?}",
                folds_snapshot.text()
            );
            let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
            log::info!("Unwrapped text (expanded tabs): {:?}", tabs_snapshot.text());

            let unwrapped_text = tabs_snapshot.text();
            let expected_text = wrap_text(&unwrapped_text, wrap_width, &mut line_wrapper);
            let (mut snapshot, wrap_edits) = wrap_map.update(&mut cx, |map, cx| {
                map.sync(tabs_snapshot.clone(), tab_edits, cx)
            });
            snapshot.check_invariants();
            snapshot.verify_chunks(&mut rng);
            edits.push((snapshot, wrap_edits));

            if wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) && rng.gen_bool(0.4) {
                log::info!("Waiting for wrapping to finish");
                while wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) {
                    notifications.next().await.unwrap();
                }
                wrap_map.read_with(&cx, |map, _| assert!(map.pending_edits.is_empty()));
            }

            if !wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) {
                let (mut wrapped_snapshot, wrap_edits) =
                    wrap_map.update(&mut cx, |map, cx| map.sync(tabs_snapshot, Vec::new(), cx));
                let actual_text = wrapped_snapshot.text();
                let actual_longest_row = wrapped_snapshot.longest_row();
                log::info!("Wrapping finished: {:?}", actual_text);
                wrapped_snapshot.check_invariants();
                wrapped_snapshot.verify_chunks(&mut rng);
                edits.push((wrapped_snapshot.clone(), wrap_edits));
                assert_eq!(
                    actual_text, expected_text,
                    "unwrapped text is: {:?}",
                    unwrapped_text
                );

                let mut summary = TextSummary::default();
                for (ix, item) in wrapped_snapshot
                    .transforms
                    .items(&())
                    .into_iter()
                    .enumerate()
                {
                    summary += &item.summary.output;
                    log::info!("{} summary: {:?}", ix, item.summary.output,);
                }

                if tab_size == 1
                    || !wrapped_snapshot
                        .tab_snapshot
                        .fold_snapshot
                        .text()
                        .contains('\t')
                {
                    let mut expected_longest_rows = Vec::new();
                    let mut longest_line_len = -1;
                    for (row, line) in expected_text.split('\n').enumerate() {
                        let line_char_count = line.chars().count() as isize;
                        if line_char_count > longest_line_len {
                            expected_longest_rows.clear();
                            longest_line_len = line_char_count;
                        }
                        if line_char_count >= longest_line_len {
                            expected_longest_rows.push(row as u32);
                        }
                    }

                    assert!(
                        expected_longest_rows.contains(&actual_longest_row),
                        "incorrect longest row {}. expected {:?} with length {}",
                        actual_longest_row,
                        expected_longest_rows,
                        longest_line_len,
                    )
                }
            }
        }

        let mut initial_text = Rope::from(initial_snapshot.text().as_str());
        for (snapshot, patch) in edits {
            let snapshot_text = Rope::from(snapshot.text().as_str());
            for edit in &patch {
                let old_start = initial_text.point_to_offset(Point::new(edit.new.start, 0));
                let old_end = initial_text.point_to_offset(cmp::min(
                    Point::new(edit.new.start + edit.old.len() as u32, 0),
                    initial_text.max_point(),
                ));
                let new_start = snapshot_text.point_to_offset(Point::new(edit.new.start, 0));
                let new_end = snapshot_text.point_to_offset(cmp::min(
                    Point::new(edit.new.end, 0),
                    snapshot_text.max_point(),
                ));
                let new_text = snapshot_text
                    .chunks_in_range(new_start..new_end)
                    .collect::<String>();

                initial_text.replace(old_start..old_end, &new_text);
            }
            assert_eq!(initial_text.to_string(), snapshot_text.to_string());
        }

        if wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) {
            log::info!("Waiting for wrapping to finish");
            while wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) {
                notifications.next().await.unwrap();
            }
        }
        wrap_map.read_with(&cx, |map, _| assert!(map.pending_edits.is_empty()));
    }

    fn wrap_text(
        unwrapped_text: &str,
        wrap_width: Option<f32>,
        line_wrapper: &mut LineWrapper,
    ) -> String {
        if let Some(wrap_width) = wrap_width {
            let mut wrapped_text = String::new();
            for (row, line) in unwrapped_text.split('\n').enumerate() {
                if row > 0 {
                    wrapped_text.push('\n')
                }

                let mut prev_ix = 0;
                for boundary in line_wrapper.wrap_line(line, wrap_width) {
                    wrapped_text.push_str(&line[prev_ix..boundary.ix]);
                    wrapped_text.push('\n');
                    wrapped_text.push_str(&" ".repeat(boundary.next_indent as usize));
                    prev_ix = boundary.ix;
                }
                wrapped_text.push_str(&line[prev_ix..]);
            }
            wrapped_text
        } else {
            unwrapped_text.to_string()
        }
    }

    impl WrapSnapshot {
        pub fn text(&self) -> String {
            self.text_chunks(0).collect()
        }

        fn verify_chunks(&mut self, rng: &mut impl Rng) {
            for _ in 0..5 {
                let mut end_row = rng.gen_range(0..=self.max_point().row());
                let start_row = rng.gen_range(0..=end_row);
                end_row += 1;

                let mut expected_text = self.text_chunks(start_row).collect::<String>();
                if expected_text.ends_with("\n") {
                    expected_text.push('\n');
                }
                let mut expected_text = expected_text
                    .lines()
                    .take((end_row - start_row) as usize)
                    .collect::<Vec<_>>()
                    .join("\n");
                if end_row <= self.max_point().row() {
                    expected_text.push('\n');
                }

                let actual_text = self
                    .chunks(start_row..end_row, true)
                    .map(|c| c.text)
                    .collect::<String>();
                assert_eq!(
                    expected_text,
                    actual_text,
                    "chunks != highlighted_chunks for rows {:?}",
                    start_row..end_row
                );
            }
        }
    }
}
