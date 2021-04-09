mod anchor;
mod point;
mod text;

pub use anchor::*;
use futures_core::future::LocalBoxFuture;
pub use point::*;
use seahash::SeaHasher;
pub use text::*;

use crate::{
    operation_queue::{self, OperationQueue},
    sum_tree::{self, Cursor, FilterCursor, SeekBias, SumTree},
    time::{self, ReplicaId},
    util::RandomCharIter,
    worktree::FileHandle,
};
use anyhow::{anyhow, Result};
use gpui::{AppContext, Entity, ModelContext};
use lazy_static::lazy_static;
use rand::prelude::*;
use std::{
    cmp::{self, Ordering},
    hash::BuildHasher,
    iter::{self, Iterator},
    mem,
    ops::{AddAssign, Range},
    path::PathBuf,
    str,
    sync::Arc,
};

pub type SelectionSetId = time::Lamport;
pub type SelectionsVersion = usize;

#[derive(Clone, Default)]
struct DeterministicState;

impl BuildHasher for DeterministicState {
    type Hasher = SeaHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SeaHasher::new()
    }
}

#[cfg(test)]
type HashMap<K, V> = std::collections::HashMap<K, V, DeterministicState>;

#[cfg(test)]
type HashSet<T> = std::collections::HashSet<T, DeterministicState>;

#[cfg(not(test))]
type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(not(test))]
type HashSet<T> = std::collections::HashSet<T>;

#[derive(Clone, Default, Debug)]
struct UndoMap(HashMap<time::Local, Vec<UndoOperation>>);

impl UndoMap {
    fn insert(&mut self, undo: UndoOperation) {
        self.0.entry(undo.edit_id).or_default().push(undo);
    }

    fn is_undone(&self, edit_id: time::Local) -> bool {
        self.undo_count(edit_id) % 2 == 1
    }

    fn was_undone(&self, edit_id: time::Local, version: &time::Global) -> bool {
        let undo_count = self
            .0
            .get(&edit_id)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|undo| version.observed(undo.id))
            .map(|undo| undo.count)
            .max()
            .unwrap_or(0);
        undo_count % 2 == 1
    }

    fn undo_count(&self, edit_id: time::Local) -> u32 {
        self.0
            .get(&edit_id)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|undo| undo.count)
            .max()
            .unwrap_or(0)
    }
}

pub struct Buffer {
    file: Option<FileHandle>,
    fragments: SumTree<Fragment>,
    insertion_splits: HashMap<time::Local, SumTree<InsertionSplit>>,
    edit_ops: HashMap<time::Local, EditOperation>,
    pub version: time::Global,
    saved_version: time::Global,
    last_edit: time::Local,
    undo_map: UndoMap,
    selections: HashMap<SelectionSetId, Vec<Selection>>,
    pub selections_last_update: SelectionsVersion,
    deferred_ops: OperationQueue<Operation>,
    deferred_replicas: HashSet<ReplicaId>,
    replica_id: ReplicaId,
    local_clock: time::Local,
    lamport_clock: time::Lamport,
}

pub struct Snapshot {
    fragments: SumTree<Fragment>,
}

#[derive(Clone)]
pub struct History {
    pub base_text: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    pub start: Anchor,
    pub end: Anchor,
    pub reversed: bool,
}

#[derive(Clone)]
pub struct CharIter<'a> {
    fragments_cursor: Cursor<'a, Fragment, usize, usize>,
    fragment_chars: str::Chars<'a>,
}

#[derive(Clone)]
pub struct FragmentIter<'a> {
    cursor: Cursor<'a, Fragment, usize, usize>,
    started: bool,
}

struct Edits<'a, F: Fn(&FragmentSummary) -> bool> {
    cursor: FilterCursor<'a, F, Fragment, usize>,
    undos: &'a UndoMap,
    since: time::Global,
    delta: isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edit {
    pub old_range: Range<usize>,
    pub new_range: Range<usize>,
}

impl Edit {
    pub fn delta(&self) -> isize {
        (self.new_range.end - self.new_range.start) as isize
            - (self.old_range.end - self.old_range.start) as isize
    }

    pub fn old_extent(&self) -> usize {
        self.old_range.end - self.old_range.start
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Insertion {
    id: time::Local,
    parent_id: time::Local,
    offset_in_parent: usize,
    text: Text,
    lamport_timestamp: time::Lamport,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Fragment {
    id: FragmentId,
    insertion: Insertion,
    text: Text,
    deletions: HashSet<time::Local>,
    max_undos: time::Global,
    visible: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FragmentSummary {
    text_summary: TextSummary,
    max_fragment_id: FragmentId,
    max_version: time::Global,
}

#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
struct FragmentExtent {
    chars: usize,
    lines: Point,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct InsertionSplit {
    extent: usize,
    fragment_id: FragmentId,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct InsertionSplitSummary {
    extent: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Edit {
        edit: EditOperation,
        lamport_timestamp: time::Lamport,
    },
    Undo {
        undo: UndoOperation,
        lamport_timestamp: time::Lamport,
    },
    UpdateSelections {
        set_id: SelectionSetId,
        selections: Option<Vec<Selection>>,
        lamport_timestamp: time::Lamport,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditOperation {
    id: time::Local,
    start_id: time::Local,
    start_offset: usize,
    end_id: time::Local,
    end_offset: usize,
    version_in_range: time::Global,
    new_text: Option<Text>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UndoOperation {
    id: time::Local,
    edit_id: time::Local,
    count: u32,
}

impl Buffer {
    pub fn new<T: Into<String>>(replica_id: ReplicaId, base_text: T) -> Self {
        Self::build(replica_id, None, base_text.into())
    }

    pub fn from_history(replica_id: ReplicaId, file: FileHandle, history: History) -> Self {
        Self::build(replica_id, Some(file), history.base_text)
    }

    fn build(replica_id: ReplicaId, file: Option<FileHandle>, base_text: String) -> Self {
        let mut insertion_splits = HashMap::default();
        let mut fragments = SumTree::new();

        let base_insertion = Insertion {
            id: time::Local::default(),
            parent_id: time::Local::default(),
            offset_in_parent: 0,
            text: base_text.into(),
            lamport_timestamp: time::Lamport::default(),
        };

        insertion_splits.insert(
            base_insertion.id,
            SumTree::from_item(InsertionSplit {
                fragment_id: FragmentId::min_value().clone(),
                extent: 0,
            }),
        );
        fragments.push(Fragment {
            id: FragmentId::min_value().clone(),
            insertion: base_insertion.clone(),
            text: base_insertion.text.slice(0..0),
            deletions: Default::default(),
            max_undos: Default::default(),
            visible: true,
        });

        if base_insertion.text.len() > 0 {
            let base_fragment_id =
                FragmentId::between(&FragmentId::min_value(), &FragmentId::max_value());

            insertion_splits
                .get_mut(&base_insertion.id)
                .unwrap()
                .push(InsertionSplit {
                    fragment_id: base_fragment_id.clone(),
                    extent: base_insertion.text.len(),
                });
            fragments.push(Fragment {
                id: base_fragment_id,
                text: base_insertion.text.clone(),
                insertion: base_insertion,
                deletions: Default::default(),
                max_undos: Default::default(),
                visible: true,
            });
        }

        Self {
            file,
            fragments,
            insertion_splits,
            edit_ops: HashMap::default(),
            version: time::Global::new(),
            saved_version: time::Global::new(),
            last_edit: time::Local::default(),
            undo_map: Default::default(),
            selections: HashMap::default(),
            selections_last_update: 0,
            deferred_ops: OperationQueue::new(),
            deferred_replicas: HashSet::default(),
            replica_id,
            local_clock: time::Local::new(replica_id),
            lamport_clock: time::Lamport::new(replica_id),
        }
    }

    pub fn path(&self, app: &AppContext) -> Option<PathBuf> {
        self.file.as_ref().map(|file| file.path(app))
    }

    pub fn entry_id(&self) -> Option<(usize, usize)> {
        self.file.as_ref().map(|file| file.entry_id())
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            fragments: self.fragments.clone(),
        }
    }

    pub fn save(&mut self, ctx: &mut ModelContext<Self>) -> LocalBoxFuture<'static, Result<()>> {
        if let Some(file) = &self.file {
            let snapshot = self.snapshot();
            let version = self.version.clone();
            let save_task = file.save(snapshot, ctx.app());
            let task = ctx.spawn(save_task, |me, save_result, ctx| {
                if save_result.is_ok() {
                    me.did_save(version, ctx);
                }
                save_result
            });
            Box::pin(task)
        } else {
            Box::pin(async { Ok(()) })
        }
    }

    fn did_save(&mut self, version: time::Global, ctx: &mut ModelContext<Buffer>) {
        self.saved_version = version;
        ctx.emit(Event::Saved);
    }

    pub fn is_dirty(&self) -> bool {
        self.version > self.saved_version
    }

    pub fn version(&self) -> time::Global {
        self.version.clone()
    }

    pub fn text_summary(&self) -> TextSummary {
        self.fragments.extent::<TextSummary>()
    }

    pub fn text_summary_for_range(&self, range: Range<usize>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self.fragments.cursor::<usize, usize>();
        cursor.seek(&range.start, SeekBias::Right);

        if let Some(fragment) = cursor.item() {
            let summary_start = cmp::max(*cursor.start(), range.start) - cursor.start();
            let summary_end = cmp::min(range.end - cursor.start(), fragment.len());
            summary += &fragment.text.slice(summary_start..summary_end).summary();
            cursor.next();
        }

        if range.end > *cursor.start() {
            summary += &cursor.summary::<TextSummary>(&range.end, SeekBias::Right);

            if let Some(fragment) = cursor.item() {
                let summary_start = cmp::max(*cursor.start(), range.start) - cursor.start();
                let summary_end = cmp::min(range.end - cursor.start(), fragment.len());
                summary += &fragment.text.slice(summary_start..summary_end).summary();
            }
        }

        summary
    }

    pub fn len(&self) -> usize {
        self.fragments.extent::<usize>()
    }

    pub fn line_len(&self, row: u32) -> Result<u32> {
        let row_start_offset = Point::new(row, 0).to_offset(self)?;
        let row_end_offset = if row >= self.max_point().row {
            self.len()
        } else {
            Point::new(row + 1, 0).to_offset(self)? - 1
        };

        Ok((row_end_offset - row_start_offset) as u32)
    }

    pub fn rightmost_point(&self) -> Point {
        self.fragments.summary().text_summary.rightmost_point
    }

    pub fn rightmost_point_in_range(&self, range: Range<usize>) -> Point {
        let mut summary = TextSummary::default();

        let mut cursor = self.fragments.cursor::<usize, usize>();
        cursor.seek(&range.start, SeekBias::Right);

        if let Some(fragment) = cursor.item() {
            let summary_start = cmp::max(*cursor.start(), range.start) - cursor.start();
            let summary_end = cmp::min(range.end - cursor.start(), fragment.len());
            summary += &fragment.text.slice(summary_start..summary_end).summary();
            cursor.next();
        }

        if range.end > *cursor.start() {
            summary += &cursor.summary::<TextSummary>(&range.end, SeekBias::Right);

            if let Some(fragment) = cursor.item() {
                let summary_start = cmp::max(*cursor.start(), range.start) - cursor.start();
                let summary_end = cmp::min(range.end - cursor.start(), fragment.len());
                summary += &fragment.text.slice(summary_start..summary_end).summary();
            }
        }

        summary.rightmost_point
    }

    pub fn max_point(&self) -> Point {
        self.fragments.extent()
    }

    pub fn line(&self, row: u32) -> Result<String> {
        Ok(self
            .chars_at(Point::new(row, 0))?
            .take_while(|c| *c != '\n')
            .collect())
    }

    pub fn text(&self) -> String {
        self.chars().collect()
    }

    pub fn text_for_range<T: ToOffset>(&self, range: Range<T>) -> Result<String> {
        let start = range.start.to_offset(self)?;
        let end = range.end.to_offset(self)?;
        Ok(self.chars_at(start)?.take(end - start).collect())
    }

    pub fn chars(&self) -> CharIter {
        self.chars_at(0).unwrap()
    }

    pub fn chars_at<T: ToOffset>(&self, position: T) -> Result<CharIter> {
        let offset = position.to_offset(self)?;
        Ok(CharIter::new(&self.fragments, offset))
    }

    pub fn selections_changed_since(&self, since: SelectionsVersion) -> bool {
        self.selections_last_update != since
    }

    pub fn edits_since<'a>(&'a self, since: time::Global) -> impl 'a + Iterator<Item = Edit> {
        let since_2 = since.clone();
        let cursor = self
            .fragments
            .filter(move |summary| summary.max_version.changed_since(&since_2));

        Edits {
            cursor,
            undos: &self.undo_map,
            since,
            delta: 0,
        }
    }

    pub fn deferred_ops_len(&self) -> usize {
        self.deferred_ops.len()
    }

    pub fn edit<I, S, T>(
        &mut self,
        old_ranges: I,
        new_text: T,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> Result<Vec<Operation>>
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<Text>,
    {
        let new_text = new_text.into();
        let new_text = if new_text.len() > 0 {
            Some(new_text)
        } else {
            None
        };

        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();
        let old_ranges = old_ranges
            .into_iter()
            .map(|range| Ok(range.start.to_offset(self)?..range.end.to_offset(self)?))
            .collect::<Result<Vec<Range<usize>>>>()?;

        let ops = self.splice_fragments(
            old_ranges
                .into_iter()
                .filter(|old_range| new_text.is_some() || old_range.end > old_range.start),
            new_text.clone(),
        );

        for op in &ops {
            if let Operation::Edit { edit, .. } = op {
                self.edit_ops.insert(edit.id, edit.clone());
            }
        }

        if let Some(op) = ops.last() {
            if let Some(ctx) = ctx {
                ctx.notify();
                let changes = self.edits_since(old_version).collect::<Vec<_>>();
                if !changes.is_empty() {
                    self.did_edit(changes, was_dirty, ctx);
                }
            }

            if let Operation::Edit { edit, .. } = op {
                self.last_edit = edit.id;
                self.version.observe(edit.id);
            } else {
                unreachable!()
            }
        }

        Ok(ops)
    }

    fn did_edit(&self, changes: Vec<Edit>, was_dirty: bool, ctx: &mut ModelContext<Self>) {
        ctx.emit(Event::Edited(changes));
        if !was_dirty {
            ctx.emit(Event::Dirtied);
        }
    }

    pub fn simulate_typing<T: Rng>(&mut self, rng: &mut T) {
        let end = rng.gen_range(0..self.len() + 1);
        let start = rng.gen_range(0..end + 1);
        let mut range = start..end;

        let new_text_len = rng.gen_range(0..100);
        let new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();

        for char in new_text.chars() {
            self.edit(Some(range.clone()), char.to_string().as_str(), None)
                .unwrap();
            range = range.end + 1..range.end + 1;
        }
    }

    pub fn randomly_edit<T>(
        &mut self,
        rng: &mut T,
        old_range_count: usize,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> (Vec<Range<usize>>, String, Vec<Operation>)
    where
        T: Rng,
    {
        let mut old_ranges: Vec<Range<usize>> = Vec::new();
        for _ in 0..old_range_count {
            let last_end = old_ranges.last().map_or(0, |last_range| last_range.end + 1);
            if last_end > self.len() {
                break;
            }
            let end = rng.gen_range(last_end..self.len() + 1);
            let start = rng.gen_range(last_end..end + 1);
            old_ranges.push(start..end);
        }
        let new_text_len = rng.gen_range(0..10);
        let new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();

        let operations = self
            .edit(old_ranges.iter().cloned(), new_text.as_str(), ctx)
            .unwrap();

        (old_ranges, new_text, operations)
    }

    pub fn add_selection_set<I>(&mut self, ranges: I) -> Result<(SelectionSetId, Operation)>
    where
        I: IntoIterator<Item = Range<Point>>,
    {
        let selections = self.selections_from_ranges(ranges)?;
        let lamport_timestamp = self.lamport_clock.tick();
        self.selections
            .insert(lamport_timestamp, selections.clone());
        self.selections_last_update += 1;

        Ok((
            lamport_timestamp,
            Operation::UpdateSelections {
                set_id: lamport_timestamp,
                selections: Some(selections),
                lamport_timestamp,
            },
        ))
    }

    pub fn replace_selection_set<I>(
        &mut self,
        set_id: SelectionSetId,
        ranges: I,
    ) -> Result<Operation>
    where
        I: IntoIterator<Item = Range<Point>>,
    {
        self.selections
            .remove(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;

        let mut selections = self.selections_from_ranges(ranges)?;
        self.merge_selections(&mut selections);
        self.selections.insert(set_id, selections.clone());

        let lamport_timestamp = self.lamport_clock.tick();
        self.selections_last_update += 1;

        Ok(Operation::UpdateSelections {
            set_id,
            selections: Some(selections),
            lamport_timestamp,
        })
    }

    pub fn remove_selection_set(&mut self, set_id: SelectionSetId) -> Result<Operation> {
        self.selections
            .remove(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;
        let lamport_timestamp = self.lamport_clock.tick();
        self.selections_last_update += 1;
        Ok(Operation::UpdateSelections {
            set_id,
            selections: None,
            lamport_timestamp,
        })
    }

    pub fn selection_ranges<'a>(
        &'a self,
        set_id: SelectionSetId,
    ) -> Result<impl Iterator<Item = Range<Point>> + 'a> {
        let selections = self
            .selections
            .get(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;
        Ok(selections.iter().map(move |selection| {
            let start = selection.start.to_point(self).unwrap();
            let end = selection.end.to_point(self).unwrap();
            if selection.reversed {
                end..start
            } else {
                start..end
            }
        }))
    }

    pub fn all_selections(&self) -> impl Iterator<Item = (&SelectionSetId, &Vec<Selection>)> {
        self.selections.iter()
    }

    pub fn all_selection_ranges<'a>(
        &'a self,
    ) -> impl 'a + Iterator<Item = (SelectionSetId, Vec<Range<Point>>)> {
        self.selections
            .keys()
            .map(move |set_id| (*set_id, self.selection_ranges(*set_id).unwrap().collect()))
    }

    fn merge_selections(&mut self, selections: &mut Vec<Selection>) {
        let mut new_selections = Vec::with_capacity(selections.len());
        {
            let mut old_selections = selections.drain(..);
            if let Some(mut prev_selection) = old_selections.next() {
                for selection in old_selections {
                    if prev_selection.end.cmp(&selection.start, self).unwrap() >= Ordering::Equal {
                        if selection.end.cmp(&prev_selection.end, self).unwrap() > Ordering::Equal {
                            prev_selection.end = selection.end;
                        }
                    } else {
                        new_selections.push(mem::replace(&mut prev_selection, selection));
                    }
                }
                new_selections.push(prev_selection);
            }
        }
        *selections = new_selections;
    }

    fn selections_from_ranges<I>(&self, ranges: I) -> Result<Vec<Selection>>
    where
        I: IntoIterator<Item = Range<Point>>,
    {
        let mut ranges = ranges.into_iter().collect::<Vec<_>>();
        ranges.sort_unstable_by_key(|range| range.start);

        let mut selections = Vec::with_capacity(ranges.len());
        for range in ranges {
            if range.start > range.end {
                selections.push(Selection {
                    start: self.anchor_before(range.end)?,
                    end: self.anchor_before(range.start)?,
                    reversed: true,
                });
            } else {
                selections.push(Selection {
                    start: self.anchor_after(range.start)?,
                    end: self.anchor_before(range.end)?,
                    reversed: false,
                });
            }
        }
        Ok(selections)
    }

    pub fn apply_ops<I: IntoIterator<Item = Operation>>(
        &mut self,
        ops: I,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> Result<()> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let mut deferred_ops = Vec::new();
        for op in ops {
            if self.can_apply_op(&op) {
                self.apply_op(op)?;
            } else {
                self.deferred_replicas.insert(op.replica_id());
                deferred_ops.push(op);
            }
        }
        self.deferred_ops.insert(deferred_ops);
        self.flush_deferred_ops()?;

        if let Some(ctx) = ctx {
            ctx.notify();
            let changes = self.edits_since(old_version).collect::<Vec<_>>();
            if !changes.is_empty() {
                self.did_edit(changes, was_dirty, ctx);
            }
        }

        Ok(())
    }

    fn apply_op(&mut self, op: Operation) -> Result<()> {
        match op {
            Operation::Edit {
                edit,
                lamport_timestamp,
                ..
            } => {
                if !self.version.observed(edit.id) {
                    self.apply_edit(
                        edit.start_id,
                        edit.start_offset,
                        edit.end_id,
                        edit.end_offset,
                        edit.new_text.as_ref().cloned(),
                        &edit.version_in_range,
                        edit.id,
                        lamport_timestamp,
                    )?;
                    self.version.observe(edit.id);
                    self.edit_ops.insert(edit.id, edit);
                }
            }
            Operation::Undo {
                undo,
                lamport_timestamp,
            } => {
                if !self.version.observed(undo.id) {
                    self.apply_undo(undo)?;
                    self.version.observe(undo.id);
                    self.lamport_clock.observe(lamport_timestamp);
                }
            }
            Operation::UpdateSelections {
                set_id,
                selections,
                lamport_timestamp,
            } => {
                if let Some(selections) = selections {
                    self.selections.insert(set_id, selections);
                } else {
                    self.selections.remove(&set_id);
                }
                self.lamport_clock.observe(lamport_timestamp);
                self.selections_last_update += 1;
            }
        }
        Ok(())
    }

    fn apply_edit(
        &mut self,
        start_id: time::Local,
        start_offset: usize,
        end_id: time::Local,
        end_offset: usize,
        new_text: Option<Text>,
        version_in_range: &time::Global,
        local_timestamp: time::Local,
        lamport_timestamp: time::Lamport,
    ) -> Result<()> {
        let mut new_text = new_text.as_ref().cloned();
        let start_fragment_id = self.resolve_fragment_id(start_id, start_offset)?;
        let end_fragment_id = self.resolve_fragment_id(end_id, end_offset)?;

        let old_fragments = self.fragments.clone();
        let last_id = old_fragments.extent::<FragmentIdRef>().0.unwrap();
        let last_id_ref = FragmentIdRef::new(&last_id);

        let mut cursor = old_fragments.cursor::<FragmentIdRef, ()>();
        let mut new_fragments =
            cursor.slice(&FragmentIdRef::new(&start_fragment_id), SeekBias::Left);

        if start_offset == cursor.item().unwrap().end_offset() {
            new_fragments.push(cursor.item().unwrap().clone());
            cursor.next();
        }

        while let Some(fragment) = cursor.item() {
            if new_text.is_none() && fragment.id > end_fragment_id {
                break;
            }

            let mut fragment = fragment.clone();

            if fragment.id == start_fragment_id || fragment.id == end_fragment_id {
                let split_start = if start_fragment_id == fragment.id {
                    start_offset
                } else {
                    fragment.start_offset()
                };
                let split_end = if end_fragment_id == fragment.id {
                    end_offset
                } else {
                    fragment.end_offset()
                };
                let (before_range, within_range, after_range) = self.split_fragment(
                    cursor.prev_item().as_ref().unwrap(),
                    &fragment,
                    split_start..split_end,
                );
                let insertion = if let Some(new_text) = new_text.take() {
                    Some(self.build_fragment_to_insert(
                        before_range.as_ref().or(cursor.prev_item()).unwrap(),
                        within_range.as_ref().or(after_range.as_ref()),
                        new_text,
                        local_timestamp,
                        lamport_timestamp,
                    ))
                } else {
                    None
                };
                if let Some(fragment) = before_range {
                    new_fragments.push(fragment);
                }
                if let Some(fragment) = insertion {
                    new_fragments.push(fragment);
                }
                if let Some(mut fragment) = within_range {
                    if fragment.was_visible(&version_in_range, &self.undo_map) {
                        fragment.deletions.insert(local_timestamp);
                        fragment.visible = false;
                    }
                    new_fragments.push(fragment);
                }
                if let Some(fragment) = after_range {
                    new_fragments.push(fragment);
                }
            } else {
                if new_text.is_some() && lamport_timestamp > fragment.insertion.lamport_timestamp {
                    new_fragments.push(self.build_fragment_to_insert(
                        cursor.prev_item().as_ref().unwrap(),
                        Some(&fragment),
                        new_text.take().unwrap(),
                        local_timestamp,
                        lamport_timestamp,
                    ));
                }

                if fragment.id < end_fragment_id
                    && fragment.was_visible(&version_in_range, &self.undo_map)
                {
                    fragment.deletions.insert(local_timestamp);
                    fragment.visible = false;
                }
                new_fragments.push(fragment);
            }

            cursor.next();
        }

        if let Some(new_text) = new_text {
            new_fragments.push(self.build_fragment_to_insert(
                cursor.prev_item().as_ref().unwrap(),
                None,
                new_text,
                local_timestamp,
                lamport_timestamp,
            ));
        }

        new_fragments.push_tree(cursor.slice(&last_id_ref, SeekBias::Right));
        self.fragments = new_fragments;
        self.local_clock.observe(local_timestamp);
        self.lamport_clock.observe(lamport_timestamp);
        Ok(())
    }

    fn undo_or_redo(
        &mut self,
        edit_id: time::Local,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> Result<Operation> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();
        let undo = UndoOperation {
            id: self.local_clock.tick(),
            edit_id,
            count: self.undo_map.undo_count(edit_id) + 1,
        };
        self.apply_undo(undo)?;
        self.version.observe(undo.id);

        if let Some(ctx) = ctx {
            ctx.notify();
            let changes = self.edits_since(old_version).collect::<Vec<_>>();
            if !changes.is_empty() {
                self.did_edit(changes, was_dirty, ctx);
            }
        }

        Ok(Operation::Undo {
            undo,
            lamport_timestamp: self.lamport_clock.tick(),
        })
    }

    fn apply_undo(&mut self, undo: UndoOperation) -> Result<()> {
        let mut new_fragments;

        self.undo_map.insert(undo);
        let edit = &self.edit_ops[&undo.edit_id];
        let start_fragment_id = self.resolve_fragment_id(edit.start_id, edit.start_offset)?;
        let end_fragment_id = self.resolve_fragment_id(edit.end_id, edit.end_offset)?;
        let mut cursor = self.fragments.cursor::<FragmentIdRef, ()>();

        if edit.start_id == edit.end_id && edit.start_offset == edit.end_offset {
            let splits = &self.insertion_splits[&undo.edit_id];
            let mut insertion_splits = splits.cursor::<(), ()>().map(|s| &s.fragment_id).peekable();

            let first_split_id = insertion_splits.next().unwrap();
            new_fragments = cursor.slice(&FragmentIdRef::new(first_split_id), SeekBias::Left);

            loop {
                let mut fragment = cursor.item().unwrap().clone();
                fragment.visible = fragment.is_visible(&self.undo_map);
                fragment.max_undos.observe(undo.id);
                new_fragments.push(fragment);
                cursor.next();
                if let Some(split_id) = insertion_splits.next() {
                    new_fragments
                        .push_tree(cursor.slice(&FragmentIdRef::new(split_id), SeekBias::Left));
                } else {
                    break;
                }
            }
        } else {
            new_fragments = cursor.slice(&FragmentIdRef::new(&start_fragment_id), SeekBias::Left);
            while let Some(fragment) = cursor.item() {
                if fragment.id > end_fragment_id {
                    break;
                } else {
                    let mut fragment = fragment.clone();
                    if edit.version_in_range.observed(fragment.insertion.id)
                        || fragment.insertion.id == undo.edit_id
                    {
                        fragment.visible = fragment.is_visible(&self.undo_map);
                        fragment.max_undos.observe(undo.id);
                    }
                    new_fragments.push(fragment);
                    cursor.next();
                }
            }
        }

        new_fragments.push_tree(cursor.suffix());
        drop(cursor);
        self.fragments = new_fragments;

        Ok(())
    }

    fn flush_deferred_ops(&mut self) -> Result<()> {
        self.deferred_replicas.clear();
        let mut deferred_ops = Vec::new();
        for op in self.deferred_ops.drain().cursor().cloned() {
            if self.can_apply_op(&op) {
                self.apply_op(op)?;
            } else {
                self.deferred_replicas.insert(op.replica_id());
                deferred_ops.push(op);
            }
        }
        self.deferred_ops.insert(deferred_ops);
        Ok(())
    }

    fn can_apply_op(&self, op: &Operation) -> bool {
        if self.deferred_replicas.contains(&op.replica_id()) {
            false
        } else {
            match op {
                Operation::Edit { edit, .. } => {
                    self.version.observed(edit.start_id)
                        && self.version.observed(edit.end_id)
                        && edit.version_in_range <= self.version
                }
                Operation::Undo { undo, .. } => self.version.observed(undo.edit_id),
                Operation::UpdateSelections { selections, .. } => {
                    if let Some(selections) = selections {
                        selections.iter().all(|selection| {
                            let contains_start = match selection.start {
                                Anchor::Middle { insertion_id, .. } => {
                                    self.version.observed(insertion_id)
                                }
                                _ => true,
                            };
                            let contains_end = match selection.end {
                                Anchor::Middle { insertion_id, .. } => {
                                    self.version.observed(insertion_id)
                                }
                                _ => true,
                            };
                            contains_start && contains_end
                        })
                    } else {
                        true
                    }
                }
            }
        }
    }

    fn resolve_fragment_id(&self, edit_id: time::Local, offset: usize) -> Result<FragmentId> {
        let split_tree = self
            .insertion_splits
            .get(&edit_id)
            .ok_or_else(|| anyhow!("invalid operation"))?;
        let mut cursor = split_tree.cursor::<usize, ()>();
        cursor.seek(&offset, SeekBias::Left);
        Ok(cursor
            .item()
            .ok_or_else(|| anyhow!("invalid operation"))?
            .fragment_id
            .clone())
    }

    fn splice_fragments<I>(&mut self, mut old_ranges: I, new_text: Option<Text>) -> Vec<Operation>
    where
        I: Iterator<Item = Range<usize>>,
    {
        let mut cur_range = old_ranges.next();
        if cur_range.is_none() {
            return Vec::new();
        }

        let mut ops = Vec::with_capacity(old_ranges.size_hint().0);

        let old_fragments = self.fragments.clone();
        let mut cursor = old_fragments.cursor::<usize, usize>();
        let mut new_fragments = SumTree::new();
        new_fragments.push_tree(cursor.slice(&cur_range.as_ref().unwrap().start, SeekBias::Right));

        let mut start_id = None;
        let mut start_offset = None;
        let mut end_id = None;
        let mut end_offset = None;
        let mut version_in_range = time::Global::new();

        let mut local_timestamp = self.local_clock.tick();
        let mut lamport_timestamp = self.lamport_clock.tick();

        while cur_range.is_some() && cursor.item().is_some() {
            let mut fragment = cursor.item().unwrap().clone();
            let fragment_summary = cursor.item_summary().unwrap();
            let mut fragment_start = *cursor.start();
            let mut fragment_end = fragment_start + fragment.visible_len();

            let old_split_tree = self
                .insertion_splits
                .remove(&fragment.insertion.id)
                .unwrap();
            let mut splits_cursor = old_split_tree.cursor::<usize, ()>();
            let mut new_split_tree = splits_cursor.slice(&fragment.start_offset(), SeekBias::Right);

            // Find all splices that start or end within the current fragment. Then, split the
            // fragment and reassemble it in both trees accounting for the deleted and the newly
            // inserted text.
            while cur_range.as_ref().map_or(false, |r| r.start < fragment_end) {
                let range = cur_range.clone().unwrap();
                if range.start > fragment_start {
                    let mut prefix = fragment.clone();
                    prefix.set_end_offset(prefix.start_offset() + (range.start - fragment_start));
                    prefix.id =
                        FragmentId::between(&new_fragments.last().unwrap().id, &fragment.id);
                    fragment.set_start_offset(prefix.end_offset());
                    new_fragments.push(prefix.clone());
                    new_split_tree.push(InsertionSplit {
                        extent: prefix.end_offset() - prefix.start_offset(),
                        fragment_id: prefix.id,
                    });
                    fragment_start = range.start;
                }

                if range.end == fragment_start {
                    end_id = Some(new_fragments.last().unwrap().insertion.id);
                    end_offset = Some(new_fragments.last().unwrap().end_offset());
                } else if range.end == fragment_end {
                    end_id = Some(fragment.insertion.id);
                    end_offset = Some(fragment.end_offset());
                }

                if range.start == fragment_start {
                    start_id = Some(new_fragments.last().unwrap().insertion.id);
                    start_offset = Some(new_fragments.last().unwrap().end_offset());

                    if let Some(new_text) = new_text.clone() {
                        let new_fragment = self.build_fragment_to_insert(
                            &new_fragments.last().unwrap(),
                            Some(&fragment),
                            new_text,
                            local_timestamp,
                            lamport_timestamp,
                        );
                        new_fragments.push(new_fragment);
                    }
                }

                if range.end < fragment_end {
                    if range.end > fragment_start {
                        let mut prefix = fragment.clone();
                        prefix.set_end_offset(prefix.start_offset() + (range.end - fragment_start));
                        prefix.id =
                            FragmentId::between(&new_fragments.last().unwrap().id, &fragment.id);
                        version_in_range.observe_all(&fragment_summary.max_version);
                        if fragment.visible {
                            prefix.deletions.insert(local_timestamp);
                            prefix.visible = false;
                        }
                        fragment.set_start_offset(prefix.end_offset());
                        new_fragments.push(prefix.clone());
                        new_split_tree.push(InsertionSplit {
                            extent: prefix.end_offset() - prefix.start_offset(),
                            fragment_id: prefix.id,
                        });
                        fragment_start = range.end;
                        end_id = Some(fragment.insertion.id);
                        end_offset = Some(fragment.start_offset());
                    }
                } else {
                    version_in_range.observe_all(&fragment_summary.max_version);
                    if fragment.visible {
                        fragment.deletions.insert(local_timestamp);
                        fragment.visible = false;
                    }
                }

                // If the splice ends inside this fragment, we can advance to the next splice and
                // check if it also intersects the current fragment. Otherwise we break out of the
                // loop and find the first fragment that the splice does not contain fully.
                if range.end <= fragment_end {
                    ops.push(Operation::Edit {
                        edit: EditOperation {
                            id: local_timestamp,
                            start_id: start_id.unwrap(),
                            start_offset: start_offset.unwrap(),
                            end_id: end_id.unwrap(),
                            end_offset: end_offset.unwrap(),
                            version_in_range,
                            new_text: new_text.clone(),
                        },
                        lamport_timestamp,
                    });

                    start_id = None;
                    start_offset = None;
                    end_id = None;
                    end_offset = None;
                    version_in_range = time::Global::new();
                    cur_range = old_ranges.next();
                    if cur_range.is_some() {
                        local_timestamp = self.local_clock.tick();
                        lamport_timestamp = self.lamport_clock.tick();
                    }
                } else {
                    break;
                }
            }
            new_split_tree.push(InsertionSplit {
                extent: fragment.end_offset() - fragment.start_offset(),
                fragment_id: fragment.id.clone(),
            });
            splits_cursor.next();
            new_split_tree
                .push_tree(splits_cursor.slice(&old_split_tree.extent::<usize>(), SeekBias::Right));
            self.insertion_splits
                .insert(fragment.insertion.id, new_split_tree);
            new_fragments.push(fragment);

            // Scan forward until we find a fragment that is not fully contained by the current splice.
            cursor.next();
            if let Some(range) = cur_range.clone() {
                while let Some(fragment) = cursor.item() {
                    let fragment_summary = cursor.item_summary().unwrap();
                    fragment_start = *cursor.start();
                    fragment_end = fragment_start + fragment.visible_len();
                    if range.start < fragment_start && range.end >= fragment_end {
                        let mut new_fragment = fragment.clone();
                        version_in_range.observe_all(&fragment_summary.max_version);
                        if new_fragment.visible {
                            new_fragment.deletions.insert(local_timestamp);
                            new_fragment.visible = false;
                        }
                        new_fragments.push(new_fragment);
                        cursor.next();

                        if range.end == fragment_end {
                            end_id = Some(fragment.insertion.id);
                            end_offset = Some(fragment.end_offset());
                            ops.push(Operation::Edit {
                                edit: EditOperation {
                                    id: local_timestamp,
                                    start_id: start_id.unwrap(),
                                    start_offset: start_offset.unwrap(),
                                    end_id: end_id.unwrap(),
                                    end_offset: end_offset.unwrap(),
                                    version_in_range,
                                    new_text: new_text.clone(),
                                },
                                lamport_timestamp,
                            });

                            start_id = None;
                            start_offset = None;
                            end_id = None;
                            end_offset = None;
                            version_in_range = time::Global::new();

                            cur_range = old_ranges.next();
                            if cur_range.is_some() {
                                local_timestamp = self.local_clock.tick();
                                lamport_timestamp = self.lamport_clock.tick();
                            }
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // If the splice we are currently evaluating starts after the end of the fragment
                // that the cursor is parked at, we should seek to the next splice's start range
                // and push all the fragments in between into the new tree.
                if cur_range.as_ref().map_or(false, |r| r.start > fragment_end) {
                    new_fragments.push_tree(
                        cursor.slice(&cur_range.as_ref().unwrap().start, SeekBias::Right),
                    );
                }
            }
        }

        // Handle range that is at the end of the buffer if it exists. There should never be
        // multiple because ranges must be disjoint.
        if cur_range.is_some() {
            debug_assert_eq!(old_ranges.next(), None);
            let last_fragment = new_fragments.last().unwrap();
            ops.push(Operation::Edit {
                edit: EditOperation {
                    id: local_timestamp,
                    start_id: last_fragment.insertion.id,
                    start_offset: last_fragment.end_offset(),
                    end_id: last_fragment.insertion.id,
                    end_offset: last_fragment.end_offset(),
                    version_in_range: time::Global::new(),
                    new_text: new_text.clone(),
                },
                lamport_timestamp,
            });

            if let Some(new_text) = new_text {
                let new_fragment = self.build_fragment_to_insert(
                    &last_fragment,
                    None,
                    new_text,
                    local_timestamp,
                    lamport_timestamp,
                );
                new_fragments.push(new_fragment);
            }
        } else {
            new_fragments
                .push_tree(cursor.slice(&old_fragments.extent::<usize>(), SeekBias::Right));
        }

        self.fragments = new_fragments;
        ops
    }

    fn split_fragment(
        &mut self,
        prev_fragment: &Fragment,
        fragment: &Fragment,
        range: Range<usize>,
    ) -> (Option<Fragment>, Option<Fragment>, Option<Fragment>) {
        debug_assert!(range.start >= fragment.start_offset());
        debug_assert!(range.start <= fragment.end_offset());
        debug_assert!(range.end <= fragment.end_offset());
        debug_assert!(range.end >= fragment.start_offset());

        if range.end == fragment.start_offset() {
            (None, None, Some(fragment.clone()))
        } else if range.start == fragment.end_offset() {
            (Some(fragment.clone()), None, None)
        } else if range.start == fragment.start_offset() && range.end == fragment.end_offset() {
            (None, Some(fragment.clone()), None)
        } else {
            let mut prefix = fragment.clone();

            let after_range = if range.end < fragment.end_offset() {
                let mut suffix = prefix.clone();
                suffix.set_start_offset(range.end);
                prefix.set_end_offset(range.end);
                prefix.id = FragmentId::between(&prev_fragment.id, &suffix.id);
                Some(suffix)
            } else {
                None
            };

            let within_range = if range.start != range.end {
                let mut suffix = prefix.clone();
                suffix.set_start_offset(range.start);
                prefix.set_end_offset(range.start);
                prefix.id = FragmentId::between(&prev_fragment.id, &suffix.id);
                Some(suffix)
            } else {
                None
            };

            let before_range = if range.start > fragment.start_offset() {
                Some(prefix)
            } else {
                None
            };

            let old_split_tree = self
                .insertion_splits
                .remove(&fragment.insertion.id)
                .unwrap();
            let mut cursor = old_split_tree.cursor::<usize, ()>();
            let mut new_split_tree = cursor.slice(&fragment.start_offset(), SeekBias::Right);

            if let Some(ref fragment) = before_range {
                new_split_tree.push(InsertionSplit {
                    extent: range.start - fragment.start_offset(),
                    fragment_id: fragment.id.clone(),
                });
            }

            if let Some(ref fragment) = within_range {
                new_split_tree.push(InsertionSplit {
                    extent: range.end - range.start,
                    fragment_id: fragment.id.clone(),
                });
            }

            if let Some(ref fragment) = after_range {
                new_split_tree.push(InsertionSplit {
                    extent: fragment.end_offset() - range.end,
                    fragment_id: fragment.id.clone(),
                });
            }

            cursor.next();
            new_split_tree
                .push_tree(cursor.slice(&old_split_tree.extent::<usize>(), SeekBias::Right));

            self.insertion_splits
                .insert(fragment.insertion.id, new_split_tree);

            (before_range, within_range, after_range)
        }
    }

    fn build_fragment_to_insert(
        &mut self,
        prev_fragment: &Fragment,
        next_fragment: Option<&Fragment>,
        text: Text,
        local_timestamp: time::Local,
        lamport_timestamp: time::Lamport,
    ) -> Fragment {
        let new_fragment_id = FragmentId::between(
            &prev_fragment.id,
            next_fragment
                .map(|f| &f.id)
                .unwrap_or(&FragmentId::max_value()),
        );

        let mut split_tree = SumTree::new();
        split_tree.push(InsertionSplit {
            extent: text.len(),
            fragment_id: new_fragment_id.clone(),
        });
        self.insertion_splits.insert(local_timestamp, split_tree);

        Fragment::new(
            new_fragment_id,
            Insertion {
                id: local_timestamp,
                parent_id: prev_fragment.insertion.id,
                offset_in_parent: prev_fragment.end_offset(),
                text,
                lamport_timestamp,
            },
        )
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Result<Anchor> {
        self.anchor_at(position, AnchorBias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Result<Anchor> {
        self.anchor_at(position, AnchorBias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: AnchorBias) -> Result<Anchor> {
        let offset = position.to_offset(self)?;
        let max_offset = self.len();
        if offset > max_offset {
            return Err(anyhow!("offset is out of range"));
        }

        let seek_bias;
        match bias {
            AnchorBias::Left => {
                if offset == 0 {
                    return Ok(Anchor::Start);
                } else {
                    seek_bias = SeekBias::Left;
                }
            }
            AnchorBias::Right => {
                if offset == max_offset {
                    return Ok(Anchor::End);
                } else {
                    seek_bias = SeekBias::Right;
                }
            }
        };

        let mut cursor = self.fragments.cursor::<usize, usize>();
        cursor.seek(&offset, seek_bias);
        let fragment = cursor.item().unwrap();
        let offset_in_fragment = offset - cursor.start();
        let offset_in_insertion = fragment.start_offset() + offset_in_fragment;
        let anchor = Anchor::Middle {
            insertion_id: fragment.insertion.id,
            offset: offset_in_insertion,
            bias,
        };
        Ok(anchor)
    }

    fn fragment_id_for_anchor(&self, anchor: &Anchor) -> Result<&FragmentId> {
        match anchor {
            Anchor::Start => Ok(FragmentId::max_value()),
            Anchor::End => Ok(FragmentId::min_value()),
            Anchor::Middle {
                insertion_id,
                offset,
                bias,
                ..
            } => {
                let seek_bias = match bias {
                    AnchorBias::Left => SeekBias::Left,
                    AnchorBias::Right => SeekBias::Right,
                };

                let splits = self
                    .insertion_splits
                    .get(&insertion_id)
                    .ok_or_else(|| anyhow!("split does not exist for insertion id"))?;
                let mut splits_cursor = splits.cursor::<usize, ()>();
                splits_cursor.seek(offset, seek_bias);
                splits_cursor
                    .item()
                    .ok_or_else(|| anyhow!("split offset is out of range"))
                    .map(|split| &split.fragment_id)
            }
        }
    }

    fn summary_for_anchor(&self, anchor: &Anchor) -> Result<TextSummary> {
        match anchor {
            Anchor::Start => Ok(TextSummary::default()),
            Anchor::End => Ok(self.fragments.summary().text_summary),
            Anchor::Middle {
                insertion_id,
                offset,
                bias,
            } => {
                let seek_bias = match bias {
                    AnchorBias::Left => SeekBias::Left,
                    AnchorBias::Right => SeekBias::Right,
                };

                let splits = self
                    .insertion_splits
                    .get(&insertion_id)
                    .ok_or_else(|| anyhow!("split does not exist for insertion id"))?;
                let mut splits_cursor = splits.cursor::<usize, ()>();
                splits_cursor.seek(offset, seek_bias);
                let split = splits_cursor
                    .item()
                    .ok_or_else(|| anyhow!("split offset is out of range"))?;

                let mut fragments_cursor = self.fragments.cursor::<FragmentIdRef, TextSummary>();
                fragments_cursor.seek(&FragmentIdRef::new(&split.fragment_id), SeekBias::Left);
                let fragment = fragments_cursor
                    .item()
                    .ok_or_else(|| anyhow!("fragment id does not exist"))?;

                let mut summary = fragments_cursor.start().clone();
                if fragment.visible {
                    summary += fragment
                        .text
                        .slice(..offset - fragment.start_offset())
                        .summary();
                }
                Ok(summary)
            }
        }
    }

    #[allow(dead_code)]
    pub fn point_for_offset(&self, offset: usize) -> Result<Point> {
        let mut fragments_cursor = self.fragments.cursor::<usize, TextSummary>();
        fragments_cursor.seek(&offset, SeekBias::Left);
        fragments_cursor
            .item()
            .ok_or_else(|| anyhow!("offset is out of range"))
            .map(|fragment| {
                let overshoot = fragment
                    .point_for_offset(offset - &fragments_cursor.start().chars)
                    .unwrap();
                fragments_cursor.start().lines + &overshoot
            })
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self {
            file: self.file.clone(),
            fragments: self.fragments.clone(),
            insertion_splits: self.insertion_splits.clone(),
            edit_ops: self.edit_ops.clone(),
            version: self.version.clone(),
            saved_version: self.saved_version.clone(),
            last_edit: self.last_edit.clone(),
            undo_map: self.undo_map.clone(),
            selections: self.selections.clone(),
            selections_last_update: self.selections_last_update.clone(),
            deferred_ops: self.deferred_ops.clone(),
            deferred_replicas: self.deferred_replicas.clone(),
            replica_id: self.replica_id,
            local_clock: self.local_clock.clone(),
            lamport_clock: self.lamport_clock.clone(),
        }
    }
}

impl Snapshot {
    pub fn fragments<'a>(&'a self) -> FragmentIter<'a> {
        FragmentIter::new(&self.fragments)
    }

    pub fn text_summary(&self) -> TextSummary {
        self.fragments.summary().text_summary
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Edited(Vec<Edit>),
    Dirtied,
    Saved,
}

impl Entity for Buffer {
    type Event = Event;
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for Point {
    fn add_summary(&mut self, summary: &FragmentSummary) {
        *self += &summary.text_summary.lines;
    }
}

impl<'a> CharIter<'a> {
    fn new(fragments: &'a SumTree<Fragment>, offset: usize) -> Self {
        let mut fragments_cursor = fragments.cursor::<usize, usize>();
        fragments_cursor.seek(&offset, SeekBias::Right);
        let fragment_chars = fragments_cursor.item().map_or("".chars(), |fragment| {
            let offset_in_fragment = offset - fragments_cursor.start();
            fragment.text[offset_in_fragment..].chars()
        });
        Self {
            fragments_cursor,
            fragment_chars,
        }
    }
}

impl<'a> Iterator for CharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(char) = self.fragment_chars.next() {
            Some(char)
        } else {
            loop {
                self.fragments_cursor.next();
                if let Some(fragment) = self.fragments_cursor.item() {
                    if fragment.visible {
                        self.fragment_chars = fragment.text.as_str().chars();
                        return self.fragment_chars.next();
                    }
                } else {
                    return None;
                }
            }
        }
    }
}

impl<'a> FragmentIter<'a> {
    fn new(fragments: &'a SumTree<Fragment>) -> Self {
        let mut cursor = fragments.cursor::<usize, usize>();
        cursor.seek(&0, SeekBias::Right);
        Self {
            cursor,
            started: false,
        }
    }
}

impl<'a> Iterator for FragmentIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.started {
                self.cursor.next();
            } else {
                self.started = true;
            }
            if let Some(fragment) = self.cursor.item() {
                if fragment.visible {
                    return Some(fragment.text.as_str());
                }
            } else {
                return None;
            }
        }
    }
}

impl<'a, F: Fn(&FragmentSummary) -> bool> Iterator for Edits<'a, F> {
    type Item = Edit;

    fn next(&mut self) -> Option<Self::Item> {
        let mut change: Option<Edit> = None;

        while let Some(fragment) = self.cursor.item() {
            let new_offset = *self.cursor.start();
            let old_offset = (new_offset as isize - self.delta) as usize;

            if !fragment.was_visible(&self.since, &self.undos) && fragment.visible {
                if let Some(ref mut change) = change {
                    if change.new_range.end == new_offset {
                        change.new_range.end += fragment.len();
                        self.delta += fragment.len() as isize;
                    } else {
                        break;
                    }
                } else {
                    change = Some(Edit {
                        old_range: old_offset..old_offset,
                        new_range: new_offset..new_offset + fragment.len(),
                    });
                    self.delta += fragment.len() as isize;
                }
            } else if fragment.was_visible(&self.since, &self.undos) && !fragment.visible {
                if let Some(ref mut change) = change {
                    if change.new_range.end == new_offset {
                        change.old_range.end += fragment.len();
                        self.delta -= fragment.len() as isize;
                    } else {
                        break;
                    }
                } else {
                    change = Some(Edit {
                        old_range: old_offset..old_offset + fragment.len(),
                        new_range: new_offset..new_offset,
                    });
                    self.delta -= fragment.len() as isize;
                }
            }

            self.cursor.next();
        }

        change
    }
}

// pub fn diff(a: &[u16], b: &[u16]) -> Vec<Edit> {
//     struct EditCollector<'a> {
//         a: &'a [u16],
//         b: &'a [u16],
//         position: Point,
//         changes: Vec<Edit>,
//     }
//
//     impl<'a> diffs::Diff for EditCollector<'a> {
//         type Error = ();
//
//         fn equal(&mut self, old: usize, _: usize, len: usize) -> Result<(), ()> {
//             self.position += &Text::extent(&self.a[old..old + len]);
//             Ok(())
//         }
//
//         fn delete(&mut self, old: usize, len: usize) -> Result<(), ()> {
//             self.changes.push(Edit {
//                 range: self.position..self.position + &Text::extent(&self.a[old..old + len]),
//                 chars: Vec::new(),
//                 new_char_count: Point::zero(),
//             });
//             Ok(())
//         }
//
//         fn insert(&mut self, _: usize, new: usize, new_len: usize) -> Result<(), ()> {
//             let new_char_count = Text::extent(&self.b[new..new + new_len]);
//             self.changes.push(Edit {
//                 range: self.position..self.position,
//                 chars: Vec::from(&self.b[new..new + new_len]),
//                 new_char_count,
//             });
//             self.position += &new_char_count;
//             Ok(())
//         }
//
//         fn replace(
//             &mut self,
//             old: usize,
//             old_len: usize,
//             new: usize,
//             new_len: usize,
//         ) -> Result<(), ()> {
//             let old_extent = text::extent(&self.a[old..old + old_len]);
//             let new_char_count = text::extent(&self.b[new..new + new_len]);
//             self.changes.push(Edit {
//                 range: self.position..self.position + &old_extent,
//                 chars: Vec::from(&self.b[new..new + new_len]),
//                 new_char_count,
//             });
//             self.position += &new_char_count;
//             Ok(())
//         }
//     }
//
//     let mut collector = diffs::Replace::new(EditCollector {
//         a,
//         b,
//         position: Point::zero(),
//         changes: Vec::new(),
//     });
//     diffs::myers::diff(&mut collector, a, 0, a.len(), b, 0, b.len()).unwrap();
//     collector.into_inner().changes
// }

impl Selection {
    pub fn head(&self) -> &Anchor {
        if self.reversed {
            &self.start
        } else {
            &self.end
        }
    }

    pub fn set_head<S>(&mut self, buffer: &Buffer, cursor: Anchor) {
        if cursor.cmp(self.tail(), buffer).unwrap() < Ordering::Equal {
            if !self.reversed {
                mem::swap(&mut self.start, &mut self.end);
                self.reversed = true;
            }
            self.start = cursor;
        } else {
            if self.reversed {
                mem::swap(&mut self.start, &mut self.end);
                self.reversed = false;
            }
            self.end = cursor;
        }
    }

    pub fn tail(&self) -> &Anchor {
        if self.reversed {
            &self.end
        } else {
            &self.start
        }
    }

    pub fn is_empty(&self, buffer: &Buffer) -> bool {
        self.start.to_offset(buffer).unwrap() == self.end.to_offset(buffer).unwrap()
    }

    pub fn anchor_range(&self) -> Range<Anchor> {
        self.start.clone()..self.end.clone()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct FragmentId(Arc<[u16]>);

lazy_static! {
    static ref FRAGMENT_ID_EMPTY: FragmentId = FragmentId(Arc::from([]));
    static ref FRAGMENT_ID_MIN_VALUE: FragmentId = FragmentId(Arc::from([0 as u16]));
    static ref FRAGMENT_ID_MAX_VALUE: FragmentId = FragmentId(Arc::from([u16::max_value()]));
}

impl Default for FragmentId {
    fn default() -> Self {
        FRAGMENT_ID_EMPTY.clone()
    }
}

impl FragmentId {
    fn min_value() -> &'static Self {
        &FRAGMENT_ID_MIN_VALUE
    }

    fn max_value() -> &'static Self {
        &FRAGMENT_ID_MAX_VALUE
    }

    fn between(left: &Self, right: &Self) -> Self {
        Self::between_with_max(left, right, u16::max_value())
    }

    fn between_with_max(left: &Self, right: &Self, max_value: u16) -> Self {
        let mut new_entries = Vec::new();

        let left_entries = left.0.iter().cloned().chain(iter::repeat(0));
        let right_entries = right.0.iter().cloned().chain(iter::repeat(max_value));
        for (l, r) in left_entries.zip(right_entries) {
            let interval = r - l;
            if interval > 1 {
                new_entries.push(l + cmp::max(1, cmp::min(8, interval / 2)));
                break;
            } else {
                new_entries.push(l);
            }
        }

        FragmentId(Arc::from(new_entries))
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Default)]
struct FragmentIdRef<'a>(Option<&'a FragmentId>);

impl<'a> FragmentIdRef<'a> {
    fn new(id: &'a FragmentId) -> Self {
        Self(Some(id))
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for FragmentIdRef<'a> {
    fn add_summary(&mut self, summary: &'a FragmentSummary) {
        self.0 = Some(&summary.max_fragment_id)
    }
}

impl Fragment {
    fn new(id: FragmentId, insertion: Insertion) -> Self {
        Self {
            id,
            text: insertion.text.clone(),
            insertion,
            deletions: Default::default(),
            max_undos: Default::default(),
            visible: true,
        }
    }

    fn start_offset(&self) -> usize {
        self.text.range().start
    }

    fn set_start_offset(&mut self, offset: usize) {
        self.text = self.insertion.text.slice(offset..self.end_offset());
    }

    fn end_offset(&self) -> usize {
        self.text.range().end
    }

    fn set_end_offset(&mut self, offset: usize) {
        self.text = self.insertion.text.slice(self.start_offset()..offset);
    }

    fn visible_len(&self) -> usize {
        if self.visible {
            self.len()
        } else {
            0
        }
    }

    fn len(&self) -> usize {
        self.text.len()
    }

    fn is_visible(&self, undos: &UndoMap) -> bool {
        !undos.is_undone(self.insertion.id) && self.deletions.iter().all(|d| undos.is_undone(*d))
    }

    fn was_visible(&self, version: &time::Global, undos: &UndoMap) -> bool {
        (version.observed(self.insertion.id) && !undos.was_undone(self.insertion.id, version))
            && self
                .deletions
                .iter()
                .all(|d| !version.observed(*d) || undos.was_undone(*d, version))
    }

    fn point_for_offset(&self, offset: usize) -> Result<Point> {
        Ok(self.text.point_for_offset(offset))
    }

    fn offset_for_point(&self, point: Point) -> Result<usize> {
        Ok(self.text.offset_for_point(point))
    }
}

impl sum_tree::Item for Fragment {
    type Summary = FragmentSummary;

    fn summary(&self) -> Self::Summary {
        let mut max_version = time::Global::new();
        max_version.observe(self.insertion.id);
        for deletion in &self.deletions {
            max_version.observe(*deletion);
        }
        max_version.observe_all(&self.max_undos);

        if self.visible {
            FragmentSummary {
                text_summary: self.text.summary(),
                max_fragment_id: self.id.clone(),
                max_version,
            }
        } else {
            FragmentSummary {
                text_summary: TextSummary::default(),
                max_fragment_id: self.id.clone(),
                max_version,
            }
        }
    }
}

impl<'a> AddAssign<&'a FragmentSummary> for FragmentSummary {
    fn add_assign(&mut self, other: &Self) {
        self.text_summary += &other.text_summary;
        debug_assert!(self.max_fragment_id <= other.max_fragment_id);
        self.max_fragment_id = other.max_fragment_id.clone();
        self.max_version.observe_all(&other.max_version);
    }
}

impl Default for FragmentSummary {
    fn default() -> Self {
        FragmentSummary {
            text_summary: TextSummary::default(),
            max_fragment_id: FragmentId::min_value().clone(),
            max_version: time::Global::new(),
        }
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for TextSummary {
    fn add_summary(&mut self, summary: &FragmentSummary) {
        *self += &summary.text_summary;
    }
}

impl<'a> AddAssign<&'a FragmentExtent> for FragmentExtent {
    fn add_assign(&mut self, other: &Self) {
        self.chars += other.chars;
        self.lines += &other.lines;
    }
}

impl Default for FragmentExtent {
    fn default() -> Self {
        FragmentExtent {
            lines: Point::zero(),
            chars: 0,
        }
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for FragmentExtent {
    fn add_summary(&mut self, summary: &FragmentSummary) {
        self.chars += summary.text_summary.chars;
        self.lines += &summary.text_summary.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for usize {
    fn add_summary(&mut self, summary: &FragmentSummary) {
        *self += summary.text_summary.chars;
    }
}

impl sum_tree::Item for InsertionSplit {
    type Summary = InsertionSplitSummary;

    fn summary(&self) -> Self::Summary {
        InsertionSplitSummary {
            extent: self.extent,
        }
    }
}

impl<'a> AddAssign<&'a InsertionSplitSummary> for InsertionSplitSummary {
    fn add_assign(&mut self, other: &Self) {
        self.extent += other.extent;
    }
}

impl Default for InsertionSplitSummary {
    fn default() -> Self {
        InsertionSplitSummary { extent: 0 }
    }
}

impl<'a> sum_tree::Dimension<'a, InsertionSplitSummary> for usize {
    fn add_summary(&mut self, summary: &InsertionSplitSummary) {
        *self += &summary.extent;
    }
}

impl Operation {
    fn replica_id(&self) -> ReplicaId {
        self.lamport_timestamp().replica_id
    }

    fn lamport_timestamp(&self) -> time::Lamport {
        match self {
            Operation::Edit {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            Operation::Undo {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            Operation::UpdateSelections {
                lamport_timestamp, ..
            } => *lamport_timestamp,
        }
    }

    pub fn is_edit(&self) -> bool {
        match self {
            Operation::Edit { .. } => true,
            _ => false,
        }
    }
}

impl operation_queue::Operation for Operation {
    fn timestamp(&self) -> time::Lamport {
        self.lamport_timestamp()
    }
}

pub trait ToOffset {
    fn to_offset(&self, buffer: &Buffer) -> Result<usize>;
}

impl ToOffset for Point {
    fn to_offset(&self, buffer: &Buffer) -> Result<usize> {
        let mut fragments_cursor = buffer.fragments.cursor::<Point, TextSummary>();
        fragments_cursor.seek(self, SeekBias::Left);
        fragments_cursor
            .item()
            .ok_or_else(|| anyhow!("point is out of range"))
            .map(|fragment| {
                let overshoot = fragment
                    .offset_for_point(*self - fragments_cursor.start().lines)
                    .unwrap();
                fragments_cursor.start().chars + overshoot
            })
    }
}

impl ToOffset for usize {
    fn to_offset(&self, _: &Buffer) -> Result<usize> {
        Ok(*self)
    }
}

impl ToOffset for Anchor {
    fn to_offset(&self, buffer: &Buffer) -> Result<usize> {
        Ok(buffer.summary_for_anchor(self)?.chars)
    }
}

pub trait ToPoint {
    fn to_point(&self, buffer: &Buffer) -> Result<Point>;
}

impl ToPoint for Anchor {
    fn to_point(&self, buffer: &Buffer) -> Result<Point> {
        Ok(buffer.summary_for_anchor(self)?.lines)
    }
}

impl ToPoint for usize {
    fn to_point(&self, buffer: &Buffer) -> Result<Point> {
        let mut fragments_cursor = buffer.fragments.cursor::<usize, TextSummary>();
        fragments_cursor.seek(&self, SeekBias::Left);
        fragments_cursor
            .item()
            .ok_or_else(|| anyhow!("offset is out of range"))
            .map(|fragment| {
                let overshoot = fragment
                    .point_for_offset(*self - &fragments_cursor.start().chars)
                    .unwrap();
                fragments_cursor.start().lines + overshoot
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::App;
    use std::collections::BTreeMap;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_edit() -> Result<()> {
        let mut buffer = Buffer::new(0, "abc");
        assert_eq!(buffer.text(), "abc");
        buffer.edit(vec![3..3], "def", None)?;
        assert_eq!(buffer.text(), "abcdef");
        buffer.edit(vec![0..0], "ghi", None)?;
        assert_eq!(buffer.text(), "ghiabcdef");
        buffer.edit(vec![5..5], "jkl", None)?;
        assert_eq!(buffer.text(), "ghiabjklcdef");
        buffer.edit(vec![6..7], "", None)?;
        assert_eq!(buffer.text(), "ghiabjlcdef");
        buffer.edit(vec![4..9], "mno", None)?;
        assert_eq!(buffer.text(), "ghiamnoef");

        Ok(())
    }

    #[test]
    fn test_edit_events() {
        App::test((), |mut app| async move {
            let buffer_1_events = Rc::new(RefCell::new(Vec::new()));
            let buffer_2_events = Rc::new(RefCell::new(Vec::new()));

            let buffer1 = app.add_model(|_| Buffer::new(0, "abcdef"));
            let buffer2 = app.add_model(|_| Buffer::new(1, "abcdef"));
            let ops = buffer1.update(&mut app, |buffer, ctx| {
                let buffer_1_events = buffer_1_events.clone();
                ctx.subscribe(&buffer1, move |_, event, _| {
                    buffer_1_events.borrow_mut().push(event.clone())
                });
                let buffer_2_events = buffer_2_events.clone();
                ctx.subscribe(&buffer2, move |_, event, _| {
                    buffer_2_events.borrow_mut().push(event.clone())
                });

                buffer.edit(Some(2..4), "XYZ", Some(ctx)).unwrap()
            });
            buffer2.update(&mut app, |buffer, ctx| {
                buffer.apply_ops(ops, Some(ctx)).unwrap();
            });

            let buffer_1_events = buffer_1_events.borrow();
            assert_eq!(
                *buffer_1_events,
                vec![
                    Event::Edited(vec![Edit {
                        old_range: 2..4,
                        new_range: 2..5
                    },]),
                    Event::Dirtied
                ]
            );

            let buffer_2_events = buffer_2_events.borrow();
            assert_eq!(
                *buffer_2_events,
                vec![
                    Event::Edited(vec![Edit {
                        old_range: 2..4,
                        new_range: 2..5
                    },]),
                    Event::Dirtied
                ]
            );
        });
    }

    #[test]
    fn test_random_edits() {
        for seed in 0..100 {
            println!("{:?}", seed);
            let mut rng = &mut StdRng::seed_from_u64(seed);

            let reference_string_len = rng.gen_range(0..3);
            let mut reference_string = RandomCharIter::new(&mut rng)
                .take(reference_string_len)
                .collect::<String>();
            let mut buffer = Buffer::new(0, reference_string.as_str());
            let mut buffer_versions = Vec::new();

            for _i in 0..10 {
                let (old_ranges, new_text, _) = buffer.randomly_mutate(rng, None);
                for old_range in old_ranges.iter().rev() {
                    reference_string = [
                        &reference_string[0..old_range.start],
                        new_text.as_str(),
                        &reference_string[old_range.end..],
                    ]
                    .concat();
                }
                assert_eq!(buffer.text(), reference_string);

                {
                    let line_lengths = line_lengths_in_range(&buffer, 0..buffer.len());

                    for (len, rows) in &line_lengths {
                        for row in rows {
                            assert_eq!(buffer.line_len(*row).unwrap(), *len);
                        }
                    }

                    let (longest_column, longest_rows) = line_lengths.iter().next_back().unwrap();
                    let rightmost_point = buffer.rightmost_point();
                    assert_eq!(rightmost_point.column, *longest_column);
                    assert!(longest_rows.contains(&rightmost_point.row));
                }

                for _ in 0..5 {
                    let end = rng.gen_range(0..buffer.len() + 1);
                    let start = rng.gen_range(0..end + 1);

                    let line_lengths = line_lengths_in_range(&buffer, start..end);
                    let (longest_column, longest_rows) = line_lengths.iter().next_back().unwrap();
                    let range_sum = buffer.text_summary_for_range(start..end);
                    assert_eq!(range_sum.rightmost_point.column, *longest_column);
                    assert!(longest_rows.contains(&range_sum.rightmost_point.row));
                    let range_text = &buffer.text()[start..end];
                    assert_eq!(range_sum.chars, range_text.chars().count());
                    assert_eq!(range_sum.bytes, range_text.len());
                }

                if rng.gen_bool(0.3) {
                    buffer_versions.push(buffer.clone());
                }
            }

            for mut old_buffer in buffer_versions {
                let mut delta = 0_isize;
                for Edit {
                    old_range,
                    new_range,
                } in buffer.edits_since(old_buffer.version.clone())
                {
                    let old_len = old_range.end - old_range.start;
                    let new_len = new_range.end - new_range.start;
                    let old_start = (old_range.start as isize + delta) as usize;

                    old_buffer
                        .edit(
                            Some(old_start..old_start + old_len),
                            buffer.text_for_range(new_range).unwrap(),
                            None,
                        )
                        .unwrap();

                    delta += new_len as isize - old_len as isize;
                }
                assert_eq!(old_buffer.text(), buffer.text());
            }
        }
    }

    #[test]
    fn test_line_len() -> Result<()> {
        let mut buffer = Buffer::new(0, "");
        buffer.edit(vec![0..0], "abcd\nefg\nhij", None)?;
        buffer.edit(vec![12..12], "kl\nmno", None)?;
        buffer.edit(vec![18..18], "\npqrs\n", None)?;
        buffer.edit(vec![18..21], "\nPQ", None)?;

        assert_eq!(buffer.line_len(0)?, 4);
        assert_eq!(buffer.line_len(1)?, 3);
        assert_eq!(buffer.line_len(2)?, 5);
        assert_eq!(buffer.line_len(3)?, 3);
        assert_eq!(buffer.line_len(4)?, 4);
        assert_eq!(buffer.line_len(5)?, 0);
        assert!(buffer.line_len(6).is_err());

        Ok(())
    }

    #[test]
    fn test_rightmost_point() -> Result<()> {
        let mut buffer = Buffer::new(0, "");
        assert_eq!(buffer.rightmost_point().row, 0);
        buffer.edit(vec![0..0], "abcd\nefg\nhij", None)?;
        assert_eq!(buffer.rightmost_point().row, 0);
        buffer.edit(vec![12..12], "kl\nmno", None)?;
        assert_eq!(buffer.rightmost_point().row, 2);
        buffer.edit(vec![18..18], "\npqrs", None)?;
        assert_eq!(buffer.rightmost_point().row, 2);
        buffer.edit(vec![10..12], "", None)?;
        assert_eq!(buffer.rightmost_point().row, 0);
        buffer.edit(vec![24..24], "tuv", None)?;
        assert_eq!(buffer.rightmost_point().row, 4);

        println!("{:?}", buffer.text());

        Ok(())
    }

    #[test]
    fn test_text_summary_for_range() {
        let buffer = Buffer::new(0, "ab\nefg\nhklm\nnopqrs\ntuvwxyz");
        let text = Text::from(buffer.text());

        assert_eq!(
            buffer.text_summary_for_range(1..3),
            text.slice(1..3).summary()
        );
        assert_eq!(
            buffer.text_summary_for_range(1..12),
            text.slice(1..12).summary()
        );
        assert_eq!(
            buffer.text_summary_for_range(0..20),
            text.slice(0..20).summary()
        );
        assert_eq!(
            buffer.text_summary_for_range(0..22),
            text.slice(0..22).summary()
        );
        assert_eq!(
            buffer.text_summary_for_range(7..22),
            text.slice(7..22).summary()
        );
    }

    #[test]
    fn test_chars_at() -> Result<()> {
        let mut buffer = Buffer::new(0, "");
        buffer.edit(vec![0..0], "abcd\nefgh\nij", None)?;
        buffer.edit(vec![12..12], "kl\nmno", None)?;
        buffer.edit(vec![18..18], "\npqrs", None)?;
        buffer.edit(vec![18..21], "\nPQ", None)?;

        let chars = buffer.chars_at(Point::new(0, 0))?;
        assert_eq!(chars.collect::<String>(), "abcd\nefgh\nijkl\nmno\nPQrs");

        let chars = buffer.chars_at(Point::new(1, 0))?;
        assert_eq!(chars.collect::<String>(), "efgh\nijkl\nmno\nPQrs");

        let chars = buffer.chars_at(Point::new(2, 0))?;
        assert_eq!(chars.collect::<String>(), "ijkl\nmno\nPQrs");

        let chars = buffer.chars_at(Point::new(3, 0))?;
        assert_eq!(chars.collect::<String>(), "mno\nPQrs");

        let chars = buffer.chars_at(Point::new(4, 0))?;
        assert_eq!(chars.collect::<String>(), "PQrs");

        // Regression test:
        let mut buffer = Buffer::new(0, "");
        buffer.edit(vec![0..0], "[workspace]\nmembers = [\n    \"xray_core\",\n    \"xray_server\",\n    \"xray_cli\",\n    \"xray_wasm\",\n]\n", None)?;
        buffer.edit(vec![60..60], "\n", None)?;

        let chars = buffer.chars_at(Point::new(6, 0))?;
        assert_eq!(chars.collect::<String>(), "    \"xray_wasm\",\n]\n");

        Ok(())
    }

    // #[test]
    // fn test_point_for_offset() -> Result<()> {
    //     let text = Text::from("abc\ndefgh\nijklm\nopq");
    //     assert_eq!(text.point_for_offset(0)?, Point { row: 0, column: 0 });
    //     assert_eq!(text.point_for_offset(1)?, Point { row: 0, column: 1 });
    //     assert_eq!(text.point_for_offset(2)?, Point { row: 0, column: 2 });
    //     assert_eq!(text.point_for_offset(3)?, Point { row: 0, column: 3 });
    //     assert_eq!(text.point_for_offset(4)?, Point { row: 1, column: 0 });
    //     assert_eq!(text.point_for_offset(5)?, Point { row: 1, column: 1 });
    //     assert_eq!(text.point_for_offset(9)?, Point { row: 1, column: 5 });
    //     assert_eq!(text.point_for_offset(10)?, Point { row: 2, column: 0 });
    //     assert_eq!(text.point_for_offset(14)?, Point { row: 2, column: 4 });
    //     assert_eq!(text.point_for_offset(15)?, Point { row: 2, column: 5 });
    //     assert_eq!(text.point_for_offset(16)?, Point { row: 3, column: 0 });
    //     assert_eq!(text.point_for_offset(17)?, Point { row: 3, column: 1 });
    //     assert_eq!(text.point_for_offset(19)?, Point { row: 3, column: 3 });
    //     assert!(text.point_for_offset(20).is_err());
    //
    //     let text = Text::from("abc");
    //     assert_eq!(text.point_for_offset(0)?, Point { row: 0, column: 0 });
    //     assert_eq!(text.point_for_offset(1)?, Point { row: 0, column: 1 });
    //     assert_eq!(text.point_for_offset(2)?, Point { row: 0, column: 2 });
    //     assert_eq!(text.point_for_offset(3)?, Point { row: 0, column: 3 });
    //     assert!(text.point_for_offset(4).is_err());
    //     Ok(())
    // }

    // #[test]
    // fn test_offset_for_point() -> Result<()> {
    //     let text = Text::from("abc\ndefgh");
    //     assert_eq!(text.offset_for_point(Point { row: 0, column: 0 })?, 0);
    //     assert_eq!(text.offset_for_point(Point { row: 0, column: 1 })?, 1);
    //     assert_eq!(text.offset_for_point(Point { row: 0, column: 2 })?, 2);
    //     assert_eq!(text.offset_for_point(Point { row: 0, column: 3 })?, 3);
    //     assert!(text.offset_for_point(Point { row: 0, column: 4 }).is_err());
    //     assert_eq!(text.offset_for_point(Point { row: 1, column: 0 })?, 4);
    //     assert_eq!(text.offset_for_point(Point { row: 1, column: 1 })?, 5);
    //     assert_eq!(text.offset_for_point(Point { row: 1, column: 5 })?, 9);
    //     assert!(text.offset_for_point(Point { row: 1, column: 6 }).is_err());
    //
    //     let text = Text::from("abc");
    //     assert_eq!(text.offset_for_point(Point { row: 0, column: 0 })?, 0);
    //     assert_eq!(text.offset_for_point(Point { row: 0, column: 1 })?, 1);
    //     assert_eq!(text.offset_for_point(Point { row: 0, column: 2 })?, 2);
    //     assert_eq!(text.offset_for_point(Point { row: 0, column: 3 })?, 3);
    //     assert!(text.offset_for_point(Point { row: 0, column: 4 }).is_err());
    //     Ok(())
    // }

    // #[test]
    // fn test_longest_row_in_range() -> Result<()> {
    //     for seed in 0..100 {
    //         println!("{:?}", seed);
    //         let mut rng = &mut StdRng::seed_from_u64(seed);
    //         let string_len = rng.gen_range(1, 10);
    //         let string = RandomCharIter(&mut rng)
    //             .take(string_len)
    //             .collect::<String>();
    //         let text = Text::from(string.as_ref());
    //
    //         for _i in 0..10 {
    //             let end = rng.gen_range(1, string.len() + 1);
    //             let start = rng.gen_range(0, end);
    //
    //             let mut cur_row = string[0..start].chars().filter(|c| *c == '\n').count() as u32;
    //             let mut cur_row_len = 0;
    //             let mut expected_longest_row = cur_row;
    //             let mut expected_longest_row_len = cur_row_len;
    //             for ch in string[start..end].chars() {
    //                 if ch == '\n' {
    //                     if cur_row_len > expected_longest_row_len {
    //                         expected_longest_row = cur_row;
    //                         expected_longest_row_len = cur_row_len;
    //                     }
    //                     cur_row += 1;
    //                     cur_row_len = 0;
    //                 } else {
    //                     cur_row_len += 1;
    //                 }
    //             }
    //             if cur_row_len > expected_longest_row_len {
    //                 expected_longest_row = cur_row;
    //                 expected_longest_row_len = cur_row_len;
    //             }
    //
    //             assert_eq!(
    //                 text.longest_row_in_range(start..end)?,
    //                 (expected_longest_row, expected_longest_row_len)
    //             );
    //         }
    //     }
    //     Ok(())
    // }

    #[test]
    fn test_fragment_ids() {
        for seed in 0..10 {
            let rng = &mut StdRng::seed_from_u64(seed);

            let mut ids = vec![FragmentId(Arc::from([0])), FragmentId(Arc::from([4]))];
            for _i in 0..100 {
                let index = rng.gen_range(1..ids.len());

                let left = ids[index - 1].clone();
                let right = ids[index].clone();
                ids.insert(index, FragmentId::between_with_max(&left, &right, 4));

                let mut sorted_ids = ids.clone();
                sorted_ids.sort();
                assert_eq!(ids, sorted_ids);
            }
        }
    }

    #[test]
    fn test_anchors() -> Result<()> {
        let mut buffer = Buffer::new(0, "");
        buffer.edit(vec![0..0], "abc", None)?;
        let left_anchor = buffer.anchor_before(2).unwrap();
        let right_anchor = buffer.anchor_after(2).unwrap();

        buffer.edit(vec![1..1], "def\n", None)?;
        assert_eq!(buffer.text(), "adef\nbc");
        assert_eq!(left_anchor.to_offset(&buffer).unwrap(), 6);
        assert_eq!(right_anchor.to_offset(&buffer).unwrap(), 6);
        assert_eq!(
            left_anchor.to_point(&buffer).unwrap(),
            Point { row: 1, column: 1 }
        );
        assert_eq!(
            right_anchor.to_point(&buffer).unwrap(),
            Point { row: 1, column: 1 }
        );

        buffer.edit(vec![2..3], "", None)?;
        assert_eq!(buffer.text(), "adf\nbc");
        assert_eq!(left_anchor.to_offset(&buffer).unwrap(), 5);
        assert_eq!(right_anchor.to_offset(&buffer).unwrap(), 5);
        assert_eq!(
            left_anchor.to_point(&buffer).unwrap(),
            Point { row: 1, column: 1 }
        );
        assert_eq!(
            right_anchor.to_point(&buffer).unwrap(),
            Point { row: 1, column: 1 }
        );

        buffer.edit(vec![5..5], "ghi\n", None)?;
        assert_eq!(buffer.text(), "adf\nbghi\nc");
        assert_eq!(left_anchor.to_offset(&buffer).unwrap(), 5);
        assert_eq!(right_anchor.to_offset(&buffer).unwrap(), 9);
        assert_eq!(
            left_anchor.to_point(&buffer).unwrap(),
            Point { row: 1, column: 1 }
        );
        assert_eq!(
            right_anchor.to_point(&buffer).unwrap(),
            Point { row: 2, column: 0 }
        );

        buffer.edit(vec![7..9], "", None)?;
        assert_eq!(buffer.text(), "adf\nbghc");
        assert_eq!(left_anchor.to_offset(&buffer).unwrap(), 5);
        assert_eq!(right_anchor.to_offset(&buffer).unwrap(), 7);
        assert_eq!(
            left_anchor.to_point(&buffer).unwrap(),
            Point { row: 1, column: 1 },
        );
        assert_eq!(
            right_anchor.to_point(&buffer).unwrap(),
            Point { row: 1, column: 3 }
        );

        // Ensure anchoring to a point is equivalent to anchoring to an offset.
        assert_eq!(
            buffer.anchor_before(Point { row: 0, column: 0 })?,
            buffer.anchor_before(0)?
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 0, column: 1 })?,
            buffer.anchor_before(1)?
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 0, column: 2 })?,
            buffer.anchor_before(2)?
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 0, column: 3 })?,
            buffer.anchor_before(3)?
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 0 })?,
            buffer.anchor_before(4)?
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 1 })?,
            buffer.anchor_before(5)?
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 2 })?,
            buffer.anchor_before(6)?
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 3 })?,
            buffer.anchor_before(7)?
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 4 })?,
            buffer.anchor_before(8)?
        );

        // Comparison between anchors.
        let anchor_at_offset_0 = buffer.anchor_before(0).unwrap();
        let anchor_at_offset_1 = buffer.anchor_before(1).unwrap();
        let anchor_at_offset_2 = buffer.anchor_before(2).unwrap();

        assert_eq!(
            anchor_at_offset_0.cmp(&anchor_at_offset_0, &buffer)?,
            Ordering::Equal
        );
        assert_eq!(
            anchor_at_offset_1.cmp(&anchor_at_offset_1, &buffer)?,
            Ordering::Equal
        );
        assert_eq!(
            anchor_at_offset_2.cmp(&anchor_at_offset_2, &buffer)?,
            Ordering::Equal
        );

        assert_eq!(
            anchor_at_offset_0.cmp(&anchor_at_offset_1, &buffer)?,
            Ordering::Less
        );
        assert_eq!(
            anchor_at_offset_1.cmp(&anchor_at_offset_2, &buffer)?,
            Ordering::Less
        );
        assert_eq!(
            anchor_at_offset_0.cmp(&anchor_at_offset_2, &buffer)?,
            Ordering::Less
        );

        assert_eq!(
            anchor_at_offset_1.cmp(&anchor_at_offset_0, &buffer)?,
            Ordering::Greater
        );
        assert_eq!(
            anchor_at_offset_2.cmp(&anchor_at_offset_1, &buffer)?,
            Ordering::Greater
        );
        assert_eq!(
            anchor_at_offset_2.cmp(&anchor_at_offset_0, &buffer)?,
            Ordering::Greater
        );
        Ok(())
    }

    #[test]
    fn test_anchors_at_start_and_end() -> Result<()> {
        let mut buffer = Buffer::new(0, "");
        let before_start_anchor = buffer.anchor_before(0).unwrap();
        let after_end_anchor = buffer.anchor_after(0).unwrap();

        buffer.edit(vec![0..0], "abc", None)?;
        assert_eq!(buffer.text(), "abc");
        assert_eq!(before_start_anchor.to_offset(&buffer).unwrap(), 0);
        assert_eq!(after_end_anchor.to_offset(&buffer).unwrap(), 3);

        let after_start_anchor = buffer.anchor_after(0).unwrap();
        let before_end_anchor = buffer.anchor_before(3).unwrap();

        buffer.edit(vec![3..3], "def", None)?;
        buffer.edit(vec![0..0], "ghi", None)?;
        assert_eq!(buffer.text(), "ghiabcdef");
        assert_eq!(before_start_anchor.to_offset(&buffer).unwrap(), 0);
        assert_eq!(after_start_anchor.to_offset(&buffer).unwrap(), 3);
        assert_eq!(before_end_anchor.to_offset(&buffer).unwrap(), 6);
        assert_eq!(after_end_anchor.to_offset(&buffer).unwrap(), 9);

        Ok(())
    }

    #[test]
    fn test_is_modified() -> Result<()> {
        App::test((), |mut app| async move {
            let model = app.add_model(|_| Buffer::new(0, "abc"));
            let events = Rc::new(RefCell::new(Vec::new()));

            // initially, the buffer isn't dirty.
            model.update(&mut app, |buffer, ctx| {
                ctx.subscribe(&model, {
                    let events = events.clone();
                    move |_, event, _| events.borrow_mut().push(event.clone())
                });

                assert!(!buffer.is_dirty());
                assert!(events.borrow().is_empty());

                buffer.edit(vec![1..2], "", Some(ctx)).unwrap();
            });

            // after the first edit, the buffer is dirty, and emits a dirtied event.
            model.update(&mut app, |buffer, ctx| {
                assert!(buffer.text() == "ac");
                assert!(buffer.is_dirty());
                assert_eq!(
                    *events.borrow(),
                    &[
                        Event::Edited(vec![Edit {
                            old_range: 1..2,
                            new_range: 1..1
                        }]),
                        Event::Dirtied
                    ]
                );
                events.borrow_mut().clear();

                buffer.did_save(buffer.version(), ctx);
            });

            // after saving, the buffer is not dirty, and emits a saved event.
            model.update(&mut app, |buffer, ctx| {
                assert!(!buffer.is_dirty());
                assert_eq!(*events.borrow(), &[Event::Saved]);
                events.borrow_mut().clear();

                buffer.edit(vec![1..1], "B", Some(ctx)).unwrap();
                buffer.edit(vec![2..2], "D", Some(ctx)).unwrap();
            });

            // after editing again, the buffer is dirty, and emits another dirty event.
            model.update(&mut app, |buffer, ctx| {
                assert!(buffer.text() == "aBDc");
                assert!(buffer.is_dirty());
                assert_eq!(
                    *events.borrow(),
                    &[
                        Event::Edited(vec![Edit {
                            old_range: 1..1,
                            new_range: 1..2
                        }]),
                        Event::Dirtied,
                        Event::Edited(vec![Edit {
                            old_range: 2..2,
                            new_range: 2..3
                        }]),
                    ],
                );
                events.borrow_mut().clear();

                // TODO - currently, after restoring the buffer to its
                // previously-saved state, the is still considered dirty.
                buffer.edit(vec![1..3], "", Some(ctx)).unwrap();
                assert!(buffer.text() == "ac");
                assert!(buffer.is_dirty());
            });

            model.update(&mut app, |_, _| {
                assert_eq!(
                    *events.borrow(),
                    &[Event::Edited(vec![Edit {
                        old_range: 1..3,
                        new_range: 1..1
                    },])]
                );
            });
        });
        Ok(())
    }

    #[test]
    fn test_undo_redo() -> Result<()> {
        let mut buffer = Buffer::new(0, "");

        let edit1 = buffer.edit(vec![0..0], "abx", None)?;
        let edit2 = buffer.edit(vec![2..3], "yzef", None)?;
        let edit3 = buffer.edit(vec![2..4], "cd", None)?;

        buffer.undo_or_redo(edit1[0].edit_id().unwrap(), None)?;
        assert_eq!(buffer.text(), "cdef");
        buffer.undo_or_redo(edit1[0].edit_id().unwrap(), None)?;
        assert_eq!(buffer.text(), "abcdef");

        buffer.undo_or_redo(edit2[0].edit_id().unwrap(), None)?;
        assert_eq!(buffer.text(), "abcdx");
        buffer.undo_or_redo(edit3[0].edit_id().unwrap(), None)?;
        assert_eq!(buffer.text(), "abx");
        buffer.undo_or_redo(edit2[0].edit_id().unwrap(), None)?;
        assert_eq!(buffer.text(), "abyzef");
        buffer.undo_or_redo(edit3[0].edit_id().unwrap(), None)?;
        assert_eq!(buffer.text(), "abcdef");

        buffer.undo_or_redo(edit3[0].edit_id().unwrap(), None)?;
        assert_eq!(buffer.text(), "abyzef");
        buffer.undo_or_redo(edit1[0].edit_id().unwrap(), None)?;
        assert_eq!(buffer.text(), "yzef");
        buffer.undo_or_redo(edit2[0].edit_id().unwrap(), None)?;
        assert_eq!(buffer.text(), "");

        Ok(())
    }

    #[test]
    fn test_random_concurrent_edits() {
        use crate::test::Network;

        const PEERS: usize = 5;

        for seed in 0..100 {
            println!("{:?}", seed);
            let mut rng = &mut StdRng::seed_from_u64(seed);

            let base_text_len = rng.gen_range(0..10);
            let base_text = RandomCharIter::new(&mut rng)
                .take(base_text_len)
                .collect::<String>();
            let mut replica_ids = Vec::new();
            let mut buffers = Vec::new();
            let mut network = Network::new();
            for i in 0..PEERS {
                let buffer = Buffer::new(i as ReplicaId, base_text.as_str());
                buffers.push(buffer);
                replica_ids.push(i as u16);
                network.add_peer(i as u16);
            }

            let mut mutation_count = 10;
            loop {
                let replica_index = rng.gen_range(0..PEERS);
                let replica_id = replica_ids[replica_index];
                let buffer = &mut buffers[replica_index];

                match rng.gen_range(0..=100) {
                    0..=50 if mutation_count != 0 => {
                        let (_, _, ops) = buffer.randomly_mutate(&mut rng, None);
                        network.broadcast(replica_id, ops, &mut rng);
                        mutation_count -= 1;
                    }
                    51..=70 if mutation_count != 0 => {
                        let ops = buffer.randomly_undo_redo(&mut rng, None);
                        network.broadcast(replica_id, ops, &mut rng);
                        mutation_count -= 1;
                    }
                    71..=100 if network.has_unreceived(replica_id) => {
                        buffer
                            .apply_ops(network.receive(replica_id, &mut rng), None)
                            .unwrap();
                    }
                    _ => {}
                }

                if mutation_count == 0 && network.is_idle() {
                    break;
                }
            }

            for buffer in &buffers[1..] {
                assert_eq!(buffer.text(), buffers[0].text());
                assert_eq!(
                    buffer.all_selections().collect::<HashMap<_, _>>(),
                    buffers[0].all_selections().collect::<HashMap<_, _>>()
                );
                assert_eq!(
                    buffer.all_selection_ranges().collect::<HashMap<_, _>>(),
                    buffers[0].all_selection_ranges().collect::<HashMap<_, _>>()
                );
            }
        }
    }

    impl Buffer {
        pub fn randomly_mutate<T>(
            &mut self,
            rng: &mut T,
            mut ctx: Option<&mut ModelContext<Self>>,
        ) -> (Vec<Range<usize>>, String, Vec<Operation>)
        where
            T: Rng,
        {
            // Randomly edit
            let (old_ranges, new_text, mut operations) =
                self.randomly_edit(rng, 5, ctx.as_deref_mut());

            // Randomly add, remove or mutate selection sets.
            let replica_selection_sets = &self
                .all_selections()
                .map(|(set_id, _)| *set_id)
                .filter(|set_id| self.replica_id == set_id.replica_id)
                .collect::<Vec<_>>();
            let set_id = replica_selection_sets.choose(rng);
            if set_id.is_some() && rng.gen_bool(1.0 / 6.0) {
                let op = self.remove_selection_set(*set_id.unwrap()).unwrap();
                operations.push(op);
            } else {
                let mut ranges = Vec::new();
                for _ in 0..5 {
                    let start = rng.gen_range(0..self.len() + 1);
                    let start_point = self.point_for_offset(start).unwrap();
                    let end = rng.gen_range(0..self.len() + 1);
                    let end_point = self.point_for_offset(end).unwrap();
                    ranges.push(start_point..end_point);
                }

                let op = if set_id.is_none() || rng.gen_bool(1.0 / 5.0) {
                    self.add_selection_set(ranges).unwrap().1
                } else {
                    self.replace_selection_set(*set_id.unwrap(), ranges)
                        .unwrap()
                };
                operations.push(op);
            }

            (old_ranges, new_text, operations)
        }

        pub fn randomly_undo_redo(
            &mut self,
            rng: &mut impl Rng,
            mut ctx: Option<&mut ModelContext<Self>>,
        ) -> Vec<Operation> {
            let mut ops = Vec::new();
            for _ in 0..rng.gen_range(0..5) {
                if let Some(edit_id) = self.edit_ops.keys().choose(rng).copied() {
                    ops.push(self.undo_or_redo(edit_id, ctx.as_deref_mut()).unwrap());
                }
            }
            ops
        }
    }

    impl Operation {
        fn edit_id(&self) -> Option<time::Local> {
            match self {
                Operation::Edit { edit, .. } => Some(edit.id),
                Operation::Undo { undo, .. } => Some(undo.edit_id),
                Operation::UpdateSelections { .. } => None,
            }
        }
    }

    fn line_lengths_in_range(buffer: &Buffer, range: Range<usize>) -> BTreeMap<u32, HashSet<u32>> {
        let mut lengths = BTreeMap::new();
        for (row, line) in buffer.text()[range].lines().enumerate() {
            lengths
                .entry(line.len() as u32)
                .or_insert(HashSet::default())
                .insert(row as u32);
        }
        if lengths.is_empty() {
            let mut rows = HashSet::default();
            rows.insert(0);
            lengths.insert(0, rows);
        }
        lengths
    }
}
