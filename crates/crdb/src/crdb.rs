mod dense_id;

use dense_id::DenseId;
use parking_lot::Mutex;
use rope::Rope;
use smallvec::{smallvec, SmallVec};
use std::{cmp::Ordering, ops::Range, path::Path, sync::Arc};
use sum_tree::{Bias, SumTree, TreeMap};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct RepoId(Uuid);

impl RepoId {
    fn new() -> Self {
        RepoId(Uuid::new_v4())
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct ReplicaId(u32);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct OperationCount(usize);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct OperationId {
    replica_id: ReplicaId,
    operation_count: OperationCount,
}

impl OperationId {
    fn tick(&mut self) -> OperationId {
        self.operation_count.0 += 1;
        *self
    }
}

impl sum_tree::Summary for OperationId {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        debug_assert!(summary > self);
        *self = *summary;
    }
}

#[derive(Clone, Default)]
pub struct Db {
    snapshot: Arc<Mutex<DbSnapshot>>,
}

impl Db {
    pub fn create_repo(&self) -> Repo {
        let id = RepoId::new();
        let snapshot = RepoSnapshot::default();
        let repo = Repo {
            id,
            db: self.clone(),
        };
        self.snapshot.lock().repos.insert(id, snapshot);
        repo
    }
}

#[derive(Clone, Default)]
struct DbSnapshot {
    repos: TreeMap<RepoId, RepoSnapshot>,
}

#[derive(Clone)]
pub struct Repo {
    id: RepoId,
    db: Db,
}

impl Repo {
    fn create_document(&self) -> Document {
        self.update(|repo| {
            let document_id = repo.last_operation_id.tick();

            let mut cursor = repo.document_fragments.cursor::<OperationId>();
            let mut new_document_fragments = cursor.slice(&document_id, Bias::Right, &());
            new_document_fragments.push(
                DocumentFragment {
                    document_id,
                    location: DenseId::min(),
                    insertion_id: document_id,
                    insertion_subrange: 0..0,
                    visible: true,
                    tombstones: Default::default(),
                    undo_count: 0,
                },
                &(),
            );
            new_document_fragments.append(cursor.suffix(&()), &());
            drop(cursor);

            repo.document_fragments = new_document_fragments;
            repo.document_metadata.insert(
                document_id,
                DocumentMetadata {
                    path: None,
                    last_change: document_id,
                },
            );

            Document {
                id: document_id,
                repo: self.clone(),
            }
        })
    }

    fn update<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RepoSnapshot) -> T,
    {
        self.db
            .snapshot
            .lock()
            .repos
            .update(&self.id, f)
            .expect("repo must exist")
    }
}

#[derive(Clone, Debug, Default)]
pub struct RepoSnapshot {
    head: OperationId,
    last_operation_id: OperationId,
    document_metadata: TreeMap<OperationId, DocumentMetadata>,
    document_fragments: SumTree<DocumentFragment>,
    insertion_fragments: SumTree<InsertionFragment>,
    visible_text: Rope,
    hidden_text: Rope,
    operations: SumTree<Operation>,
}

#[derive(Clone, Debug)]
struct DocumentMetadata {
    path: Option<Arc<Path>>,
    last_change: OperationId,
}

#[derive(Clone, Debug)]
struct DocumentFragment {
    document_id: OperationId,
    location: DenseId,
    insertion_id: OperationId,
    insertion_subrange: Range<usize>,
    visible: bool,
    tombstones: SmallVec<[Tombstone; 2]>,
    undo_count: u16,
}

impl DocumentFragment {
    fn len(&self) -> usize {
        self.insertion_subrange.end - self.insertion_subrange.start
    }
}

impl sum_tree::Item for DocumentFragment {
    type Summary = DocumentFragmentSummary;

    fn summary(&self) -> DocumentFragmentSummary {
        DocumentFragmentSummary {
            visible_len: if self.visible { self.len() } else { 0 },
            hidden_len: if self.visible { 0 } else { self.len() },
            max_document_id: self.document_id,
            max_location: self.location.clone(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct DocumentFragmentSummary {
    visible_len: usize,
    hidden_len: usize,
    max_document_id: OperationId,
    max_location: DenseId,
}

impl sum_tree::Summary for DocumentFragmentSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.visible_len += summary.visible_len;
        self.hidden_len += summary.hidden_len;

        debug_assert!(summary.max_document_id >= self.max_document_id);
        self.max_document_id = summary.max_document_id;

        debug_assert!(
            summary.max_document_id > self.max_document_id
                || summary.max_location > self.max_location
        );
        self.max_location = summary.max_location.clone();
    }
}

impl<'a> sum_tree::Dimension<'a, DocumentFragmentSummary> for OperationId {
    fn add_summary(&mut self, summary: &'a DocumentFragmentSummary, _: &()) {
        *self = summary.max_document_id
    }
}

#[derive(Clone, Debug)]
struct Tombstone {
    id: OperationId,
    undo_count: u16,
}

#[derive(Clone, Debug)]
struct InsertionFragment {
    insertion_id: OperationId,
    offset_in_insertion: usize,
    fragment_location: DenseId,
}

impl sum_tree::Item for InsertionFragment {
    type Summary = InsertionFragmentSummary;

    fn summary(&self) -> Self::Summary {
        InsertionFragmentSummary {
            max_insertion_id: self.insertion_id,
            max_offset_in_insertion: self.offset_in_insertion,
        }
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
struct InsertionFragmentSummary {
    max_insertion_id: OperationId,
    max_offset_in_insertion: usize,
}

impl sum_tree::Summary for InsertionFragmentSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        debug_assert!(summary.max_insertion_id >= self.max_insertion_id);
        self.max_insertion_id = summary.max_insertion_id;

        debug_assert!(
            summary.max_insertion_id > self.max_insertion_id
                || summary.max_offset_in_insertion > self.max_offset_in_insertion
        );
        self.max_offset_in_insertion = summary.max_offset_in_insertion;
    }
}

struct Document {
    repo: Repo,
    id: OperationId,
}

impl Document {
    pub fn edit<E, I, T>(&mut self, edits: E) -> Operation
    where
        E: IntoIterator<IntoIter = I>,
        I: ExactSizeIterator<Item = (Range<usize>, T)>,
        T: Into<Arc<str>>,
    {
        self.repo.update(|repo| {
            let edits = edits
                .into_iter()
                .map(|(range, new_text)| (range, new_text.into())).peekable();

            let mut edit_op = EditOperation {
                id: repo.last_operation_id.tick(),
                parent_ids: smallvec![repo.head],
                edits: SmallVec::with_capacity(edits.len()),
            };
            let mut new_insertions = Vec::new();
            let mut insertion_offset = 0;
            let mut insertion_slices = Vec::new();

            #[derive(Clone, Debug, Default)]
            pub struct LocalEditDimension {
                visible_len: usize,
                hidden_len: usize,
                max_document_id: OperationId,
            }

            impl<'a> sum_tree::Dimension<'a, DocumentFragmentSummary> for LocalEditDimension {
                fn add_summary(&mut self, summary: &'a DocumentFragmentSummary, _: &()) {
                    self.visible_len += summary.visible_len;
                    self.hidden_len += summary.hidden_len;
                    debug_assert!(summary.max_document_id >= self.max_document_id);
                    self.max_document_id = summary.max_document_id;
                }
            }

            impl<'a> sum_tree::SeekTarget<'a, DocumentFragmentSummary, LocalEditDimension>
                for OperationId
            {
                fn cmp(&self, cursor_location: &LocalEditDimension, cx: &()) -> Ordering {
                    Ord::cmp(self, &cursor_location.max_document_id)
                }
            }

            impl<'a> sum_tree::SeekTarget<'a, DocumentFragmentSummary, LocalEditDimension>
                for usize
            {
                fn cmp(&self, cursor_location: &LocalEditDimension, cx: &()) -> Ordering {
                    Ord::cmp(self, &cursor_location.visible_len)
                }
            }

            let mut old_fragments = repo.document_fragments.cursor::<LocalEditDimension>();
            let mut new_fragments =
                old_fragments.slice(&self.id, Bias::Right, &());
            let document_visible_start = old_fragments.start().visible_len;
            let document_hidden_start = old_fragments.start().hidden_len;

            let mut new_ropes = RopeBuilder::new(
                repo.visible_text.cursor(document_visible_start),
                repo.hidden_text.cursor(document_hidden_start)
            );
            let mut new_fragments =
                old_fragments.slice(&(document_visible_start + edits.peek().unwrap().0.start), Bias::Right, &());

            new_ropes.append(new_fragments.summary().visible_len, new_fragments.summary().hidden_len);

            let mut fragment_start = old_fragments.start().visible_len;
            for (range, new_text) in edits {
                let new_text = LineEnding::normalize_arc(new_text.into());
                let fragment_end = old_fragments.end(&None).visible;

                // If the current fragment ends before this range, then jump ahead to the first fragment
                // that extends past the start of this range, reusing any intervening fragments.
                if fragment_end < range.start {
                    // If the current fragment has been partially consumed, then consume the rest of it
                    // and advance to the next fragment before slicing.
                    if fragment_start > old_fragments.start().visible {
                        if fragment_end > fragment_start {
                            let mut suffix = old_fragments.item().unwrap().clone();
                            suffix.len = fragment_end - fragment_start;
                            suffix.insertion_offset +=
                                fragment_start - old_fragments.start().visible;
                            new_insertions.push(InsertionFragment::insert_new(&suffix));
                            new_ropes.push_fragment(&suffix, suffix.visible);
                            new_fragments.push(suffix, &None);
                        }
                        old_fragments.next(&None);
                    }

                    let slice = old_fragments.slice(&range.start, Bias::Right, &None);
                    new_ropes.append(slice.summary().text);
                    new_fragments.append(slice, &None);
                    fragment_start = old_fragments.start().visible;
                }

                let full_range_start = FullOffset(range.start + old_fragments.start().deleted);

                // Preserve any portion of the current fragment that precedes this range.
                if fragment_start < range.start {
                    let mut prefix = old_fragments.item().unwrap().clone();
                    prefix.len = range.start - fragment_start;
                    prefix.insertion_offset += fragment_start - old_fragments.start().visible;
                    prefix.id = Locator::between(&new_fragments.summary().max_id, &prefix.id);
                    new_insertions.push(InsertionFragment::insert_new(&prefix));
                    new_ropes.push_fragment(&prefix, prefix.visible);
                    new_fragments.push(prefix, &None);
                    fragment_start = range.start;
                }

                // Insert the new text before any existing fragments within the range.
                if !new_text.is_empty() {
                    let new_start = new_fragments.summary().text.visible;

                    let fragment = Fragment {
                        id: Locator::between(
                            &new_fragments.summary().max_id,
                            old_fragments
                                .item()
                                .map_or(&Locator::max(), |old_fragment| &old_fragment.id),
                        ),
                        insertion_timestamp: timestamp,
                        insertion_offset,
                        len: new_text.len(),
                        deletions: Default::default(),
                        max_undos: Default::default(),
                        visible: true,
                    };
                    edits_patch.push(EditOperation {
                        old: fragment_start..fragment_start,
                        new: new_start..new_start + new_text.len(),
                    });
                    insertion_slices.push(fragment.insertion_slice());
                    new_insertions.push(InsertionFragment::insert_new(&fragment));
                    new_ropes.push_str(new_text.as_ref());
                    new_fragments.push(fragment, &None);
                    insertion_offset += new_text.len();
                }

                // Advance through every fragment that intersects this range, marking the intersecting
                // portions as deleted.
                while fragment_start < range.end {
                    let fragment = old_fragments.item().unwrap();
                    let fragment_end = old_fragments.end(&None).visible;
                    let mut intersection = fragment.clone();
                    let intersection_end = cmp::min(range.end, fragment_end);
                    if fragment.visible {
                        intersection.len = intersection_end - fragment_start;
                        intersection.insertion_offset +=
                            fragment_start - old_fragments.start().visible;
                        intersection.id =
                            Locator::between(&new_fragments.summary().max_id, &intersection.id);
                        intersection.deletions.insert(timestamp.local());
                        intersection.visible = false;
                    }
                    if intersection.len > 0 {
                        if fragment.visible && !intersection.visible {
                            let new_start = new_fragments.summary().text.visible;
                            edits_patch.push(EditOperation {
                                old: fragment_start..intersection_end,
                                new: new_start..new_start,
                            });
                            insertion_slices.push(intersection.insertion_slice());
                        }
                        new_insertions.push(InsertionFragment::insert_new(&intersection));
                        new_ropes.push_fragment(&intersection, fragment.visible);
                        new_fragments.push(intersection, &None);
                        fragment_start = intersection_end;
                    }
                    if fragment_end <= range.end {
                        old_fragments.next(&None);
                    }
                }

                let full_range_end = FullOffset(range.end + old_fragments.start().deleted);
                edit_op.edits.push(full_range_start..full_range_end);
                edit_op.new_text.push(new_text);
            }

            // If the current fragment has been partially consumed, then consume the rest of it
            // and advance to the next fragment before slicing.
            if fragment_start > old_fragments.start().visible {
                let fragment_end = old_fragments.end(&None).visible;
                if fragment_end > fragment_start {
                    let mut suffix = old_fragments.item().unwrap().clone();
                    suffix.len = fragment_end - fragment_start;
                    suffix.insertion_offset += fragment_start - old_fragments.start().visible;
                    new_insertions.push(InsertionFragment::insert_new(&suffix));
                    new_ropes.push_fragment(&suffix, suffix.visible);
                    new_fragments.push(suffix, &None);
                }
                old_fragments.next(&None);
            }

            let suffix = old_fragments.suffix(&None);
            new_ropes.append(suffix.summary().text);
            new_fragments.append(suffix, &None);
            let (visible_text, deleted_text) = new_ropes.finish();
            drop(old_fragments);

            repo.snapshot.fragments = new_fragments;
            repo.snapshot.insertions.edit(new_insertions, &());
            repo.snapshot.visible_text = visible_text;
            repo.snapshot.deleted_text = deleted_text;
            repo.history
                .insertion_slices
                .insert(timestamp.local(), insertion_slices);

            Operation::Edit(edit_op)
        })
    }
}

#[derive(Clone, Debug)]
enum Operation {
    CreateDocument(CreateDocumentOperation),
    Edit(EditOperation),
}

impl Operation {
    fn id(&self) -> OperationId {
        match self {
            Operation::CreateDocument(op) => op.id,
            Operation::Edit(op) => op.id,
        }
    }

    fn parent_ids(&self) -> &[OperationId] {
        match self {
            Operation::CreateDocument(op) => &op.parent_ids,
            Operation::Edit(op) => &op.parent_ids,
        }
    }
}

impl sum_tree::Item for Operation {
    type Summary = OperationId;

    fn summary(&self) -> Self::Summary {
        self.id()
    }
}

impl sum_tree::KeyedItem for Operation {
    type Key = OperationId;

    fn key(&self) -> Self::Key {
        self.id()
    }
}

#[derive(Clone, Debug)]
struct CreateDocumentOperation {
    id: OperationId,
    parent_ids: SmallVec<[OperationId; 2]>,
}

#[derive(Clone, Debug)]
struct EditOperation {
    id: OperationId,
    parent_ids: SmallVec<[OperationId; 2]>,
    edits: SmallVec<[(Range<Anchor>, Arc<str>); 2]>,
}

#[derive(Copy, Clone, Debug)]
struct Anchor {
    insertion_id: OperationId,
    offset_in_insertion: usize,
    bias: Bias,
}

struct RopeBuilder<'a> {
    old_visible_cursor: rope::Cursor<'a>,
    old_deleted_cursor: rope::Cursor<'a>,
    new_visible: Rope,
    new_deleted: Rope,
}

impl<'a> RopeBuilder<'a> {
    fn new(old_visible_cursor: rope::Cursor<'a>, old_deleted_cursor: rope::Cursor<'a>) -> Self {
        Self {
            old_visible_cursor,
            old_deleted_cursor,
            new_visible: Rope::new(),
            new_deleted: Rope::new(),
        }
    }

    fn append(&mut self, visible_len: usize, hidden_len: usize) {
        self.push(visible_len, true, true);
        self.push(hidden_len, false, false);
    }

    fn push_fragment(&mut self, fragment: &DocumentFragment, was_visible: bool) {
        debug_assert!(fragment.len() > 0);
        self.push(fragment.len(), was_visible, fragment.visible)
    }

    fn push(&mut self, len: usize, was_visible: bool, is_visible: bool) {
        let text = if was_visible {
            self.old_visible_cursor
                .slice(self.old_visible_cursor.offset() + len as usize)
        } else {
            self.old_deleted_cursor
                .slice(self.old_deleted_cursor.offset() + len)
        };
        if is_visible {
            self.new_visible.append(text);
        } else {
            self.new_deleted.append(text);
        }
    }

    fn push_str(&mut self, text: &str) {
        self.new_visible.push(text);
    }

    fn finish(mut self) -> (Rope, Rope) {
        self.new_visible.append(self.old_visible_cursor.suffix());
        self.new_deleted.append(self.old_deleted_cursor.suffix());
        (self.new_visible, self.new_deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo() {
        let db = Db::default();
        let repo = db.create_repo();
        let doc = repo.create_document();

        // doc.edit()
    }
}
