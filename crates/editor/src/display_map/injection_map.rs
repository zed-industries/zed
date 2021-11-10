use std::{
    cmp, mem,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

use buffer::{Anchor, Bias, Edit, Point, Rope, TextSummary};
use gpui::{fonts::HighlightStyle, AppContext, ModelHandle};
use language::Buffer;
use parking_lot::Mutex;
use sum_tree::SumTree;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct InjectionId(usize);

pub struct InjectionMap {
    buffer: ModelHandle<Buffer>,
    transforms: Mutex<SumTree<Transform>>,
    injections: SumTree<Injection>,
    injection_contents: SumTree<InjectionContent>,
    version: AtomicUsize,
    last_sync: Mutex<SyncState>,
}

pub struct InjectionSnapshot {
    transforms: SumTree<Transform>,
    injection_contents: SumTree<InjectionContent>,
    buffer_snapshot: language::Snapshot,
    pub version: usize,
}

pub struct InjectionMapWriter<'a>(&'a mut InjectionMap);

#[derive(Clone)]
struct SyncState {
    version: clock::Global,
    parse_count: usize,
    diagnostics_update_count: usize,
}

#[derive(Clone, Debug)]
struct Injection {
    id: InjectionId,
    position: Anchor,
    is_block: bool,
}

#[derive(Clone, Debug)]
struct InjectionSummary {
    min_id: InjectionId,
    max_id: InjectionId,
    min_position: Anchor,
    max_position: Anchor,
}

#[derive(Clone, Debug)]
struct InjectionContent {
    injection_id: InjectionId,
    runs: Vec<(usize, HighlightStyle)>,
    text: Rope,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Transform {
    summary: TransformSummary,
    injection_id: Option<InjectionId>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TransformSummary {
    output: TextSummary,
    input: TextSummary,
}

#[derive(Copy, Clone)]
struct InjectionOffset(usize);

impl sum_tree::Summary for InjectionId {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, cx: &Self::Context) {
        *self = *summary
    }
}

impl InjectionMap {
    pub fn read(&self, cx: &AppContext) -> (InjectionSnapshot, Vec<Edit<InjectionOffset>>) {
        let edits = self.sync(cx);
        // self.check_invariants(cx);
        let snapshot = InjectionSnapshot {
            transforms: self.transforms.lock().clone(),
            injection_contents: self.injection_contents.clone(),
            buffer_snapshot: self.buffer.read(cx).snapshot(),
            version: self.version.load(SeqCst),
        };
        (snapshot, edits)
    }

    pub fn write(
        &mut self,
        cx: &AppContext,
    ) -> (
        InjectionMapWriter,
        InjectionSnapshot,
        Vec<Edit<InjectionOffset>>,
    ) {
        let (snapshot, edits) = self.read(cx);
        (InjectionMapWriter(self), snapshot, edits)
    }

    fn sync(&self, cx: &AppContext) -> Vec<Edit<InjectionOffset>> {
        let buffer = self.buffer.read(cx);
        let last_sync = mem::replace(
            &mut *self.last_sync.lock(),
            SyncState {
                version: buffer.version(),
                parse_count: buffer.parse_count(),
                diagnostics_update_count: buffer.diagnostics_update_count(),
            },
        );
        let edits = buffer
            .edits_since(&last_sync.version)
            .map(Into::into)
            .collect::<Vec<_>>();
        if edits.is_empty() {
            if last_sync.parse_count != buffer.parse_count()
                || last_sync.diagnostics_update_count != buffer.diagnostics_update_count()
            {
                self.version.fetch_add(1, SeqCst);
            }
            Vec::new()
        } else {
            self.apply_edits(edits, cx)
        }
    }

    fn apply_edits(
        &self,
        buffer_edits: Vec<buffer::Edit<usize>>,
        cx: &AppContext,
    ) -> Vec<Edit<InjectionOffset>> {
        let buffer = self.buffer.read(cx).snapshot();
        let mut buffer_edits_iter = buffer_edits.iter().cloned().peekable();

        let mut new_transforms = SumTree::<Transform>::new();
        let mut transforms = self.transforms.lock();
        let mut cursor = transforms.cursor::<usize>();

        while let Some(mut edit) = buffer_edits_iter.next() {
            new_transforms.push_tree(cursor.slice(&edit.old.start, Bias::Left, &()), &());
            edit.new.start -= edit.old.start - cursor.start();
            edit.old.start = *cursor.start();

            cursor.seek(&edit.old.end, Bias::Right, &());
            cursor.next(&());

            let mut delta = edit.new.len() as isize - edit.old.len() as isize;
            loop {
                edit.old.end = *cursor.start();

                if let Some(next_edit) = buffer_edits_iter.peek() {
                    if next_edit.old.start > edit.old.end {
                        break;
                    }

                    let next_edit = buffer_edits_iter.next().unwrap();
                    delta += next_edit.new.len() as isize - next_edit.old.len() as isize;

                    if next_edit.old.end >= edit.old.end {
                        edit.old.end = next_edit.old.end;
                        cursor.seek(&edit.old.end, Bias::Right, &());
                        cursor.next(&());
                    }
                } else {
                    break;
                }
            }

            edit.new.end = ((edit.new.start + edit.old.len()) as isize + delta) as usize;

            let anchor = buffer.anchor_before(edit.new.start);
            let mut injections_cursor = self.injections.cursor::<Anchor>();
            // folds_cursor.seek(&Fold(anchor..Anchor::max()), Bias::Left, &buffer);
        }

        todo!()
    }
}

impl sum_tree::Item for Injection {
    type Summary = InjectionSummary;

    fn summary(&self) -> Self::Summary {
        InjectionSummary {
            min_id: self.id,
            max_id: self.id,
            min_position: self.position.clone(),
            max_position: self.position.clone(),
        }
    }
}

impl sum_tree::Summary for InjectionSummary {
    type Context = buffer::Snapshot;

    fn add_summary(&mut self, summary: &Self, _: &buffer::Snapshot) {
        self.max_position = summary.max_position.clone();
        self.min_id = cmp::min(self.min_id, summary.min_id);
        self.max_id = cmp::max(self.max_id, summary.max_id);
    }
}

impl Default for InjectionSummary {
    fn default() -> Self {
        Self {
            min_id: InjectionId(0),
            max_id: InjectionId(usize::MAX),
            min_position: Anchor::max(),
            max_position: Anchor::min(),
        }
    }
}

impl sum_tree::Item for InjectionContent {
    type Summary = InjectionId;

    fn summary(&self) -> Self::Summary {
        self.injection_id
    }
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self) -> Self::Summary {
        self.summary.clone()
    }
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.input += &other.input;
        self.output += &other.output;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for usize {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += summary.input.bytes
    }
}
