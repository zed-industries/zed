use super::{
    fold_map,
    line_wrapper::LineWrapper,
    tab_map::{self, Edit as TabEdit, Snapshot as TabSnapshot, TabPoint, TextSummary},
};
use crate::{
    editor::Point,
    settings::StyleId,
    sum_tree::{self, Cursor, SumTree},
    util::Bias,
    Settings,
};
use gpui::{executor::Background, MutableAppContext, Task};
use parking_lot::Mutex;
use postage::{prelude::Stream, sink::Sink, watch};
use smol::future::yield_now;
use std::{collections::VecDeque, ops::Range, sync::Arc, time::Duration};

#[derive(Clone)]
pub struct WrapMap(Arc<Mutex<WrapMapState>>);

struct WrapMapState {
    snapshot: Snapshot,
    pending_edits: VecDeque<(TabSnapshot, Vec<TabEdit>)>,
    wrap_width: Option<f32>,
    background_task: Option<Task<()>>,
    updates: (watch::Sender<()>, watch::Receiver<()>),
    line_wrapper: Arc<LineWrapper>,
}

#[derive(Clone)]
pub struct Snapshot {
    tab_snapshot: TabSnapshot,
    transforms: SumTree<Transform>,
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
    transforms: Cursor<'a, Transform, WrapPoint, TabPoint>,
}

pub struct HighlightedChunks<'a> {
    input_chunks: tab_map::HighlightedChunks<'a>,
    input_chunk: &'a str,
    style_id: StyleId,
    output_position: WrapPoint,
    max_output_row: u32,
    transforms: Cursor<'a, Transform, WrapPoint, TabPoint>,
}

pub struct BufferRows<'a> {
    input_buffer_rows: fold_map::BufferRows<'a>,
    input_buffer_row: u32,
    output_row: u32,
    max_output_row: u32,
    transforms: Cursor<'a, Transform, WrapPoint, TabPoint>,
}

impl WrapMap {
    pub fn new(
        tab_snapshot: TabSnapshot,
        settings: Settings,
        wrap_width: Option<f32>,
        cx: &mut MutableAppContext,
    ) -> Self {
        let this = Self(Arc::new(Mutex::new(WrapMapState {
            background_task: None,
            wrap_width: None,
            updates: watch::channel(),
            pending_edits: Default::default(),
            snapshot: Snapshot::new(tab_snapshot),
            line_wrapper: Arc::new(LineWrapper::new(
                cx.platform().fonts(),
                cx.font_cache(),
                settings,
            )),
        })));
        this.set_wrap_width(wrap_width, cx);
        this
    }

    #[cfg(test)]
    pub fn is_rewrapping(&self) -> bool {
        self.0.lock().background_task.is_some()
    }

    pub fn notifications(&self) -> impl Stream<Item = ()> {
        let state = self.0.lock();
        let mut rx = state.updates.1.clone();
        // The first item in the stream always returns what's stored on the watch, but we only want
        // to receive notifications occurring after calling this method, so we discard the first
        // item.
        let _ = rx.blocking_recv();
        rx
    }

    pub fn sync(
        &self,
        tab_snapshot: TabSnapshot,
        edits: Vec<TabEdit>,
        cx: &mut MutableAppContext,
    ) -> Snapshot {
        self.0.lock().pending_edits.push_back((tab_snapshot, edits));
        self.flush_edits(cx.background());
        self.0.lock().snapshot.clone()
    }

    pub fn set_wrap_width(&self, wrap_width: Option<f32>, cx: &mut MutableAppContext) -> bool {
        let mut state = self.0.lock();
        if wrap_width == state.wrap_width {
            return false;
        }

        state.wrap_width = wrap_width;
        state.background_task.take();

        if let Some(wrap_width) = wrap_width {
            let mut new_snapshot = state.snapshot.clone();
            let line_wrapper = state.line_wrapper.clone();
            let task = cx.background().spawn(async move {
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
                        line_wrapper.as_ref(),
                    )
                    .await;
                new_snapshot
            });

            let executor = cx.background();
            match executor.block_with_timeout(Duration::from_millis(5), task) {
                Ok(snapshot) => {
                    state.snapshot = snapshot;
                }
                Err(wrap_task) => {
                    let this = self.clone();
                    let exec = executor.clone();
                    state.background_task = Some(executor.spawn(async move {
                        let snapshot = wrap_task.await;
                        {
                            let mut state = this.0.lock();
                            state.snapshot = snapshot;
                            state.background_task = None;
                        }
                        this.flush_edits(&exec);
                        this.0.lock().updates.0.blocking_send(()).ok();
                    }));
                }
            }
        }

        true
    }

    fn flush_edits(&self, executor: &Arc<Background>) {
        let mut state = self.0.lock();

        while let Some((tab_snapshot, _)) = state.pending_edits.front() {
            if tab_snapshot.version() <= state.snapshot.tab_snapshot.version() {
                state.pending_edits.pop_front();
            } else {
                break;
            }
        }

        if state.pending_edits.is_empty() {
            return;
        }

        if let Some(wrap_width) = state.wrap_width {
            if state.background_task.is_none() {
                let pending_edits = state.pending_edits.clone();
                let mut snapshot = state.snapshot.clone();
                let line_wrapper = state.line_wrapper.clone();

                let update_task = executor.spawn(async move {
                    for (tab_snapshot, edits) in pending_edits {
                        snapshot
                            .update(tab_snapshot, &edits, wrap_width, &line_wrapper)
                            .await;
                    }
                    snapshot
                });

                match executor.block_with_timeout(Duration::from_micros(500), update_task) {
                    Ok(snapshot) => {
                        state.snapshot = snapshot;
                    }
                    Err(update_task) => {
                        let this = self.clone();
                        let exec = executor.clone();
                        state.background_task = Some(executor.spawn(async move {
                            let snapshot = update_task.await;
                            {
                                let mut state = this.0.lock();
                                state.snapshot = snapshot;
                                state.background_task = None;
                            }
                            this.flush_edits(&exec);
                            this.0.lock().updates.0.blocking_send(()).ok();
                        }));
                    }
                }
            }
        }

        while let Some((tab_snapshot, _)) = state.pending_edits.front() {
            if tab_snapshot.version() <= state.snapshot.tab_snapshot.version() {
                state.pending_edits.pop_front();
            } else {
                break;
            }
        }

        for (tab_snapshot, edits) in state.pending_edits.clone() {
            state.snapshot.interpolate(tab_snapshot, &edits);
        }
    }
}

impl Snapshot {
    fn new(tab_snapshot: TabSnapshot) -> Self {
        let extent = tab_snapshot.text_summary();
        Self {
            transforms: SumTree::from_item(
                Transform {
                    summary: TransformSummary {
                        input: extent.clone(),
                        output: extent.clone(),
                    },
                    display_text: None,
                },
                &(),
            ),
            tab_snapshot,
        }
    }

    fn interpolate(&mut self, new_tab_snapshot: TabSnapshot, edits: &[TabEdit]) {
        let mut new_transforms;
        if edits.is_empty() {
            new_transforms = self.transforms.clone();
        } else {
            let mut old_cursor = self.transforms.cursor::<TabPoint, ()>();
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
                    if next_edit.old_lines.start > old_cursor.seek_end(&()) {
                        if old_cursor.seek_end(&()) > edit.old_lines.end {
                            let summary = self.tab_snapshot.text_summary_for_range(
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
                            .tab_snapshot
                            .text_summary_for_range(edit.old_lines.end..old_cursor.seek_end(&()));
                        new_transforms.push_or_extend(Transform::isomorphic(summary));
                    }
                    old_cursor.next(&());
                    new_transforms.push_tree(old_cursor.suffix(&()), &());
                }
            }
        }

        self.transforms = new_transforms;
        self.tab_snapshot = new_tab_snapshot;
    }

    async fn update(
        &mut self,
        new_tab_snapshot: TabSnapshot,
        edits: &[TabEdit],
        wrap_width: f32,
        line_wrapper: &LineWrapper,
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
            let mut old_cursor = self.transforms.cursor::<TabPoint, ()>();

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
                    for boundary_ix in line_wrapper.wrap_line(&line, wrap_width) {
                        let wrapped = &line[prev_boundary_ix..boundary_ix];
                        new_transforms
                            .push_or_extend(Transform::isomorphic(TextSummary::from(wrapped)));
                        new_transforms.push_or_extend(Transform::newline());
                        prev_boundary_ix = boundary_ix;
                    }

                    if prev_boundary_ix < line.len() {
                        new_transforms.push_or_extend(Transform::isomorphic(TextSummary::from(
                            &line[prev_boundary_ix..],
                        )));
                    }

                    line.clear();
                    yield_now().await;
                }

                old_cursor.seek_forward(&TabPoint::new(edit.old_rows.end, 0), Bias::Right, &());
                if let Some(next_edit) = row_edits.peek() {
                    if next_edit.old_rows.start > old_cursor.seek_end(&()).row() {
                        if old_cursor.seek_end(&()) > TabPoint::new(edit.old_rows.end, 0) {
                            let summary = self.tab_snapshot.text_summary_for_range(
                                TabPoint::new(edit.old_rows.end, 0)..old_cursor.seek_end(&()),
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
                    if old_cursor.seek_end(&()) > TabPoint::new(edit.old_rows.end, 0) {
                        let summary = self.tab_snapshot.text_summary_for_range(
                            TabPoint::new(edit.old_rows.end, 0)..old_cursor.seek_end(&()),
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
    }

    pub fn chunks_at(&self, point: WrapPoint) -> Chunks {
        let mut transforms = self.transforms.cursor::<WrapPoint, TabPoint>();
        transforms.seek(&point, Bias::Right, &());
        let input_position =
            TabPoint(transforms.sum_start().0 + (point.0 - transforms.seek_start().0));
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
        let mut transforms = self.transforms.cursor::<WrapPoint, TabPoint>();
        transforms.seek(&output_start, Bias::Right, &());
        let input_start =
            TabPoint(transforms.sum_start().0 + (output_start.0 - transforms.seek_start().0));
        let input_end = self
            .to_tab_point(output_end)
            .min(self.tab_snapshot.max_point());
        HighlightedChunks {
            input_chunks: self.tab_snapshot.highlighted_chunks(input_start..input_end),
            input_chunk: "",
            style_id: StyleId::default(),
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
        for chunk in self.chunks_at(WrapPoint::new(row, 0)) {
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
        let mut transforms = self.transforms.cursor::<WrapPoint, TabPoint>();
        transforms.seek(&WrapPoint::new(start_row, 0), Bias::Right, &());
        let input_row = transforms.sum_start().row() + (start_row - transforms.seek_start().row());
        let mut input_buffer_rows = self.tab_snapshot.buffer_rows(input_row);
        let input_buffer_row = input_buffer_rows.next().unwrap();
        BufferRows {
            transforms,
            input_buffer_row,
            input_buffer_rows,
            output_row: start_row,
            max_output_row: self.max_point().row(),
        }
    }

    pub fn to_tab_point(&self, point: WrapPoint) -> TabPoint {
        let mut cursor = self.transforms.cursor::<WrapPoint, TabPoint>();
        cursor.seek(&point, Bias::Right, &());
        TabPoint(cursor.sum_start().0 + (point.0 - cursor.seek_start().0))
    }

    pub fn to_wrap_point(&self, point: TabPoint) -> WrapPoint {
        let mut cursor = self.transforms.cursor::<TabPoint, WrapPoint>();
        cursor.seek(&point, Bias::Right, &());
        WrapPoint(cursor.sum_start().0 + (point.0 - cursor.seek_start().0))
    }

    pub fn clip_point(&self, mut point: WrapPoint, bias: Bias) -> WrapPoint {
        if bias == Bias::Left {
            let mut cursor = self.transforms.cursor::<WrapPoint, ()>();
            cursor.seek(&point, Bias::Right, &());
            let transform = cursor.item().expect("invalid point");
            if !transform.is_isomorphic() {
                *point.column_mut() -= 1;
            }
        }

        self.to_wrap_point(self.tab_snapshot.clip_point(self.to_tab_point(point), bias))
    }
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let transform = self.transforms.item()?;
        if let Some(display_text) = transform.display_text {
            self.output_position.0 += transform.summary.output.lines;
            self.transforms.next(&());
            return Some(display_text);
        }

        if self.input_chunk.is_empty() {
            self.input_chunk = self.input_chunks.next().unwrap();
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
            .seek_forward(&WrapPoint::new(self.output_row, 0), Bias::Left, &());
        if self.transforms.item().map_or(false, |t| t.is_isomorphic()) {
            self.input_buffer_row = self.input_buffer_rows.next().unwrap();
        }

        Some(buffer_row)
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

impl WrapPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(super::Point::new(row, column))
    }

    #[cfg(test)]
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
        editor::{
            display_map::{fold_map::FoldMap, tab_map::TabMap},
            Buffer,
        },
        util::RandomCharIter,
    };
    use rand::prelude::*;
    use std::env;

    #[gpui::test]
    async fn test_random_wraps(mut cx: gpui::TestAppContext) {
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
            cx.foreground().forbid_parking();

            dbg!(seed);
            let mut rng = StdRng::seed_from_u64(seed);
            let font_cache = cx.font_cache().clone();
            let font_system = cx.platform().fonts();
            let wrap_width = rng.gen_range(0.0..=1000.0);
            let settings = Settings {
                tab_size: rng.gen_range(1..=4),
                buffer_font_family: font_cache.load_family(&["Helvetica"]).unwrap(),
                buffer_font_size: 14.0,
                ..Settings::new(&font_cache).unwrap()
            };
            log::info!("Tab size: {}", settings.tab_size);
            log::info!("Wrap width: {}", wrap_width);

            let buffer = cx.add_model(|cx| {
                let len = rng.gen_range(0..10);
                let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
                Buffer::new(0, text, cx)
            });
            let (fold_map, folds_snapshot) = cx.read(|cx| FoldMap::new(buffer.clone(), cx));
            let (tab_map, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), settings.tab_size);
            log::info!(
                "Unwrapped text (unexpanded tabs): {:?}",
                folds_snapshot.text()
            );
            log::info!("Unwrapped text (expanded tabs): {:?}", tabs_snapshot.text());
            let wrap_map = cx.update(|cx| {
                WrapMap::new(
                    tabs_snapshot.clone(),
                    settings.clone(),
                    Some(wrap_width),
                    cx,
                )
            });
            let mut notifications = wrap_map.notifications();

            let mut line_wrapper = LineWrapper::new(font_system, &font_cache, settings);
            let unwrapped_text = tabs_snapshot.text();
            let expected_text = wrap_text(&unwrapped_text, wrap_width, &mut line_wrapper);

            if wrap_map.is_rewrapping() {
                notifications.recv().await;
            }

            let snapshot = cx.update(|cx| wrap_map.sync(tabs_snapshot, Vec::new(), cx));
            let actual_text = snapshot.text();
            assert_eq!(
                actual_text, expected_text,
                "unwrapped text is: {:?}",
                unwrapped_text
            );
            log::info!("Wrapped text: {:?}", actual_text);

            let mut interpolated_snapshot = snapshot.clone();
            for _i in 0..operations {
                buffer.update(&mut cx, |buffer, cx| buffer.randomly_mutate(&mut rng, cx));
                let (folds_snapshot, edits) = cx.read(|cx| fold_map.read(cx));
                log::info!(
                    "Unwrapped text (unexpanded tabs): {:?}",
                    folds_snapshot.text()
                );
                let (tabs_snapshot, edits) = tab_map.sync(folds_snapshot, edits);
                log::info!("Unwrapped text (expanded tabs): {:?}", tabs_snapshot.text());
                interpolated_snapshot.interpolate(tabs_snapshot.clone(), &edits);
                interpolated_snapshot.check_invariants(&mut rng);

                let unwrapped_text = tabs_snapshot.text();
                let expected_text = wrap_text(&unwrapped_text, wrap_width, &mut line_wrapper);
                let mut snapshot = cx.update(|cx| wrap_map.sync(tabs_snapshot.clone(), edits, cx));
                snapshot.check_invariants(&mut rng);

                if wrap_map.is_rewrapping() {
                    notifications.recv().await;
                    snapshot = cx.update(|cx| wrap_map.sync(tabs_snapshot, Vec::new(), cx));
                }

                snapshot.check_invariants(&mut rng);
                let actual_text = snapshot.text();
                assert_eq!(
                    actual_text, expected_text,
                    "unwrapped text is: {:?}",
                    unwrapped_text
                );
                log::info!("New wrapped text: {:?}", actual_text);

                interpolated_snapshot = snapshot.clone();
            }
        }
    }

    fn wrap_text(unwrapped_text: &str, wrap_width: f32, line_wrapper: &mut LineWrapper) -> String {
        let mut wrapped_text = String::new();
        for (row, line) in unwrapped_text.split('\n').enumerate() {
            if row > 0 {
                wrapped_text.push('\n')
            }

            let mut prev_ix = 0;
            for ix in line_wrapper.wrap_line(line, wrap_width) {
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
            self.chunks_at(WrapPoint::zero()).collect()
        }

        fn check_invariants(&mut self, rng: &mut impl Rng) {
            assert_eq!(
                TabPoint::from(self.transforms.summary().input.lines),
                self.tab_snapshot.max_point()
            );

            {
                let mut transforms = self.transforms.cursor::<(), ()>().peekable();
                while let Some(transform) = transforms.next() {
                    let next_transform = transforms.peek();
                    assert!(
                        !transform.is_isomorphic()
                            || next_transform.map_or(true, |t| !t.is_isomorphic())
                    );
                }
            }

            for _ in 0..5 {
                let mut end_row = rng.gen_range(0..=self.max_point().row());
                let start_row = rng.gen_range(0..=end_row);
                end_row += 1;

                let mut expected_text = self
                    .chunks_at(WrapPoint::new(start_row, 0))
                    .collect::<String>();
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
                    .map(|c| c.0)
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
