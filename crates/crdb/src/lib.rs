mod document_fragment;
mod insertion_subrange;

/// An efficiently-editable an cloneable chunk of text.
use rope::Rope;
use std::{cmp::Reverse, ops::Range};

/// These names seem more descriptive. Perhaps we should consider renaming these
/// within the sum_tree crate?
type OrderedMap<K, T> = sum_tree::TreeMap<K, T>;
type Sequence<T> = sum_tree::SumTree<T>;

/// All the state in the system.
/// Only a subset is actually resident.
pub struct Db {
    document_fragments: Sequence<DocumentFragment>,
    insertion_fragments: OrderedMap<InsertionSubrange, DocumentFragmentId>,
}

/// A group of documents with a permissions boundary.
///
/// This type is actually a windowed reference to `Db`.
pub struct Context {}

/// A coherent body of editable text.
/// This type is actually a reference to `Context`.
pub struct Document {}

/// A chunk of immutable text inside a document.
/// When text is initially inserted, it is a single document
/// fragment, which is then subsequently split into smaller fragments
/// upon further editing.
#[derive(Clone, Debug)]
pub struct DocumentFragment {
    id: DocumentFragmentId,
    insertion_subrange: InsertionSubrange,
}

/// Globally identifies the fragment in an installation, but this
/// value is not intended to be transmitted over the network.
#[derive(Clone, Default, Debug)]
struct DocumentFragmentId {
    document: DocumentId,
    index: Ordering,
}

/// Documents are identified with their creation operation.
pub type DocumentId = OperationId;

/// Insertions get subdivided by subsequent edits. This datatype tracks those subdivisions.
/// Subranges are continuous and disjoint.
#[derive(Eq, PartialEq, Clone, Default, Debug)]
struct InsertionSubrange {
    insertion: InsertionId,
    range: Range<usize>,
}

/// A unique identifier for a change in state applied on a given branch.
#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct OperationId {
    branch: BranchId,
    op_count: OperationCount,
    causality: Reverse<LamportTime>,
}

/// An insertion is identified with its occurrence.
type InsertionId = OperationId;

/// Causal ordering between events produced on different branches.
type LamportTime = u64;

/// Each installation can create new contexts independently.
#[derive(Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ContextId(InstallationId, ContextCount);

/// Each installation can create new branches independently for each context
#[derive(Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct BranchId(ContextId, InstallationId, BranchCount);

/// Assigned by our server on first connection.
type InstallationId = u32;

/// The number of operations created on a specific branch.
type OperationCount = u32;

/// The number of contexts created by a given user.
type ContextCount = u32;

/// The number of contexts created by a given user.
type BranchCount = u32;

/// A logical position in a document.
///
/// If you create an anchor to a word and the document is subsequently edited,
/// the anchor still resolves to the same word. If the word has been deleted,
/// the anchor resolves to the word's tombstone, which is the closest location
/// to where the word would be if it hadn't been deleted.
pub struct Anchor {
    insertion_id: InsertionId,
    offset: usize,
    bias: Bias,
}

/// Controls whether the anchor is pushed rightward by subsequent insertions
/// occurring at the location of the anchor.
pub enum Bias {
    Left,
    Right,
}

/// A dense, totally-ordered CRDT, such as LSEQ.
#[derive(Clone, Default, Debug)]
struct Ordering {}

/// An operation sent or received over the network.
pub enum Operation {
    DocumentCreation(DocumentCreation),
    Insertion(Insertion),
}

pub struct DocumentCreation {
    id: OperationId,
}

pub struct Insertion {
    id: OperationId,
    position: Anchor,
    text: Rope,
}
