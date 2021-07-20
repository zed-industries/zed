use super::tab_map::{
    self, Edit as InputEdit, OutputPoint as InputPoint, Snapshot as InputSnapshot, TextSummary,
};
use crate::{
    editor::Point,
    sum_tree::{self, Cursor, SumTree},
    util::Bias,
};
use gpui::{font_cache::FamilyId, AppContext, FontCache, FontSystem, Task};
use parking_lot::Mutex;
use postage::{prelude::Sink, watch};
use smol::channel;
use std::{
    ops::{AddAssign, Range, Sub},
    sync::Arc,
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

impl AddAssign<Self> for OutputPoint {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += &rhs.0;
    }
}

impl Sub<Self> for OutputPoint {
    type Output = OutputPoint;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

#[derive(Clone)]
pub struct Snapshot {
    transforms: SumTree<Transform>,
    input: InputSnapshot,
    version: usize,
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
            version: input.version(),
            input,
        }
    }

    pub fn chunks_at(&self, point: OutputPoint) -> Chunks {
        let mut transforms = self.transforms.cursor();
        transforms.seek(&point, Bias::Right, &());
        let input_position =
            *transforms.sum_start() + InputPoint((point - *transforms.seek_start()).0);
        let input_chunks = self.input.chunks_at(input_position);
        Chunks {
            input_chunks,
            transforms,
            input_position,
            input_chunk: "",
        }
    }
}

pub struct Chunks<'a> {
    input_chunks: tab_map::Chunks<'a>,
    input_chunk: &'a str,
    input_position: InputPoint,
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

struct State {
    snapshot: Snapshot,
    interpolated_version: usize,
}

#[derive(Clone)]
pub struct Config {
    pub wrap_width: f32,
    pub font_family: FamilyId,
    pub font_size: f32,
}

pub struct WrapMap {
    state: Mutex<State>,
    edits_tx: channel::Sender<(InputSnapshot, Vec<InputEdit>)>,
    background_snapshots: watch::Receiver<Snapshot>,
    _background_task: Task<()>,
}

impl WrapMap {
    pub fn new(input: InputSnapshot, config: Config, cx: &AppContext) -> Self {
        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().fonts();
        let snapshot = Snapshot::new(input.clone());
        let (background_snapshots_tx, background_snapshots_rx) =
            watch::channel_with(snapshot.clone());
        let (edits_tx, edits_rx) = channel::unbounded();
        let background_task = {
            let snapshot = snapshot.clone();
            cx.background().spawn(async move {
                let mut wrapper = BackgroundWrapper::new(snapshot, config, font_cache, font_system);
                wrapper.run(input, edits_rx, background_snapshots_tx).await;
            })
        };

        Self {
            state: Mutex::new(State {
                interpolated_version: snapshot.version,
                snapshot,
            }),
            edits_tx,
            background_snapshots: background_snapshots_rx,
            _background_task: background_task,
        }
    }

    pub fn sync(&self, input: InputSnapshot, edits: Vec<InputEdit>) -> Snapshot {
        // TODO: interpolate
        self.edits_tx.try_send((input, edits)).unwrap();
        self.state.lock().snapshot.clone()
    }
}

struct BackgroundWrapper {
    config: Config,
    font_cache: Arc<FontCache>,
    font_system: Arc<dyn FontSystem>,
    snapshot: Snapshot,
}

impl BackgroundWrapper {
    fn new(
        snapshot: Snapshot,
        config: Config,
        font_cache: Arc<FontCache>,
        font_system: Arc<dyn FontSystem>,
    ) -> Self {
        Self {
            config,
            font_cache,
            font_system,
            snapshot,
        }
    }

    async fn run(
        &mut self,
        snapshot: InputSnapshot,
        edits_rx: channel::Receiver<(InputSnapshot, Vec<InputEdit>)>,
        mut snapshots_tx: watch::Sender<Snapshot>,
    ) {
        let edit = InputEdit {
            old_lines: Default::default()..snapshot.max_point(),
            new_lines: Default::default()..snapshot.max_point(),
        };
        self.sync(snapshot, vec![edit]);
        if snapshots_tx.send(self.snapshot.clone()).await.is_err() {
            return;
        }

        while let Ok((snapshot, edits)) = edits_rx.recv().await {
            self.sync(snapshot, edits);
            if snapshots_tx.send(self.snapshot.clone()).await.is_err() {
                break;
            }
        }
    }

    fn sync(&mut self, new_snapshot: InputSnapshot, edits: Vec<InputEdit>) {
        if edits.is_empty() {
            return;
        }

        let font_id = self
            .font_cache
            .select_font(self.config.font_family, &Default::default())
            .unwrap();
        let font_size = self.config.font_size;
        let wrap_width = self.config.wrap_width;

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
                    for boundary_ix in self
                        .font_system
                        .wrap_line(&line, font_id, font_size, wrap_width)
                    {
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
            version: new_snapshot.version(),
            input: new_snapshot,
        };
        self.check_invariants();
    }

    fn check_invariants(&self) {
        #[cfg(debug_assertions)]
        {
            let mut transforms = self.snapshot.transforms.cursor::<(), ()>().peekable();
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
        *self += InputPoint(summary.input.lines);
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for OutputPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += OutputPoint(summary.output.lines);
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
    use futures::StreamExt;
    use gpui::fonts::FontId;
    use rand::prelude::*;
    use std::env;

    #[gpui::test]
    async fn test_simple_wraps(mut cx: gpui::TestAppContext) {
        let text = "one two three four five\nsix seven eight";
        let font_cache = cx.font_cache().clone();
        let config = Config {
            wrap_width: 64.,
            font_family: font_cache.load_family(&["Helvetica"]).unwrap(),
            font_size: 14.0,
        };

        let buffer = cx.add_model(|cx| Buffer::new(0, text.to_string(), cx));
        let mut wrap_map = cx.read(|cx| {
            let fold_map = FoldMap::new(buffer.clone(), cx);
            let (folds_snapshot, edits) = fold_map.read(cx);
            let tab_map = TabMap::new(folds_snapshot.clone(), 4);
            let (tabs_snapshot, _) = tab_map.sync(folds_snapshot, edits);
            WrapMap::new(tabs_snapshot, config, cx)
        });

        wrap_map.background_snapshots.next().await;
        let snapshot = wrap_map.background_snapshots.next().await.unwrap();

        assert_eq!(
            snapshot
                .chunks_at(OutputPoint(Point::new(0, 3)))
                .collect::<String>(),
            " two \nthree four \nfive\nsix seven \neight"
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

            let buffer = cx.add_model(|cx| {
                let len = rng.gen_range(0..10);
                let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
                log::info!("Initial buffer text: {:?}", text);
                Buffer::new(0, text, cx)
            });
            let fold_map = FoldMap::new(buffer.clone(), cx.as_ref());
            let (folds_snapshot, edits) = fold_map.read(cx.as_ref());
            let tab_map = TabMap::new(folds_snapshot.clone(), rng.gen_range(1..=4));
            let (tabs_snapshot, _) = tab_map.sync(folds_snapshot, edits);
            let font_cache = cx.font_cache().clone();
            let font_system = cx.platform().fonts();
            let config = Config {
                wrap_width: rng.gen_range(100.0..=1000.0),
                font_family: font_cache.load_family(&["Helvetica"]).unwrap(),
                font_size: 14.0,
            };
            let font_id = font_cache
                .select_font(config.font_family, &Default::default())
                .unwrap();
            let mut wrapper = BackgroundWrapper::new(
                Snapshot::new(tabs_snapshot.clone()),
                config.clone(),
                font_cache.clone(),
                font_system.clone(),
            );
            let edit = InputEdit {
                old_lines: Default::default()..tabs_snapshot.max_point(),
                new_lines: Default::default()..tabs_snapshot.max_point(),
            };
            wrapper.sync(tabs_snapshot.clone(), vec![edit]);

            let unwrapped_text = tabs_snapshot.text();
            let expected_text = wrap_text(
                &unwrapped_text,
                config.wrap_width,
                font_id,
                font_system.as_ref(),
            );

            let actual_text = wrapper
                .snapshot
                .chunks_at(OutputPoint::zero())
                .collect::<String>();

            assert_eq!(
                actual_text, expected_text,
                "unwrapped text is: {:?}",
                unwrapped_text
            );

            for _i in 0..operations {
                buffer.update(cx, |buffer, cx| buffer.randomly_mutate(&mut rng, cx));
                let (snapshot, edits) = fold_map.read(cx.as_ref());
                let (snapshot, edits) = tab_map.sync(snapshot, edits);
                let unwrapped_text = snapshot.text();
                let expected_text = wrap_text(
                    &unwrapped_text,
                    config.wrap_width,
                    font_id,
                    font_system.as_ref(),
                );
                wrapper.sync(snapshot, edits);

                let actual_text = wrapper
                    .snapshot
                    .chunks_at(OutputPoint::zero())
                    .collect::<String>();
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
}
