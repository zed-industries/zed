use super::tab_map::{
    Edit as InputEdit, OutputOffset as InputOffset, OutputPoint as InputPoint,
    Snapshot as InputSnapshot, TextSummary,
};
use crate::{
    editor::Point,
    sum_tree::{self, SumTree},
    util::Bias,
};
use gpui::{font_cache::FamilyId, AppContext, FontCache, FontSystem, Task};
use parking_lot::Mutex;
use postage::{prelude::Sink, watch};
use smol::channel;
use std::{ops::Range, sync::Arc};

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
            old_lines: Default::default()..Default::default(),
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

        let mut new_transforms;
        {
            struct RowEdit {
                old_rows: Range<u32>,
                new_rows: Range<u32>,
            }

            let mut edits = edits
                .into_iter()
                .map(|edit| RowEdit {
                    old_rows: edit.old_lines.start.row()..edit.old_lines.end.row() + 1,
                    new_rows: edit.new_lines.start.row()..edit.new_lines.end.row() + 1,
                })
                .peekable();
            let mut old_cursor = self.snapshot.transforms.cursor::<InputPoint, ()>();

            new_transforms = old_cursor.slice(
                &InputPoint::new(edits.peek().unwrap().old_rows.start, 0),
                Bias::Right,
                &(),
            );

            for edit in edits {
                if edit.new_rows.start > new_transforms.summary().input.row() {
                    new_transforms.push(
                        Transform::isomorphic(new_snapshot.input.text_summary_for_rows(
                            new_transforms.summary().input.row()..edit.new_rows.start,
                        )),
                        &(),
                    );
                }

                let mut row = edit.new_rows.start;
                let mut line = String::new();
                'outer: for chunk in new_snapshot.chunks_at(InputPoint::new(row, 0)) {
                    for (ix, line_chunk) in chunk.split('\n').enumerate() {
                        if ix > 0 {
                            let mut prev_boundary_ix = 0;
                            for boundary_ix in self
                                .font_system
                                .wrap_line(&line, font_id, font_size, wrap_width)
                            {
                                let wrapped = &line[prev_boundary_ix..boundary_ix];
                                new_transforms
                                    .push(Transform::isomorphic(TextSummary::from(wrapped)), &());
                                new_transforms.push(Transform::newline(), &());
                                prev_boundary_ix = boundary_ix;
                            }

                            line.clear();
                            row += 1;
                            if row == edit.new_rows.end {
                                break 'outer;
                            }
                        }

                        line.push_str(line_chunk);
                    }
                }

                old_cursor.seek_forward(&edit.old_rows.end, Bias::Right, &());
                if let Some(next_edit) = edits.peek() {
                    if next_edit.old_rows.start > old_cursor.seek_end(&()).row() {
                        new_transforms.push(
                            Transform::isomorphic(self.snapshot.input.text_summary_for_rows(
                                edit.old_rows.end..old_cursor.seek_end(&()).row(),
                            )),
                            &(),
                        );
                        old_cursor.next(&());
                        new_transforms.push_tree(
                            old_cursor.slice(&next_edit.old_rows.start, Bias::Right, &()),
                            &(),
                        );
                    }
                } else {
                    new_transforms.push(
                        Transform::isomorphic(self.snapshot.input.text_summary_for_rows(
                            edit.old_rows.end..old_cursor.seek_end(&()).row(),
                        )),
                        &(),
                    );
                    old_cursor.next(&());
                    new_transforms.push_tree(old_cursor.suffix(&()), &());
                }
            }
        }

        self.snapshot.transforms = new_transforms;
        self.snapshot.version = new_snapshot.version();
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
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self) -> Self::Summary {
        self.summary.clone()
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
        *self += &InputPoint(summary.input.lines);
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
                old_lines: Default::default()..Default::default(),
                new_lines: Default::default()..tabs_snapshot.max_point(),
            };
            wrapper.sync(tabs_snapshot.clone(), vec![edit]);

            let mut expected_text = String::new();
            for line in tabs_snapshot.text().lines() {
                let mut prev_ix = 0;
                for ix in font_system.wrap_line(line, font_id, 14.0, config.wrap_width) {
                    expected_text.push_str(&line[prev_ix..ix]);
                    expected_text.push('\n');
                    prev_ix = ix;
                }
            }
            dbg!(expected_text);
        }
    }
}
