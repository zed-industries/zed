use super::{
    fold_map,
    tab_map::{
        self, Edit as InputEdit, OutputPoint as InputPoint, Snapshot as InputSnapshot, TextSummary,
    },
};
use crate::{
    editor::Point,
    settings::StyleId,
    sum_tree::{self, Cursor, SumTree},
    util::Bias,
    Settings,
};
use gpui::{fonts::FontId, AppContext, FontCache, FontSystem, Task};
use parking_lot::Mutex;
use postage::{
    prelude::{Sink, Stream},
    watch,
};
use smol::channel;
use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    sync::Arc,
    time::Duration,
};

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct OutputPoint(super::Point);

impl OutputPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(super::Point::new(row, column))
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
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

#[derive(Clone)]
pub struct Snapshot {
    transforms: SumTree<Transform>,
    input: InputSnapshot,
}

impl Snapshot {
    fn new(input: InputSnapshot) -> Self {
        Self {
            transforms: SumTree::from_item(
                Transform {
                    summary: TransformSummary {
                        input: input.text_summary(),
                        output: input.text_summary(),
                    },
                    display_text: None,
                },
                &(),
            ),
            input,
        }
    }

    fn interpolate(&mut self, new_snapshot: InputSnapshot, edits: &[InputEdit]) {
        if edits.is_empty() {
            return;
        }

        let mut new_transforms;
        {
            let mut old_cursor = self.transforms.cursor::<InputPoint, ()>();
            let mut edits = edits.into_iter().peekable();
            new_transforms =
                old_cursor.slice(&edits.peek().unwrap().old_lines.start, Bias::Right, &());

            while let Some(edit) = edits.next() {
                if edit.new_lines.start > InputPoint::from(new_transforms.summary().input.lines) {
                    let summary = new_snapshot.text_summary_for_range(
                        InputPoint::from(new_transforms.summary().input.lines)
                            ..edit.new_lines.start,
                    );
                    new_transforms.push_or_extend(Transform::isomorphic(summary));
                }

                new_transforms.push_or_extend(Transform::isomorphic(
                    new_snapshot.text_summary_for_range(edit.new_lines.clone()),
                ));

                old_cursor.seek_forward(&edit.old_lines.end, Bias::Right, &());
                if let Some(next_edit) = edits.peek() {
                    if next_edit.old_lines.start > old_cursor.seek_end(&()) {
                        if old_cursor.seek_end(&()) > edit.old_lines.end {
                            let summary = self.input.text_summary_for_range(
                                edit.old_lines.end..old_cursor.seek_end(&()),
                            );
                            new_transforms.push_or_extend(Transform::isomorphic(summary));
                        }
                        old_cursor.next(&());
                        new_transforms.push_tree(
                            old_cursor.slice(&next_edit.old_lines.start, Bias::Right, &()),
                            &(),
                        );
                    }
                } else {
                    if old_cursor.seek_end(&()) > edit.old_lines.end {
                        let summary = self
                            .input
                            .text_summary_for_range(edit.old_lines.end..old_cursor.seek_end(&()));
                        new_transforms.push_or_extend(Transform::isomorphic(summary));
                    }
                    old_cursor.next(&());
                    new_transforms.push_tree(old_cursor.suffix(&()), &());
                }
            }
        }

        self.transforms = new_transforms;
        self.input = new_snapshot;
    }

    pub fn chunks_at(&self, point: OutputPoint) -> Chunks {
        let mut transforms = self.transforms.cursor::<OutputPoint, InputPoint>();
        transforms.seek(&point, Bias::Right, &());
        let input_position =
            InputPoint(transforms.sum_start().0 + (point.0 - transforms.seek_start().0));
        let input_chunks = self.input.chunks_at(input_position);
        Chunks {
            input_chunks,
            transforms,
            input_position,
            input_chunk: "",
        }
    }

    pub fn highlighted_chunks_for_rows(&mut self, rows: Range<u32>) -> HighlightedChunks {
        let output_start = OutputPoint::new(rows.start, 0);
        let output_end = OutputPoint::new(rows.end, 0);
        let mut transforms = self.transforms.cursor::<OutputPoint, InputPoint>();
        transforms.seek(&output_start, Bias::Right, &());
        let input_start =
            InputPoint(transforms.sum_start().0 + (output_start.0 - transforms.seek_start().0));
        let input_end = self.to_input_point(output_end).min(self.input.max_point());
        HighlightedChunks {
            input_chunks: self.input.highlighted_chunks(input_start..input_end),
            input_chunk: "",
            style_id: StyleId::default(),
            output_position: output_start,
            max_output_row: rows.end,
            transforms,
        }
    }

    pub fn max_point(&self) -> OutputPoint {
        self.to_output_point(self.input.max_point())
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let mut len = 0;
        for chunk in self.chunks_at(OutputPoint::new(row, 0)) {
            if let Some(newline_ix) = chunk.find('\n') {
                len += newline_ix;
                break;
            } else {
                len += chunk.len();
            }
        }
        len as u32
    }

    pub fn longest_row(&self) -> u32 {
        self.transforms.summary().output.longest_row
    }

    pub fn buffer_rows(&self, start_row: u32) -> BufferRows {
        let mut transforms = self.transforms.cursor::<OutputPoint, InputPoint>();
        transforms.seek(&OutputPoint::new(start_row, 0), Bias::Right, &());
        let input_row = transforms.sum_start().row() + (start_row - transforms.seek_start().row());
        let mut input_buffer_rows = self.input.buffer_rows(input_row);
        let input_buffer_row = input_buffer_rows.next().unwrap();
        BufferRows {
            transforms,
            input_buffer_row,
            input_buffer_rows,
            output_row: start_row,
            max_output_row: self.max_point().row(),
        }
    }

    pub fn to_input_point(&self, point: OutputPoint) -> InputPoint {
        let mut cursor = self.transforms.cursor::<OutputPoint, InputPoint>();
        cursor.seek(&point, Bias::Right, &());
        InputPoint(cursor.sum_start().0 + (point.0 - cursor.seek_start().0))
    }

    pub fn to_output_point(&self, point: InputPoint) -> OutputPoint {
        let mut cursor = self.transforms.cursor::<InputPoint, OutputPoint>();
        cursor.seek(&point, Bias::Right, &());
        OutputPoint(cursor.sum_start().0 + (point.0 - cursor.seek_start().0))
    }

    pub fn clip_point(&self, mut point: OutputPoint, bias: Bias) -> OutputPoint {
        if bias == Bias::Left {
            let mut cursor = self.transforms.cursor::<OutputPoint, ()>();
            cursor.seek(&point, Bias::Right, &());
            let transform = cursor.item().expect("invalid point");
            if !transform.is_isomorphic() {
                *point.column_mut() -= 1;
            }
        }

        self.to_output_point(self.input.clip_point(self.to_input_point(point), bias))
    }
}

pub struct Chunks<'a> {
    input_chunks: tab_map::Chunks<'a>,
    input_chunk: &'a str,
    input_position: InputPoint,
    transforms: Cursor<'a, Transform, OutputPoint, InputPoint>,
}

pub struct HighlightedChunks<'a> {
    input_chunks: tab_map::HighlightedChunks<'a>,
    input_chunk: &'a str,
    style_id: StyleId,
    output_position: OutputPoint,
    max_output_row: u32,
    transforms: Cursor<'a, Transform, OutputPoint, InputPoint>,
}

pub struct BufferRows<'a> {
    input_buffer_rows: fold_map::BufferRows<'a>,
    input_buffer_row: u32,
    output_row: u32,
    max_output_row: u32,
    transforms: Cursor<'a, Transform, OutputPoint, InputPoint>,
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let transform = self.transforms.item()?;
        if let Some(display_text) = transform.display_text {
            self.transforms.next(&());
            return Some(display_text);
        }

        if self.input_chunk.is_empty() {
            self.input_chunk = self.input_chunks.next().unwrap();
        }

        let mut input_len = 0;
        let transform_end = self.transforms.sum_end(&());
        for c in self.input_chunk.chars() {
            let char_len = c.len_utf8();
            input_len += char_len;
            if c == '\n' {
                *self.input_position.row_mut() += 1;
                *self.input_position.column_mut() = 0;
            } else {
                *self.input_position.column_mut() += char_len as u32;
            }

            if self.input_position >= transform_end {
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
    type Item = (&'a str, StyleId);

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_position.row() >= self.max_output_row {
            return None;
        }

        let transform = self.transforms.item()?;
        if let Some(display_text) = transform.display_text {
            self.output_position.0 += transform.summary.output.lines;
            self.transforms.next(&());
            return Some((display_text, self.style_id));
        }

        if self.input_chunk.is_empty() {
            let (chunk, style_id) = self.input_chunks.next().unwrap();
            self.input_chunk = chunk;
            self.style_id = style_id;
        }

        let mut input_len = 0;
        let transform_end = self.transforms.seek_end(&());
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
        Some((prefix, self.style_id))
    }
}

impl<'a> Iterator for BufferRows<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_row > self.max_output_row {
            return None;
        }

        let buffer_row = self.input_buffer_row;
        self.output_row += 1;
        self.transforms
            .seek_forward(&OutputPoint::new(self.output_row, 0), Bias::Left, &());
        if self.transforms.item().map_or(false, |t| t.is_isomorphic()) {
            self.input_buffer_row = self.input_buffer_rows.next().unwrap();
        }

        Some(buffer_row)
    }
}

struct State {
    snapshot: Snapshot,
    pending_edits: VecDeque<(InputSnapshot, Vec<InputEdit>)>,
}

pub struct WrapMap {
    state: Mutex<State>,
    background_changes_tx: channel::Sender<Change>,
    background_snapshot: watch::Receiver<Snapshot>,
    _background_task: Task<()>,
}

impl WrapMap {
    pub fn new(
        input: InputSnapshot,
        settings: Settings,
        wrap_width: Option<f32>,
        cx: &AppContext,
    ) -> Self {
        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().fonts();
        let snapshot = Snapshot::new(input.clone());
        let (background_snapshot_tx, background_snapshot_rx) =
            watch::channel_with(snapshot.clone());
        let (edits_tx, edits_rx) = channel::unbounded();
        let background_task = {
            let snapshot = snapshot.clone();
            cx.background().spawn(async move {
                let mut wrapper =
                    BackgroundWrapper::new(snapshot, settings, wrap_width, font_cache, font_system);
                wrapper.run(input, edits_rx, background_snapshot_tx).await;
            })
        };

        Self {
            state: Mutex::new(State {
                snapshot,
                pending_edits: VecDeque::new(),
            }),
            background_changes_tx: edits_tx,
            background_snapshot: background_snapshot_rx,
            _background_task: background_task,
        }
    }

    pub fn sync(&self, input: InputSnapshot, edits: Vec<InputEdit>, cx: &AppContext) -> Snapshot {
        let mut background_snapshot = self.background_snapshot.clone();
        let mut snapshot = background_snapshot.borrow().clone();

        if !edits.is_empty() {
            self.background_changes_tx
                .try_send(Change::Input {
                    snapshot: input.clone(),
                    edits: edits.clone(),
                })
                .unwrap();

            cx.background().block_on(Duration::from_millis(5), async {
                loop {
                    snapshot = background_snapshot.recv().await.unwrap();
                    if snapshot.input.version() == input.version() {
                        break;
                    }
                }
            });
        }

        let mut state = &mut *self.state.lock();
        state.snapshot = snapshot;
        state.pending_edits.push_back((input, edits));

        while let Some((pending_input, _)) = state.pending_edits.front() {
            if pending_input.version() <= state.snapshot.input.version() {
                state.pending_edits.pop_front();
            } else {
                break;
            }
        }

        for (input, edits) in &state.pending_edits {
            state.snapshot.interpolate(input.clone(), &edits);
        }
        state.snapshot.clone()
    }

    pub fn set_wrap_width(&self, width: Option<f32>) {
        self.background_changes_tx
            .try_send(Change::Width(width))
            .unwrap();
    }

    pub fn notifications(&self) -> impl Stream<Item = ()> {
        self.background_snapshot.clone().map(|_| ())
    }
}

struct BackgroundWrapper {
    wrap_width: Option<f32>,
    snapshot: Snapshot,
    line_wrapper: LineWrapper,
}

struct LineWrapper {
    font_system: Arc<dyn FontSystem>,
    font_cache: Arc<FontCache>,
    font_id: FontId,
    font_size: f32,
    cached_ascii_char_widths: [f32; 128],
    cached_other_char_widths: HashMap<char, f32>,
}

enum Change {
    Input {
        snapshot: InputSnapshot,
        edits: Vec<tab_map::Edit>,
    },
    Width(Option<f32>),
}

impl BackgroundWrapper {
    fn new(
        snapshot: Snapshot,
        settings: Settings,
        wrap_width: Option<f32>,
        font_cache: Arc<FontCache>,
        font_system: Arc<dyn FontSystem>,
    ) -> Self {
        Self {
            wrap_width,
            snapshot,
            line_wrapper: LineWrapper::new(font_system, font_cache, settings),
        }
    }

    async fn run(
        &mut self,
        input: InputSnapshot,
        edits_rx: channel::Receiver<Change>,
        mut snapshot_tx: watch::Sender<Snapshot>,
    ) {
        let edit = InputEdit {
            old_lines: Default::default()..input.max_point(),
            new_lines: Default::default()..input.max_point(),
        };
        self.sync(input, vec![edit]);
        if snapshot_tx.send(self.snapshot.clone()).await.is_err() {
            return;
        }

        while let Ok(change) = edits_rx.recv().await {
            match change {
                Change::Input { snapshot, edits } => self.sync(snapshot, edits),
                Change::Width(wrap_width) => {
                    if self.wrap_width == wrap_width {
                        continue;
                    } else {
                        self.wrap_width = wrap_width;
                        let input = self.snapshot.input.clone();
                        let edit = InputEdit {
                            old_lines: Default::default()..input.max_point(),
                            new_lines: Default::default()..input.max_point(),
                        };
                        self.sync(input, vec![edit])
                    }
                }
            };

            if snapshot_tx.send(self.snapshot.clone()).await.is_err() {
                break;
            }
        }
    }

    fn sync(&mut self, new_snapshot: InputSnapshot, edits: Vec<InputEdit>) {
        if edits.is_empty() {
            return;
        }

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
        {
            let mut row_edits = row_edits.into_iter().peekable();
            let mut old_cursor = self.snapshot.transforms.cursor::<InputPoint, ()>();

            new_transforms = old_cursor.slice(
                &InputPoint::new(row_edits.peek().unwrap().old_rows.start, 0),
                Bias::Right,
                &(),
            );

            while let Some(edit) = row_edits.next() {
                if edit.new_rows.start > new_transforms.summary().input.lines.row {
                    let summary = new_snapshot.text_summary_for_range(
                        InputPoint::new(new_transforms.summary().input.lines.row, 0)
                            ..InputPoint::new(edit.new_rows.start, 0),
                    );
                    new_transforms.push_or_extend(Transform::isomorphic(summary));
                }

                let mut input_row = edit.new_rows.start;
                let mut line = String::new();
                let mut remaining = None;
                let mut chunks = new_snapshot.chunks_at(InputPoint::new(input_row, 0));
                loop {
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
                    if let Some(wrap_width) = self.wrap_width {
                        for boundary_ix in self
                            .line_wrapper
                            .wrap_line_without_shaping(&line, wrap_width)
                        {
                            let wrapped = &line[prev_boundary_ix..boundary_ix];
                            new_transforms
                                .push_or_extend(Transform::isomorphic(TextSummary::from(wrapped)));
                            new_transforms.push_or_extend(Transform::newline());
                            prev_boundary_ix = boundary_ix;
                        }
                    }

                    if prev_boundary_ix < line.len() {
                        new_transforms.push_or_extend(Transform::isomorphic(TextSummary::from(
                            &line[prev_boundary_ix..],
                        )));
                    }

                    line.clear();
                    input_row += 1;
                    if input_row == edit.new_rows.end {
                        break;
                    }
                }

                old_cursor.seek_forward(&InputPoint::new(edit.old_rows.end, 0), Bias::Right, &());
                if let Some(next_edit) = row_edits.peek() {
                    if next_edit.old_rows.start > old_cursor.seek_end(&()).row() {
                        if old_cursor.seek_end(&()) > InputPoint::new(edit.old_rows.end, 0) {
                            let summary = self.snapshot.input.text_summary_for_range(
                                InputPoint::new(edit.old_rows.end, 0)..old_cursor.seek_end(&()),
                            );
                            new_transforms.push_or_extend(Transform::isomorphic(summary));
                        }
                        old_cursor.next(&());
                        new_transforms.push_tree(
                            old_cursor.slice(
                                &InputPoint::new(next_edit.old_rows.start, 0),
                                Bias::Right,
                                &(),
                            ),
                            &(),
                        );
                    }
                } else {
                    if old_cursor.seek_end(&()) > InputPoint::new(edit.old_rows.end, 0) {
                        let summary = self.snapshot.input.text_summary_for_range(
                            InputPoint::new(edit.old_rows.end, 0)..old_cursor.seek_end(&()),
                        );
                        new_transforms.push_or_extend(Transform::isomorphic(summary));
                    }
                    old_cursor.next(&());
                    new_transforms.push_tree(old_cursor.suffix(&()), &());
                }
            }
        }

        self.snapshot = Snapshot {
            transforms: new_transforms,
            input: new_snapshot,
        };
    }
}

impl LineWrapper {
    fn new(
        font_system: Arc<dyn FontSystem>,
        font_cache: Arc<FontCache>,
        settings: Settings,
    ) -> Self {
        let font_id = font_cache
            .select_font(settings.buffer_font_family, &Default::default())
            .unwrap();
        let font_size = settings.buffer_font_size;
        Self {
            font_cache,
            font_system,
            font_id,
            font_size,
            cached_ascii_char_widths: [f32::NAN; 128],
            cached_other_char_widths: HashMap::new(),
        }
    }

    fn wrap_line_with_shaping(&mut self, line: &str, wrap_width: f32) -> Vec<usize> {
        self.font_system
            .wrap_line(line, self.font_id, self.font_size, wrap_width)
    }

    fn wrap_line_without_shaping(&mut self, line: &str, wrap_width: f32) -> Vec<usize> {
        let mut width = 0.0;
        let mut result = Vec::new();
        let mut last_boundary_ix = 0;
        let mut last_boundary_width = 0.0;
        let mut prev_c = '\0';
        for (ix, c) in line.char_indices() {
            if self.is_boundary(prev_c, c) {
                last_boundary_ix = ix;
                last_boundary_width = width;
            }

            let char_width = self.width_for_char(c);
            width += char_width;
            if width > wrap_width {
                if last_boundary_ix > 0 {
                    result.push(last_boundary_ix);
                    width -= last_boundary_width;
                    last_boundary_ix = 0;
                } else {
                    result.push(ix);
                    width = char_width;
                }
            }
            prev_c = c;
        }
        result
    }

    fn is_boundary(&self, prev: char, next: char) -> bool {
        if prev == ' ' || next == ' ' {
            return true;
        }
        false
    }

    fn width_for_char(&mut self, c: char) -> f32 {
        if (c as u32) < 128 {
            let mut width = self.cached_ascii_char_widths[c as usize];
            if width.is_nan() {
                width = self.compute_width_for_char(c);
                self.cached_ascii_char_widths[c as usize] = width;
            }
            width
        } else {
            let mut width = self
                .cached_other_char_widths
                .get(&c)
                .copied()
                .unwrap_or(f32::NAN);
            if width.is_nan() {
                width = self.compute_width_for_char(c);
                self.cached_other_char_widths.insert(c, width);
            }
            width
        }
    }

    fn compute_width_for_char(&self, c: char) -> f32 {
        self.font_system
            .layout_line(
                &c.to_string(),
                self.font_size,
                &[(1, self.font_id, Default::default())],
            )
            .width
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Transform {
    summary: TransformSummary,
    display_text: Option<&'static str>,
}

impl Transform {
    fn isomorphic(summary: TextSummary) -> Self {
        Self {
            summary: TransformSummary {
                input: summary.clone(),
                output: summary,
            },
            display_text: None,
        }
    }

    fn newline() -> Self {
        Self {
            summary: TransformSummary {
                input: TextSummary::default(),
                output: TextSummary {
                    lines: Point::new(1, 0),
                    first_line_chars: 0,
                    last_line_chars: 0,
                    longest_row: 0,
                    longest_row_chars: 0,
                },
            },
            display_text: Some("\n"),
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

impl SumTree<Transform> {
    pub fn push_or_extend(&mut self, transform: Transform) {
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TransformSummary {
    input: TextSummary,
    output: TextSummary,
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.input += &other.input;
        self.output += &other.output;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InputPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.input.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for OutputPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.output.lines;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor::{
            display_map::{fold_map::FoldMap, tab_map::TabMap},
            Buffer,
        },
        util::RandomCharIter,
    };
    use gpui::fonts::FontId;
    use rand::prelude::*;
    use std::env;

    #[gpui::test]
    fn test_line_wrapper(cx: &mut gpui::MutableAppContext) {
        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().fonts();
        let settings = Settings {
            tab_size: 4,
            buffer_font_family: font_cache.load_family(&["Courier"]).unwrap(),
            buffer_font_size: 16.0,
            ..Settings::new(&font_cache).unwrap()
        };

        let mut wrapper = LineWrapper::new(font_system, font_cache, settings);

        assert_eq!(
            wrapper.wrap_line_with_shaping("aa bbb cccc ddddd eeee", 72.0),
            &[7, 12, 18],
        );
        assert_eq!(
            wrapper.wrap_line_without_shaping("aa bbb cccc ddddd eeee", 72.0),
            &[7, 12, 18],
        );

        assert_eq!(
            wrapper.wrap_line_with_shaping("aaa aaaaaaaaaaaaaaaaaa", 72.0),
            &[4, 11, 18],
        );
        assert_eq!(
            wrapper.wrap_line_without_shaping("aaa aaaaaaaaaaaaaaaaaa", 72.0),
            &[4, 11, 18],
        );
    }

    #[gpui::test]
    fn test_random_wraps(cx: &mut gpui::MutableAppContext) {
        let iterations = env::var("ITERATIONS")
            .map(|i| i.parse().expect("invalid `ITERATIONS` variable"))
            .unwrap_or(100);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);
        let seed_range = if let Ok(seed) = env::var("SEED") {
            let seed = seed.parse().expect("invalid `SEED` variable");
            seed..seed + 1
        } else {
            0..iterations
        };

        for seed in seed_range {
            dbg!(seed);
            let mut rng = StdRng::seed_from_u64(seed);
            let font_cache = cx.font_cache().clone();
            let font_system = cx.platform().fonts();
            let wrap_width = rng.gen_range(100.0..=1000.0);
            let settings = Settings {
                tab_size: rng.gen_range(1..=4),
                buffer_font_family: font_cache.load_family(&["Helvetica"]).unwrap(),
                buffer_font_size: 14.0,
                ..Settings::new(&font_cache).unwrap()
            };
            log::info!("Tab size: {}", settings.tab_size);
            log::info!("Wrap width: {}", wrap_width);

            let font_id = font_cache
                .select_font(settings.buffer_font_family, &Default::default())
                .unwrap();

            let buffer = cx.add_model(|cx| {
                let len = rng.gen_range(0..10);
                let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
                log::info!("Initial buffer text: {:?} (len: {})", text, text.len());
                Buffer::new(0, text, cx)
            });
            let (fold_map, folds_snapshot) = FoldMap::new(buffer.clone(), cx.as_ref());
            let (tab_map, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), settings.tab_size);
            let mut wrapper = BackgroundWrapper::new(
                Snapshot::new(tabs_snapshot.clone()),
                settings.clone(),
                Some(wrap_width),
                font_cache.clone(),
                font_system.clone(),
            );
            let edit = InputEdit {
                old_lines: Default::default()..tabs_snapshot.max_point(),
                new_lines: Default::default()..tabs_snapshot.max_point(),
            };
            wrapper.sync(tabs_snapshot.clone(), vec![edit]);

            let unwrapped_text = tabs_snapshot.text();
            let expected_text =
                wrap_text(&unwrapped_text, wrap_width, font_id, font_system.as_ref());

            let actual_text = wrapper
                .snapshot
                .chunks_at(OutputPoint::zero())
                .collect::<String>();

            assert_eq!(
                actual_text, expected_text,
                "unwrapped text is: {:?}",
                unwrapped_text
            );

            let mut interpolated_snapshot = wrapper.snapshot.clone();
            for _i in 0..operations {
                buffer.update(cx, |buffer, cx| buffer.randomly_mutate(&mut rng, cx));
                let (snapshot, edits) = fold_map.read(cx.as_ref());
                let (snapshot, edits) = tab_map.sync(snapshot, edits);
                interpolated_snapshot.interpolate(snapshot.clone(), &edits);
                interpolated_snapshot.check_invariants();

                let unwrapped_text = snapshot.text();
                let expected_text =
                    wrap_text(&unwrapped_text, wrap_width, font_id, font_system.as_ref());
                wrapper.sync(snapshot, edits);
                wrapper.snapshot.check_invariants();

                let actual_text = wrapper.snapshot.text();
                assert_eq!(
                    actual_text, expected_text,
                    "unwrapped text is: {:?}",
                    unwrapped_text
                );

                interpolated_snapshot = wrapper.snapshot.clone();
            }
        }
    }

    fn wrap_text(
        unwrapped_text: &str,
        wrap_width: f32,
        font_id: FontId,
        font_system: &dyn FontSystem,
    ) -> String {
        let mut wrapped_text = String::new();
        for (row, line) in unwrapped_text.split('\n').enumerate() {
            if row > 0 {
                wrapped_text.push('\n')
            }

            let mut prev_ix = 0;
            for ix in font_system.wrap_line(line, font_id, 14.0, wrap_width) {
                wrapped_text.push_str(&line[prev_ix..ix]);
                wrapped_text.push('\n');
                prev_ix = ix;
            }
            wrapped_text.push_str(&line[prev_ix..]);
        }
        wrapped_text
    }

    impl Snapshot {
        fn text(&self) -> String {
            self.chunks_at(OutputPoint::zero()).collect()
        }

        fn check_invariants(&self) {
            assert_eq!(
                InputPoint::from(self.transforms.summary().input.lines),
                self.input.max_point()
            );

            let mut transforms = self.transforms.cursor::<(), ()>().peekable();
            while let Some(transform) = transforms.next() {
                let next_transform = transforms.peek();
                assert!(
                    !transform.is_isomorphic()
                        || next_transform.map_or(true, |t| !t.is_isomorphic())
                );
            }
        }
    }
}
