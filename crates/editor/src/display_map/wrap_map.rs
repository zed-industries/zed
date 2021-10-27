use super::{
    fold_map,
    tab_map::{self, Edit as TabEdit, Snapshot as TabSnapshot, TabPoint, TextSummary},
};
use gpui::{fonts::FontId, text_layout::LineWrapper, Entity, ModelContext, Task};
use language::{HighlightedChunk, Point};
use lazy_static::lazy_static;
use smol::future::yield_now;
use std::{collections::VecDeque, ops::Range, time::Duration};
use sum_tree::{Bias, Cursor, SumTree};

pub struct WrapMap {
    snapshot: Snapshot,
    pending_edits: VecDeque<(TabSnapshot, Vec<TabEdit>)>,
    wrap_width: Option<f32>,
    background_task: Option<Task<()>>,
    font: (FontId, f32),
}

impl Entity for WrapMap {
    type Event = ();
}

#[derive(Clone)]
pub struct Snapshot {
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
pub struct WrapPoint(super::Point);

pub struct Chunks<'a> {
    input_chunks: tab_map::Chunks<'a>,
    input_chunk: &'a str,
    output_position: WrapPoint,
    transforms: Cursor<'a, Transform, (WrapPoint, TabPoint)>,
}

pub struct HighlightedChunks<'a> {
    input_chunks: tab_map::HighlightedChunks<'a>,
    input_chunk: HighlightedChunk<'a>,
    output_position: WrapPoint,
    max_output_row: u32,
    transforms: Cursor<'a, Transform, (WrapPoint, TabPoint)>,
}

pub struct BufferRows<'a> {
    input_buffer_rows: fold_map::BufferRows<'a>,
    input_buffer_row: u32,
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
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let mut this = Self {
            font: (font_id, font_size),
            wrap_width: None,
            pending_edits: Default::default(),
            snapshot: Snapshot::new(tab_snapshot),
            background_task: None,
        };
        this.set_wrap_width(wrap_width, cx);

        this
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
    ) -> Snapshot {
        self.pending_edits.push_back((tab_snapshot, edits));
        self.flush_edits(cx);
        self.snapshot.clone()
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

        if let Some(wrap_width) = self.wrap_width {
            let mut new_snapshot = self.snapshot.clone();
            let font_cache = cx.font_cache().clone();
            let (font_id, font_size) = self.font;
            let task = cx.background().spawn(async move {
                let mut line_wrapper = font_cache.line_wrapper(font_id, font_size);
                let tab_snapshot = new_snapshot.tab_snapshot.clone();
                let range = TabPoint::zero()..tab_snapshot.max_point();
                new_snapshot
                    .update(
                        tab_snapshot,
                        &[TabEdit {
                            old_lines: range.clone(),
                            new_lines: range.clone(),
                        }],
                        wrap_width,
                        &mut line_wrapper,
                    )
                    .await;
                new_snapshot
            });

            match cx
                .background()
                .block_with_timeout(Duration::from_millis(5), task)
            {
                Ok(snapshot) => {
                    self.snapshot = snapshot;
                    cx.notify();
                }
                Err(wrap_task) => {
                    self.background_task = Some(cx.spawn(|this, mut cx| async move {
                        let snapshot = wrap_task.await;
                        this.update(&mut cx, |this, cx| {
                            this.snapshot = snapshot;
                            this.background_task = None;
                            this.flush_edits(cx);
                            cx.notify();
                        });
                    }));
                }
            }
        } else {
            self.snapshot.transforms = SumTree::new();
            let summary = self.snapshot.tab_snapshot.text_summary();
            if !summary.lines.is_zero() {
                self.snapshot
                    .transforms
                    .push(Transform::isomorphic(summary), &());
            }
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

                    for (tab_snapshot, edits) in pending_edits {
                        snapshot
                            .update(tab_snapshot, &edits, wrap_width, &mut line_wrapper)
                            .await;
                    }
                    snapshot
                });

                match cx
                    .background()
                    .block_with_timeout(Duration::from_millis(1), update_task)
                {
                    Ok(snapshot) => {
                        self.snapshot = snapshot;
                    }
                    Err(update_task) => {
                        self.background_task = Some(cx.spawn(|this, mut cx| async move {
                            let snapshot = update_task.await;
                            this.update(&mut cx, |this, cx| {
                                this.snapshot = snapshot;
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
                self.snapshot.interpolate(tab_snapshot.clone(), &edits);
            }
        }

        if !was_interpolated {
            self.pending_edits.drain(..to_remove_len);
        }
    }
}

impl Snapshot {
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

    fn interpolate(&mut self, new_tab_snapshot: TabSnapshot, edits: &[TabEdit]) {
        let mut new_transforms;
        if edits.is_empty() {
            new_transforms = self.transforms.clone();
        } else {
            let mut old_cursor = self.transforms.cursor::<TabPoint>();
            let mut edits = edits.into_iter().peekable();
            new_transforms =
                old_cursor.slice(&edits.peek().unwrap().old_lines.start, Bias::Right, &());

            while let Some(edit) = edits.next() {
                if edit.new_lines.start > TabPoint::from(new_transforms.summary().input.lines) {
                    let summary = new_tab_snapshot.text_summary_for_range(
                        TabPoint::from(new_transforms.summary().input.lines)..edit.new_lines.start,
                    );
                    new_transforms.push_or_extend(Transform::isomorphic(summary));
                }

                if !edit.new_lines.is_empty() {
                    new_transforms.push_or_extend(Transform::isomorphic(
                        new_tab_snapshot.text_summary_for_range(edit.new_lines.clone()),
                    ));
                }

                old_cursor.seek_forward(&edit.old_lines.end, Bias::Right, &());
                if let Some(next_edit) = edits.peek() {
                    if next_edit.old_lines.start > old_cursor.end(&()) {
                        if old_cursor.end(&()) > edit.old_lines.end {
                            let summary = self
                                .tab_snapshot
                                .text_summary_for_range(edit.old_lines.end..old_cursor.end(&()));
                            new_transforms.push_or_extend(Transform::isomorphic(summary));
                        }
                        old_cursor.next(&());
                        new_transforms.push_tree(
                            old_cursor.slice(&next_edit.old_lines.start, Bias::Right, &()),
                            &(),
                        );
                    }
                } else {
                    if old_cursor.end(&()) > edit.old_lines.end {
                        let summary = self
                            .tab_snapshot
                            .text_summary_for_range(edit.old_lines.end..old_cursor.end(&()));
                        new_transforms.push_or_extend(Transform::isomorphic(summary));
                    }
                    old_cursor.next(&());
                    new_transforms.push_tree(old_cursor.suffix(&()), &());
                }
            }
        }

        self.transforms = new_transforms;
        self.tab_snapshot = new_tab_snapshot;
        self.interpolated = true;
        self.check_invariants();
    }

    async fn update(
        &mut self,
        new_tab_snapshot: TabSnapshot,
        edits: &[TabEdit],
        wrap_width: f32,
        line_wrapper: &mut LineWrapper,
    ) {
        #[derive(Debug)]
        struct RowEdit {
            old_rows: Range<u32>,
            new_rows: Range<u32>,
        }

        let mut edits = edits.into_iter().peekable();
        let mut row_edits = Vec::new();
        while let Some(edit) = edits.next() {
            let mut row_edit = RowEdit {
                old_rows: edit.old_lines.start.row()..edit.old_lines.end.row() + 1,
                new_rows: edit.new_lines.start.row()..edit.new_lines.end.row() + 1,
            };

            while let Some(next_edit) = edits.peek() {
                if next_edit.old_lines.start.row() <= row_edit.old_rows.end {
                    row_edit.old_rows.end = next_edit.old_lines.end.row() + 1;
                    row_edit.new_rows.end = next_edit.new_lines.end.row() + 1;
                    edits.next();
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
                        TabPoint::new(new_transforms.summary().input.lines.row, 0)
                            ..TabPoint::new(edit.new_rows.start, 0),
                    );
                    new_transforms.push_or_extend(Transform::isomorphic(summary));
                }

                let mut line = String::new();
                let mut remaining = None;
                let mut chunks = new_tab_snapshot.chunks_at(TabPoint::new(edit.new_rows.start, 0));
                let mut edit_transforms = Vec::<Transform>::new();
                for _ in edit.new_rows.start..edit.new_rows.end {
                    while let Some(chunk) = remaining.take().or_else(|| chunks.next()) {
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

        self.transforms = new_transforms;
        self.tab_snapshot = new_tab_snapshot;
        self.interpolated = false;
        self.check_invariants();
    }

    pub fn chunks_at(&self, wrap_row: u32) -> Chunks {
        let point = WrapPoint::new(wrap_row, 0);
        let mut transforms = self.transforms.cursor::<(WrapPoint, TabPoint)>();
        transforms.seek(&point, Bias::Right, &());
        let mut input_position = TabPoint(transforms.start().1 .0);
        if transforms.item().map_or(false, |t| t.is_isomorphic()) {
            input_position.0 += point.0 - transforms.start().0 .0;
        }
        let input_chunks = self.tab_snapshot.chunks_at(input_position);
        Chunks {
            input_chunks,
            transforms,
            output_position: point,
            input_chunk: "",
        }
    }

    pub fn highlighted_chunks_for_rows(&mut self, rows: Range<u32>) -> HighlightedChunks {
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
        HighlightedChunks {
            input_chunks: self.tab_snapshot.highlighted_chunks(input_start..input_end),
            input_chunk: Default::default(),
            output_position: output_start,
            max_output_row: rows.end,
            transforms,
        }
    }

    pub fn max_point(&self) -> WrapPoint {
        self.to_wrap_point(self.tab_snapshot.max_point())
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let mut len = 0;
        for chunk in self.chunks_at(row) {
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

    pub fn buffer_rows(&self, start_row: u32) -> BufferRows {
        let mut transforms = self.transforms.cursor::<(WrapPoint, TabPoint)>();
        transforms.seek(&WrapPoint::new(start_row, 0), Bias::Left, &());
        let mut input_row = transforms.start().1.row();
        if transforms.item().map_or(false, |t| t.is_isomorphic()) {
            input_row += start_row - transforms.start().0.row();
        }
        let soft_wrapped = transforms.item().map_or(false, |t| !t.is_isomorphic());
        let mut input_buffer_rows = self.tab_snapshot.buffer_rows(input_row);
        let input_buffer_row = input_buffer_rows.next().unwrap();
        BufferRows {
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

    pub fn to_wrap_point(&self, point: TabPoint) -> WrapPoint {
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

        self.to_wrap_point(self.tab_snapshot.clip_point(self.to_tab_point(point), bias))
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

            let mut expected_buffer_rows = Vec::new();
            let mut buffer_row = 0;
            let mut prev_tab_row = 0;
            for display_row in 0..=self.max_point().row() {
                let tab_point = self.to_tab_point(WrapPoint::new(display_row, 0));
                let soft_wrapped;
                if tab_point.row() == prev_tab_row {
                    soft_wrapped = display_row != 0;
                } else {
                    let fold_point = self.tab_snapshot.to_fold_point(tab_point, Bias::Left).0;
                    let buffer_point = fold_point.to_buffer_point(&self.tab_snapshot.fold_snapshot);
                    buffer_row = buffer_point.row;
                    prev_tab_row = tab_point.row();
                    soft_wrapped = false;
                }
                expected_buffer_rows.push((buffer_row, soft_wrapped));
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

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let transform = self.transforms.item()?;
        if let Some(display_text) = transform.display_text {
            if self.output_position > self.transforms.start().0 {
                self.output_position.0.column += transform.summary.output.lines.column;
                self.transforms.next(&());
                return Some(&display_text[1..]);
            } else {
                self.output_position.0 += transform.summary.output.lines;
                self.transforms.next(&());
                return Some(display_text);
            }
        }

        if self.input_chunk.is_empty() {
            self.input_chunk = self.input_chunks.next().unwrap();
        }

        let mut input_len = 0;
        let transform_end = self.transforms.end(&()).0;
        for c in self.input_chunk.chars() {
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

        let (prefix, suffix) = self.input_chunk.split_at(input_len);
        self.input_chunk = suffix;
        Some(prefix)
    }
}

impl<'a> Iterator for HighlightedChunks<'a> {
    type Item = HighlightedChunk<'a>;

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
            return Some(HighlightedChunk {
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
        Some(HighlightedChunk {
            text: prefix,
            ..self.input_chunk
        })
    }
}

impl<'a> Iterator for BufferRows<'a> {
    type Item = (u32, bool);

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

        Some((buffer_row, soft_wrapped))
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

    #[cfg(test)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn row(self) -> u32 {
        self.0.row
    }

    pub fn column(self) -> u32 {
        self.0.column
    }

    pub fn row_mut(&mut self) -> &mut u32 {
        &mut self.0.row
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

impl<'a> sum_tree::Dimension<'a, TransformSummary> for WrapPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.output.lines;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::{fold_map::FoldMap, tab_map::TabMap},
        test::Observer,
    };
    use language::{Buffer, RandomCharIter};
    use rand::prelude::*;
    use std::env;

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

        let buffer = cx.add_model(|cx| {
            let len = rng.gen_range(0..10);
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            Buffer::new(0, text, cx)
        });
        let (mut fold_map, folds_snapshot) = cx.read(|cx| FoldMap::new(buffer.clone(), cx));
        let (tab_map, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), tab_size);
        log::info!(
            "Unwrapped text (no folds): {:?}",
            buffer.read_with(&cx, |buf, _| buf.text())
        );
        log::info!(
            "Unwrapped text (unexpanded tabs): {:?}",
            folds_snapshot.text()
        );
        log::info!("Unwrapped text (expanded tabs): {:?}", tabs_snapshot.text());

        let mut line_wrapper = LineWrapper::new(font_id, font_size, font_system);
        let unwrapped_text = tabs_snapshot.text();
        let expected_text = wrap_text(&unwrapped_text, wrap_width, &mut line_wrapper);

        let wrap_map = cx.add_model(|cx| {
            WrapMap::new(tabs_snapshot.clone(), font_id, font_size, wrap_width, cx)
        });
        let (_observer, notifications) = Observer::new(&wrap_map, &mut cx);

        if wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) {
            notifications.recv().await.unwrap();
        }

        let snapshot = wrap_map.update(&mut cx, |map, cx| map.sync(tabs_snapshot, Vec::new(), cx));
        let actual_text = snapshot.text();
        assert_eq!(
            actual_text, expected_text,
            "unwrapped text is: {:?}",
            unwrapped_text
        );
        log::info!("Wrapped text: {:?}", actual_text);

        for _i in 0..operations {
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
                    for (folds_snapshot, edits) in
                        cx.read(|cx| fold_map.randomly_mutate(&mut rng, cx))
                    {
                        let (tabs_snapshot, edits) = tab_map.sync(folds_snapshot, edits);
                        let mut snapshot =
                            wrap_map.update(&mut cx, |map, cx| map.sync(tabs_snapshot, edits, cx));
                        snapshot.check_invariants();
                        snapshot.verify_chunks(&mut rng);
                    }
                }
                _ => {
                    buffer.update(&mut cx, |buffer, _| buffer.randomly_mutate(&mut rng));
                }
            }

            log::info!(
                "Unwrapped text (no folds): {:?}",
                buffer.read_with(&cx, |buf, _| buf.text())
            );
            let (folds_snapshot, edits) = cx.read(|cx| fold_map.read(cx));
            log::info!(
                "Unwrapped text (unexpanded tabs): {:?}",
                folds_snapshot.text()
            );
            let (tabs_snapshot, edits) = tab_map.sync(folds_snapshot, edits);
            log::info!("Unwrapped text (expanded tabs): {:?}", tabs_snapshot.text());

            let unwrapped_text = tabs_snapshot.text();
            let expected_text = wrap_text(&unwrapped_text, wrap_width, &mut line_wrapper);
            let mut snapshot = wrap_map.update(&mut cx, |map, cx| {
                map.sync(tabs_snapshot.clone(), edits, cx)
            });
            snapshot.check_invariants();
            snapshot.verify_chunks(&mut rng);

            if wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) && rng.gen_bool(0.4) {
                log::info!("Waiting for wrapping to finish");
                while wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) {
                    notifications.recv().await.unwrap();
                }
            }

            if !wrap_map.read_with(&cx, |map, _| map.is_rewrapping()) {
                let mut wrapped_snapshot =
                    wrap_map.update(&mut cx, |map, cx| map.sync(tabs_snapshot, Vec::new(), cx));
                let actual_text = wrapped_snapshot.text();
                log::info!("Wrapping finished: {:?}", actual_text);
                wrapped_snapshot.check_invariants();
                wrapped_snapshot.verify_chunks(&mut rng);
                assert_eq!(
                    actual_text, expected_text,
                    "unwrapped text is: {:?}",
                    unwrapped_text
                );
            }
        }
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

    impl Snapshot {
        fn text(&self) -> String {
            self.chunks_at(0).collect()
        }

        fn verify_chunks(&mut self, rng: &mut impl Rng) {
            for _ in 0..5 {
                let mut end_row = rng.gen_range(0..=self.max_point().row());
                let start_row = rng.gen_range(0..=end_row);
                end_row += 1;

                let mut expected_text = self.chunks_at(start_row).collect::<String>();
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
                    .highlighted_chunks_for_rows(start_row..end_row)
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
