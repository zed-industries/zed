use std::{
    cmp::Ordering,
    mem,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

use buffer::{rope::TextDimension, Anchor, Bias, Edit, Rope, TextSummary, ToOffset};
use gpui::{fonts::HighlightStyle, AppContext, ModelHandle};
use language::Buffer;
use parking_lot::Mutex;
use sum_tree::{SeekTarget, SumTree};
use util::post_inc;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct InjectionId(usize);

pub struct InjectionMap {
    buffer: ModelHandle<Buffer>,
    transforms: Mutex<SumTree<Transform>>,
    injections: SumTree<Injection>,
    version: AtomicUsize,
    last_sync: Mutex<SyncState>,
    next_injection_id: usize,
}

pub struct InjectionSnapshot {
    transforms: SumTree<Transform>,
    injections: SumTree<Injection>,
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
struct InjectionSummary {
    min_id: InjectionId,
    max_id: InjectionId,
    min_position: Anchor,
    max_position: Anchor,
}

#[derive(Clone, Debug)]
struct Injection {
    id: InjectionId,
    text: Rope,
    runs: Vec<(usize, HighlightStyle)>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Transform {
    input: TextSummary,
    output: TextSummary,
    injection_id: Option<InjectionId>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TransformSummary {
    input: TextSummary,
    output: TextSummary,
    min_injection_id: InjectionId,
    max_injection_id: InjectionId,
}

#[derive(Copy, Clone, Debug, Default)]
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
            injections: self.injections.clone(),
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
            new_transforms.push_tree(cursor.slice(&edit.old.start, Bias::Right, &()), &());
            edit.new.start -= edit.old.start - cursor.start();
            edit.old.start = *cursor.start();

            cursor.seek(&edit.old.end, Bias::Left, &());
            cursor.next(&());

            let mut delta = edit.new.len() as isize - edit.old.len() as isize;
            loop {
                edit.old.end = *cursor.start();

                if let Some(next_edit) = buffer_edits_iter.peek() {
                    if next_edit.old.start >= edit.old.end {
                        break;
                    }

                    let next_edit = buffer_edits_iter.next().unwrap();
                    delta += next_edit.new.len() as isize - next_edit.old.len() as isize;

                    if next_edit.old.end >= edit.old.end {
                        edit.old.end = next_edit.old.end;
                        cursor.seek(&edit.old.end, Bias::Left, &());
                        cursor.next(&());
                    }
                } else {
                    break;
                }
            }

            edit.new.end = ((edit.new.start + edit.old.len()) as isize + delta) as usize;

            if !edit.new.is_empty() {
                let text_summary = buffer.text_summary_for_range(edit.new.start..edit.new.end);
                new_transforms.push(
                    Transform {
                        input: text_summary.clone(),
                        output: text_summary,
                        injection_id: None,
                    },
                    &(),
                );
            }
        }
        new_transforms.push_tree(cursor.suffix(&()), &());
        drop(cursor);

        let injection_edits = {
            let mut old_transforms = transforms.cursor::<(usize, InjectionOffset)>();
            let mut new_transforms = new_transforms.cursor::<(usize, InjectionOffset)>();

            buffer_edits
                .into_iter()
                .map(|edit| {
                    old_transforms.seek(&edit.old.start, Bias::Right, &());
                    let old_start =
                        old_transforms.start().1 .0 + (edit.old.start - old_transforms.start().0);

                    old_transforms.seek_forward(&edit.old.end, Bias::Left, &());
                    let old_end =
                        old_transforms.start().1 .0 + (edit.old.end - old_transforms.start().0);

                    new_transforms.seek(&edit.new.start, Bias::Right, &());
                    let new_start =
                        new_transforms.start().1 .0 + (edit.new.start - new_transforms.start().0);

                    new_transforms.seek_forward(&edit.new.end, Bias::Left, &());
                    let new_end =
                        new_transforms.start().1 .0 + (edit.new.end - new_transforms.start().0);

                    Edit {
                        old: InjectionOffset(old_start)..InjectionOffset(old_end),
                        new: InjectionOffset(new_start)..InjectionOffset(new_end),
                    }
                })
                .collect()
        };

        *transforms = new_transforms;
        injection_edits
    }
}

impl<'a> InjectionMapWriter<'a> {
    pub fn insert<'b, T, U>(
        &mut self,
        injections: T,
        cx: &AppContext,
    ) -> (
        Vec<InjectionId>,
        InjectionSnapshot,
        Vec<Edit<InjectionOffset>>,
    )
    where
        T: IntoIterator<Item = (U, &'b str, Vec<(usize, HighlightStyle)>)>,
        U: ToOffset,
    {
        let buffer = self.0.buffer.read(cx);
        let mut injections = injections
            .into_iter()
            .map(|(position, text, runs)| (position.to_offset(buffer), text, runs))
            .peekable();
        let mut edits = Vec::new();
        let mut injection_ids = Vec::new();
        let mut new_transforms = SumTree::new();
        let mut transforms = self.0.transforms.lock();
        let mut cursor = transforms.cursor::<usize>();

        while let Some((injection_offset, text, runs)) = injections.next() {
            new_transforms.push_tree(cursor.slice(&injection_offset, Bias::Right, &()), &());
            let new_transforms_end = new_transforms.summary().input.bytes;
            if injection_offset > new_transforms_end {
                new_transforms.push(
                    Transform::isomorphic(
                        buffer.text_summary_for_range(new_transforms_end..injection_offset),
                    ),
                    &(),
                );
            }

            let injection = Injection {
                id: InjectionId(post_inc(&mut self.0.next_injection_id)),
                runs,
                text: text.into(),
            };
            new_transforms.push(
                Transform {
                    input: Default::default(),
                    output: injection.text.summary(),
                    injection_id: Some(injection.id),
                },
                &(),
            );
            self.0.injections.push(injection, &());

            if let Some((next_injection_offset, _, _)) = injections.peek() {
                let old_transform_end = cursor.end(&());
                if *next_injection_offset > old_transform_end {
                    new_transforms.push(
                        Transform::isomorphic(
                            buffer.text_summary_for_range(new_transforms_end..old_transform_end),
                        ),
                        &(),
                    );
                    cursor.next(&());
                }
            }
        }

        (injection_ids, todo!(), edits)
    }
}

impl sum_tree::Item for Injection {
    type Summary = InjectionId;

    fn summary(&self) -> Self::Summary {
        self.id
    }
}

impl Transform {
    fn isomorphic(text_summary: TextSummary) -> Self {
        Self {
            input: text_summary.clone(),
            output: text_summary,
            injection_id: None,
        }
    }
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self) -> Self::Summary {
        let min_injection_id;
        let max_injection_id;
        if let Some(id) = self.injection_id {
            min_injection_id = id;
            max_injection_id = id;
        } else {
            min_injection_id = InjectionId(usize::MAX);
            max_injection_id = InjectionId(0);
        }

        TransformSummary {
            input: self.input.clone(),
            output: self.output.clone(),
            min_injection_id,
            max_injection_id,
        }
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

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InjectionOffset {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.output.bytes
    }
}
