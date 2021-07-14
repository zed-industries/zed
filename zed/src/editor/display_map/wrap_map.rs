use crate::{
    editor::{display_map::FoldMap, Buffer, TextSummary},
    sum_tree::{self, SumTree},
    time,
};
use gpui::{AppContext, Entity, ModelContext, ModelHandle, Task};
use parking_lot::Mutex;
use postage::{
    mpsc,
    prelude::{Sink, Stream},
    watch,
};

#[derive(Clone)]
struct Snapshot {
    transforms: SumTree<Transform>,
    version: time::Global,
}

struct State {
    snapshot: Snapshot,
    interpolated_version: time::Global,
}

struct WrapMap {
    buffer: ModelHandle<Buffer>,
    fold_map: FoldMap,
    state: Mutex<State>,
    background_snapshots: watch::Receiver<Snapshot>,
    _background_task: Task<()>,
}

impl Entity for WrapMap {
    type Event = ();
}

impl WrapMap {
    fn new(buffer_handle: ModelHandle<Buffer>, cx: &mut ModelContext<Self>) -> Self {
        let buffer = buffer_handle.read(cx).clone();
        let version = buffer.version();
        let snapshot = Snapshot {
            transforms: SumTree::from_item(
                Transform {
                    summary: TransformSummary {
                        buffer: buffer.text_summary(),
                        display: buffer.text_summary(),
                    },
                    display_text: None,
                },
                &(),
            ),
            version: version.clone(),
        };
        let (background_snapshots_tx, background_snapshots_rx) =
            watch::channel_with(snapshot.clone());
        let (buffers_tx, buffers_rx) = mpsc::channel(32);
        cx.observe(&buffer_handle, move |_, buffer, cx| {
            let mut buffers_tx = buffers_tx.clone();
            let buffer = buffer.read(cx).clone();
            cx.spawn_weak(|_, _| async move {
                let _ = buffers_tx.send(buffer).await;
            })
            .detach();
        });
        let background_task = cx.background().spawn(async move {
            let mut wrapper = BackgroundWrapper::new(buffers_rx, background_snapshots_tx);
            wrapper.run(buffer).await;
        });

        Self {
            buffer: buffer_handle.clone(),
            fold_map: FoldMap::new(buffer_handle, cx.as_ref()),
            state: Mutex::new(State {
                snapshot,
                interpolated_version: version,
            }),
            background_snapshots: background_snapshots_rx,
            _background_task: background_task,
        }
    }
}

struct BackgroundWrapper {
    buffers_rx: mpsc::Receiver<Buffer>,
    snapshots_tx: watch::Sender<Snapshot>,
    snapshot: Snapshot,
}

impl BackgroundWrapper {
    fn new(buffers_rx: mpsc::Receiver<Buffer>, snapshots_tx: watch::Sender<Snapshot>) -> Self {
        Self {
            buffers_rx,
            snapshots_tx,
            snapshot: Snapshot {
                transforms: Default::default(),
                version: Default::default(),
            },
        }
    }

    async fn run(&mut self, buffer: Buffer) {
        if !self.sync(buffer).await {
            return;
        }

        while let Some(buffer) = self.buffers_rx.recv().await {
            if !self.sync(buffer).await {
                break;
            }
        }
    }

    async fn sync(&mut self, buffer: Buffer) -> bool {
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
    display: TextSummary,
    buffer: TextSummary,
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.buffer += &other.buffer;
        self.display += &other.display;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for TransformSummary {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        sum_tree::Summary::add_summary(self, summary, &());
    }
}
