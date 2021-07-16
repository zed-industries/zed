use crate::{
    editor::{display_map::fold_map, Point, TextSummary},
    sum_tree::{self, SumTree},
    util::Bias,
};
use gpui::{AppContext, Task};
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

pub struct WrapMap {
    state: Mutex<State>,
    edits_tx: channel::Sender<(fold_map::Snapshot, Vec<fold_map::Edit>)>,
    background_snapshots: watch::Receiver<Snapshot>,
    _background_task: Task<()>,
}

impl WrapMap {
    pub fn new(folds_snapshot: fold_map::Snapshot, cx: &AppContext) -> Self {
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
            let mut wrapper = BackgroundWrapper::new(edits_rx, background_snapshots_tx);
            wrapper.run(folds_snapshot).await;
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

    pub fn read(&self, folds_snapshot: fold_map::Snapshot, edits: Vec<fold_map::Edit>) -> Snapshot {
        // TODO: interpolate
        self.edits_tx.try_send((folds_snapshot, edits)).unwrap();
        self.state.lock().snapshot.clone()
    }
}

struct BackgroundWrapper {
    edits_rx: channel::Receiver<(fold_map::Snapshot, Vec<fold_map::Edit>)>,
    snapshots_tx: watch::Sender<Snapshot>,
    snapshot: Snapshot,
}

impl BackgroundWrapper {
    fn new(
        edits_rx: channel::Receiver<(fold_map::Snapshot, Vec<fold_map::Edit>)>,
        snapshots_tx: watch::Sender<Snapshot>,
    ) -> Self {
        Self {
            edits_rx,
            snapshots_tx,
            snapshot: Snapshot {
                transforms: Default::default(),
                version: Default::default(),
            },
        }
    }

    async fn run(&mut self, snapshot: fold_map::Snapshot) {
        let edit = fold_map::Edit {
            old_bytes: 0..0,
            new_bytes: 0..snapshot.len(),
        };
        if !self.sync(snapshot, vec![edit]).await {
            return;
        }

        while let Ok((snapshot, edits)) = self.edits_rx.recv().await {
            if !self.sync(snapshot, edits).await {
                break;
            }
        }
    }

    async fn sync(&mut self, snapshot: fold_map::Snapshot, edits: Vec<fold_map::Edit>) -> bool {
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
        self.snapshots_tx.send(self.snapshot.clone()).await.is_ok()
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
