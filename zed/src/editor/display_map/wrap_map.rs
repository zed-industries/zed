use std::sync::Arc;

use crate::{
    editor::{
        display_map::fold_map::{self, DisplayOffset},
        Point, TextSummary,
    },
    sum_tree::{self, SumTree},
    util::Bias,
};
use gpui::{font_cache::FamilyId, AppContext, FontCache, FontSystem, Task};
use parking_lot::Mutex;
use postage::{prelude::Sink, watch};
use smol::channel;

#[derive(Clone)]
pub struct Snapshot {
    transforms: SumTree<Transform>,
    version: usize,
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
    edits_tx: channel::Sender<(fold_map::Snapshot, Vec<fold_map::Edit>)>,
    background_snapshots: watch::Receiver<Snapshot>,
    _background_task: Task<()>,
}

impl WrapMap {
    pub fn new(folds_snapshot: fold_map::Snapshot, config: Config, cx: &AppContext) -> Self {
        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().fonts();
        let snapshot = Snapshot {
            transforms: SumTree::from_item(
                Transform {
                    summary: TransformSummary {
                        folded: folds_snapshot.text_summary(),
                        wrapped: folds_snapshot.text_summary(),
                    },
                    display_text: None,
                },
                &(),
            ),
            version: folds_snapshot.version,
        };
        let (background_snapshots_tx, background_snapshots_rx) =
            watch::channel_with(snapshot.clone());
        let (edits_tx, edits_rx) = channel::unbounded();
        let background_task = cx.background().spawn(async move {
            let mut wrapper = BackgroundWrapper::new(config, font_cache, font_system);
            wrapper
                .run(folds_snapshot, edits_rx, background_snapshots_tx)
                .await;
        });

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

    pub fn sync(&self, folds_snapshot: fold_map::Snapshot, edits: Vec<fold_map::Edit>) -> Snapshot {
        // TODO: interpolate
        self.edits_tx.try_send((folds_snapshot, edits)).unwrap();
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
    fn new(config: Config, font_cache: Arc<FontCache>, font_system: Arc<dyn FontSystem>) -> Self {
        Self {
            config,
            font_cache,
            font_system,
            snapshot: Snapshot {
                transforms: Default::default(),
                version: Default::default(),
            },
        }
    }

    async fn run(
        &mut self,
        snapshot: fold_map::Snapshot,
        edits_rx: channel::Receiver<(fold_map::Snapshot, Vec<fold_map::Edit>)>,
        mut snapshots_tx: watch::Sender<Snapshot>,
    ) {
        let edit = fold_map::Edit {
            old_bytes: DisplayOffset(0)..DisplayOffset(0),
            new_bytes: DisplayOffset(0)..DisplayOffset(snapshot.len()),
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

    fn sync(&mut self, snapshot: fold_map::Snapshot, edits: Vec<fold_map::Edit>) {
        let mut new_transforms = SumTree::new();
        {
            // let mut old_cursor = self.snapshot.transforms.cursor::<Point, ()>();
            // for edit in buffer.edits_since(self.snapshot.version.clone()) {
            //     new_transforms.push_tree(
            //         old_cursor.slice(&Point::new(edit.old_lines.start.row, 0), Bias::Left, &()),
            //         &(),
            //     );
            // }
        }

        self.snapshot.transforms = new_transforms;
        self.snapshot.version = snapshot.version;
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Transform {
    summary: TransformSummary,
    display_text: Option<&'static str>,
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self) -> Self::Summary {
        self.summary.clone()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TransformSummary {
    folded: TextSummary,
    wrapped: TextSummary,
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.folded += &other.folded;
        self.wrapped += &other.wrapped;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for Point {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += &summary.folded.lines;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor::{display_map::fold_map::FoldMap, Buffer},
        util::RandomCharIter,
    };
    use rand::prelude::*;
    use std::env;
    use Bias::{Left, Right};

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
            let (snapshot, _) = fold_map.read(cx.as_ref());
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
            let mut wrapper =
                BackgroundWrapper::new(config.clone(), font_cache.clone(), font_system.clone());
            let edit = fold_map::Edit {
                old_bytes: DisplayOffset(0)..DisplayOffset(0),
                new_bytes: DisplayOffset(0)..DisplayOffset(snapshot.len()),
            };
            wrapper.sync(snapshot.clone(), vec![edit]);

            let mut expected_text = String::new();
            for line in snapshot.text().lines() {
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
