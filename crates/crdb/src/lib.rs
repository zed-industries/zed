mod document_fragment;

use std::{cmp::Reverse, ops::Range};
use sum_tree::{SumTree, TreeMap};
type OrderedMap<K, T> = TreeMap<K, T>;

/// All the state
pub struct Db {
    document_fragments: Sequence<DocumentFragment>,
    insertion_fragments: OrderedMap<InsertionRange, DocumentFragmentId>,
}

#[derive(Eq, PartialEq, Clone, Default, Debug)]
struct InsertionRange {
    insertion: InsertionId,
    range: Range<usize>,
}

impl Ord for InsertionRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for InsertionRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.insertion
                .cmp(&other.insertion)
                .then_with(|| self.range.start.cmp(&other.range.start)),
        )
    }
}

type Sequence<T> = SumTree<T>;
type DocumentFragmentId = Ordering;

#[derive(Clone, Debug)]
pub struct DocumentFragment {
    id: DocumentFragmentId,
    insertion: InsertionId,
    text: Rope,
}

pub struct InsertionFragment {}

/// A group of documents with a permissions boundary.
///
/// This type is actually a windowed reference to `Db`.
pub struct Context {}

/// A coherent body of editable text.
pub struct Document {}

/// The document is identified with its creation operation.
pub type DocumentId = OperationId;

/// A sequence of one or more characters inserted into a `Document`
pub struct Insertion {
    id: OperationId,
    position: Anchor,
    text: Rope,
}

/// An insertion is identified with its occurrence.
type InsertionId = OperationId;

/// A unique identifier for a change in state applied on a given branch.
#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct OperationId {
    branch: BranchId,
    op_count: OperationCount,
    causality: Reverse<LamportTime>,
}

/// Causal ordering between events produced on different branches.
type LamportTime = u64;

/// How many operations have occurred on a given branch?
struct OperationCount(u32);

/// Unique to each context on each installation.
struct BranchId {
    context: ContextId,
    installation: InstallationId,
}

/// Assigned by our server on first connection.
type InstallationId = u32;

/// Handed out by our server on context creation.
/// We could potentially make this amenable to distribution,
struct ContextId(UserId, ContextCount);

/// We'll need a bigger data type before we know it.
type UserId = u32;

/// The number of contexts created by a given user.
type ContextCount = u32;

/// A logical "anchor point" into a document.
/// If you create an anchor to a word and its containing document
/// is subsequently edited, the anchor still resolves to the same word.
/// If the word has been deleted, the anchor resolves to the site of its tombstone.
struct Anchor {
    insertion_id: InsertionId,
}

/// A dense, totally-ordered CRDT, such as LSEQ.
#[derive(Clone, Default, Debug)]
struct Ordering {}

/// An efficiently-editable an cloneable chunk of text.
type Rope = rope::Rope;
