use std::{
    cmp::{self, Ordering},
    collections::BTreeMap,
    mem,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

use buffer::{Anchor, Bias, Edit, Point, Rope, TextSummary, ToOffset, ToPoint};
use gpui::{fonts::HighlightStyle, AppContext, ModelHandle};
use language::Buffer;
use parking_lot::Mutex;
use sum_tree::SumTree;
use util::post_inc;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct InjectionId(usize);

pub struct InjectionMap {
    buffer: ModelHandle<Buffer>,
    transforms: Mutex<SumTree<Transform>>,
    injections: SumTree<Injection>,
    injection_sites: SumTree<InjectionSite>,
    version: AtomicUsize,
    last_sync: Mutex<SyncState>,
    next_injection_id: usize,
}

pub struct Snapshot {
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

#[derive(Clone, Debug)]
pub struct InjectionProps {
    text: Rope,
    runs: Vec<(usize, HighlightStyle)>,
    disposition: Disposition,
}

#[derive(Clone, Debug)]
pub enum Disposition {
    BeforeLine,
    AfterLine,
}

#[derive(Clone, Debug)]
struct InjectionSite {
    injection_id: InjectionId,
    position: Anchor,
    disposition: Disposition,
}

#[derive(Clone, Debug)]
struct InjectionSitePosition(Anchor);

#[derive(Clone, Debug, Eq, PartialEq)]
struct InjectionSiteSummary {
    min_injection_id: InjectionId,
    max_injection_id: InjectionId,
    min_position: Anchor,
    max_position: Anchor,
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
pub struct InjectionOffset(usize);

impl sum_tree::Summary for InjectionId {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, cx: &Self::Context) {
        *self = *summary
    }
}

impl InjectionMap {
    pub fn new(buffer_handle: ModelHandle<Buffer>, cx: &AppContext) -> (Self, Snapshot) {
        let buffer = buffer_handle.read(cx);
        let this = Self {
            buffer: buffer_handle,
            injections: Default::default(),
            injection_sites: Default::default(),
            transforms: Mutex::new(SumTree::from_item(
                Transform::isomorphic(buffer.text_summary()),
                &(),
            )),
            last_sync: Mutex::new(SyncState {
                version: buffer.version(),
                parse_count: buffer.parse_count(),
                diagnostics_update_count: buffer.diagnostics_update_count(),
            }),
            version: AtomicUsize::new(0),
            next_injection_id: 0,
        };
        let (snapshot, _) = this.read(cx);
        (this, snapshot)
    }

    pub fn read(&self, cx: &AppContext) -> (Snapshot, Vec<Edit<InjectionOffset>>) {
        let edits = self.sync(cx);
        // self.check_invariants(cx);
        let snapshot = Snapshot {
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
    ) -> (InjectionMapWriter, Snapshot, Vec<Edit<InjectionOffset>>) {
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
        buffer_edits: Vec<buffer::Edit<Point>>,
        cx: &AppContext,
    ) -> Vec<Edit<InjectionOffset>> {
        let buffer = self.buffer.read(cx);
        let mut buffer_edits_iter = buffer_edits.iter().cloned().peekable();

        let mut new_transforms = SumTree::<Transform>::new();
        let mut transforms = self.transforms.lock();
        let old_max_point = transforms.summary().input.lines;
        let new_max_point = buffer.max_point();
        let mut cursor = transforms.cursor::<Point>();
        let mut injection_sites = self.injection_sites.cursor::<InjectionSitePosition>();
        let mut pending_after_injections: Vec<InjectionId> = Vec::new();

        while let Some(mut edit) = buffer_edits_iter.next() {
            dbg!(&edit);
            // Expand this edit to line boundaries.
            edit.old.start.column = 0;
            edit.old.end += Point::new(1, 0);
            edit.new.start.column = 0;
            edit.new.end += Point::new(1, 0);

            // Push any transforms preceding the edit.
            new_transforms.push_tree(cursor.slice(&edit.old.start, Bias::Left, &()), &());

            // Snap edits to row boundaries of intersecting transforms.
            loop {
                if cmp::min(edit.old.end, old_max_point) <= cursor.end(&()) {
                    cursor.seek(&edit.old.end, Bias::Left, &());
                    cursor.next(&());
                    let new_old_end = *cursor.start() + Point::new(1, 0);
                    edit.new.end += new_old_end - edit.old.end;
                    edit.old.end = new_old_end;
                }

                if buffer_edits_iter.peek().map_or(false, |next_edit| {
                    edit.old.end.row >= next_edit.old.start.row
                }) {
                    let next_edit = buffer_edits_iter.next().unwrap();
                    edit.old.end = cmp::max(edit.old.end, next_edit.old.end + Point::new(1, 0));
                    let row_delta = (next_edit.new.end.row as i32 - next_edit.new.start.row as i32)
                        - (next_edit.old.end.row as i32 - next_edit.old.start.row as i32);
                    edit.new.end.row = (edit.new.end.row as i32 + row_delta) as u32;
                } else {
                    break;
                }
            }

            dbg!(&edit);

            // Find and insert all injections on the lines spanned by the edit, interleaved with isomorphic regions
            injection_sites.seek(
                &InjectionSitePosition(buffer.anchor_before(edit.new.start)),
                Bias::Right,
                buffer,
            );
            let mut last_injection_row: Option<u32> = None;
            while let Some(site) = injection_sites.item() {
                let injection_row = site.position.to_point(buffer).row;

                if injection_row > edit.new.end.row {
                    break;
                }

                // If we've moved on to a new injection row, ensure that any pending injections with an after
                // disposition are inserted after their target row
                if let Some(last_injection_row) = last_injection_row {
                    if injection_row != last_injection_row {
                        let injection_point = Point::new(last_injection_row + 1, 0);
                        if injection_point > new_transforms.summary().input.lines {
                            let injection_offset = injection_point.to_offset(buffer);
                            new_transforms.push(
                                Transform::isomorphic(buffer.text_summary_for_range(
                                    new_transforms.summary().input.bytes..injection_offset,
                                )),
                                &(),
                            );
                        }
                        for injection_id in pending_after_injections.drain(..) {
                            new_transforms.push(
                                Transform::for_injection(
                                    self.injections.get(&injection_id, &()).unwrap(),
                                ),
                                &(),
                            )
                        }
                    }
                }

                match site.disposition {
                    Disposition::AfterLine => pending_after_injections.push(site.injection_id),
                    Disposition::BeforeLine => {
                        let injection_point = Point::new(injection_row, 0);
                        if injection_point > new_transforms.summary().input.lines {
                            let injection_offset = injection_point.to_offset(buffer);
                            new_transforms.push(
                                Transform::isomorphic(buffer.text_summary_for_range(
                                    new_transforms.summary().input.bytes..injection_offset,
                                )),
                                &(),
                            );
                        }
                        new_transforms.push(
                            Transform::for_injection(
                                self.injections.get(&site.injection_id, &()).unwrap(),
                            ),
                            &(),
                        );
                    }
                }

                last_injection_row = Some(injection_row);
            }

            if let Some(last_injection_row) = last_injection_row {
                let injection_point = Point::new(last_injection_row + 1, 0);
                if injection_point > new_transforms.summary().input.lines {
                    let injection_offset = injection_point.to_offset(buffer);
                    new_transforms.push(
                        Transform::isomorphic(buffer.text_summary_for_range(
                            new_transforms.summary().input.bytes..injection_offset,
                        )),
                        &(),
                    );
                }
                for injection_id in pending_after_injections.drain(..) {
                    new_transforms.push(
                        Transform::for_injection(self.injections.get(&injection_id, &()).unwrap()),
                        &(),
                    )
                }
            }

            let sum = new_transforms.summary();
            let new_end = cmp::min(edit.new.end, new_max_point);
            if sum.input.lines < new_end {
                let text_summary =
                    buffer.text_summary_for_range(sum.input.bytes..new_end.to_offset(buffer));
                new_transforms.push(Transform::isomorphic(text_summary), &());
            }
        }
        new_transforms.push_tree(cursor.suffix(&()), &());
        drop(cursor);

        *transforms = new_transforms;
        Vec::new()
    }
}

impl<'a> InjectionMapWriter<'a> {
    pub fn insert<'b, T, U>(
        &mut self,
        injections: T,
        cx: &AppContext,
    ) -> (Vec<InjectionId>, Snapshot, Vec<Edit<InjectionOffset>>)
    where
        T: IntoIterator<Item = (Anchor, InjectionProps)>,
    {
        let buffer = self.0.buffer.read(cx);
        let mut cursor = self.0.injection_sites.cursor::<InjectionSitePosition>();
        let mut new_sites = SumTree::new();
        let mut injection_ids = Vec::new();
        let mut edits = Vec::new();

        for (position, props) in injections {
            let point = position.to_point(buffer);
            edits.push(Edit {
                old: point..point,
                new: point..point,
            });

            let id = InjectionId(post_inc(&mut self.0.next_injection_id));
            injection_ids.push(id);
            new_sites.push_tree(
                cursor.slice(
                    &InjectionSitePosition(position.clone()),
                    Bias::Right,
                    buffer,
                ),
                buffer,
            );
            new_sites.push(
                InjectionSite {
                    injection_id: id,
                    position,
                    disposition: props.disposition,
                },
                buffer,
            );
            self.0.injections.push(
                Injection {
                    id,
                    text: props.text,
                    runs: props.runs,
                },
                &(),
            );
        }
        new_sites.push_tree(cursor.suffix(buffer), buffer);

        drop(cursor);
        self.0.injection_sites = new_sites;

        let edits = self.0.apply_edits(edits, cx);
        let snapshot = Snapshot {
            transforms: self.0.transforms.lock().clone(),
            injections: self.0.injections.clone(),
            buffer_snapshot: buffer.snapshot(),
            version: self.0.version.load(SeqCst),
        };

        (injection_ids, snapshot, edits)
    }
}

impl sum_tree::Item for Injection {
    type Summary = InjectionId;

    fn summary(&self) -> Self::Summary {
        self.id
    }
}

impl sum_tree::KeyedItem for Injection {
    type Key = InjectionId;

    fn key(&self) -> Self::Key {
        self.id
    }
}

impl sum_tree::Item for InjectionSite {
    type Summary = InjectionSiteSummary;

    fn summary(&self) -> Self::Summary {
        InjectionSiteSummary {
            min_injection_id: self.injection_id,
            max_injection_id: self.injection_id,
            min_position: self.position.clone(),
            max_position: self.position.clone(),
        }
    }
}

impl Default for InjectionSitePosition {
    fn default() -> Self {
        Self(Anchor::min())
    }
}

impl sum_tree::Summary for InjectionSiteSummary {
    type Context = buffer::Buffer;

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.min_injection_id = cmp::min(self.min_injection_id, summary.min_injection_id);
        self.max_injection_id = cmp::max(self.max_injection_id, summary.max_injection_id);
        self.max_position = summary.max_position.clone();
    }
}

impl<'a> sum_tree::Dimension<'a, InjectionSiteSummary> for InjectionSitePosition {
    fn add_summary(&mut self, summary: &'a InjectionSiteSummary, _: &buffer::Buffer) {
        self.0 = summary.max_position.clone();
    }
}

impl<'a> sum_tree::SeekTarget<'a, InjectionSiteSummary, Self> for InjectionSitePosition {
    fn cmp(&self, cursor_location: &Self, snapshot: &buffer::Buffer) -> Ordering {
        self.0.cmp(&cursor_location.0, snapshot).unwrap()
    }
}

impl Default for InjectionSiteSummary {
    fn default() -> Self {
        Self {
            min_injection_id: InjectionId(usize::MAX),
            max_injection_id: InjectionId(0),
            min_position: Anchor::max(),
            max_position: Anchor::min(),
        }
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

    fn for_injection(injection: &Injection) -> Self {
        Self {
            input: Default::default(),
            output: injection.text.summary(),
            injection_id: Some(injection.id),
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

impl<'a> sum_tree::Dimension<'a, TransformSummary> for Point {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += summary.input.lines
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InjectionOffset {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.output.bytes
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use buffer::RandomCharIter;
    use rand::prelude::*;

    #[gpui::test(iterations = 1000)]
    fn test_random(cx: &mut gpui::MutableAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(1);

        let buffer = cx.add_model(|cx| {
            let len = rng.gen_range(0..10);
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            Buffer::new(0, text, cx)
        });
        let (map, initial_snapshot) = InjectionMap::new(buffer.clone(), cx.as_ref());
        assert_eq!(
            initial_snapshot.transforms.summary().input,
            buffer.read(cx).text_summary()
        );

        for _ in 0..operations {
            log::info!("text: {:?}", buffer.read(cx).text());
            match rng.gen_range(0..=100) {
                _ => {
                    let edits = buffer.update(cx, |buffer, _| {
                        let start_version = buffer.version.clone();
                        let edit_count = rng.gen_range(1..=5);
                        buffer.randomly_edit(&mut rng, edit_count);
                        buffer
                            .edits_since::<Point>(&start_version)
                            .collect::<Vec<_>>()
                    });
                    log::info!("editing {:?}", edits);
                }
            }

            let (snapshot, edits) = map.read(cx.as_ref());
            assert_eq!(
                snapshot.transforms.summary().input,
                buffer.read(cx).text_summary()
            );
        }
    }
}
