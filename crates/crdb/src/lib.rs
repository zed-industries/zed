mod clock;
mod document_fragment;
mod insertion_subrange;
mod version;

/// An efficiently-editable an cloneable chunk of text.
use rope::Rope;
use std::{
    ops::Range,
    sync::{Arc, Mutex},
};

/// These names seem more descriptive. Perhaps we should consider renaming these
/// within the sum_tree crate?
type OrderedMap<K, T> = sum_tree::TreeMap<K, T>;
type OrderedSet<K> = sum_tree::TreeSet<K>;
type Sequence<T> = sum_tree::SumTree<T>;

/// All the state in the system.
/// Only a subset is actually resident.
pub struct Db(Arc<Mutex<DbSnapshot>>);

#[derive(Clone)]
pub struct DbSnapshot {
    document_fragments: Sequence<DocumentFragment>,
    insertion_fragments: OrderedMap<InsertionSubrange, DocumentFragmentId>,
}

/// A group of documents with a permissions boundary. Can either be a worktree
/// or a channel.
///
/// This type is actually a reference to `Db` that scopes all interaction to
/// a specific context.
pub struct Context {
    id: ContextId,
}

/// A coherent body of editable text.
/// This type is actually a reference to `Context`.
pub struct Document {
    id: DocumentId,
}

/// The ability to efficiently represent and compare version vectors is a fundamental
/// assumption of the system as it is currently designed.
///
/// Is there any way to exploit structural sharing? For representation, yes.
///
/// But what about comparison. Can we phrase comparison in terms of reachability over a DAG?
pub struct VersionVector {
    graph: VersionGraph,
    current: OrderedMap<BranchId, OperationCount>,
    previous: OrderedMap<BranchId, OperationCount>,
}

#[derive(Clone)]
pub struct VersionGraph;

/// A chunk of immutable text inside a document.
/// When text is initially inserted, it is a single document
/// fragment, which is then subsequently split into smaller fragments
/// upon further editing.
#[derive(Clone, Debug)]
pub struct DocumentFragment {
    id: DocumentFragmentId,
    insertion_subrange: InsertionSubrange,
    insertion_time: LamportTime,
}

/// Globally identifies the fragment in an installation, but this
/// value is not intended to be transmitted over the network.
#[derive(Clone, Default, Debug)]
struct DocumentFragmentId {
    document: DocumentId,
    index: DenseIndex,
}

/// Documents are identified with their creation operation in a context.
#[derive(Clone, Default, Debug)]
pub struct DocumentId {
    // OperationId alone would be sufficient for uniqueness, but adding the
    // context id groups documents in the same context together in the sequence.
    context: ContextId,
    creation: OperationId,
}

/// Insertions get subdivided by subsequent edits. This datatype tracks those subdivisions.
/// Subranges are continuous and disjoint.
#[derive(Eq, PartialEq, Clone, Default, Debug)]
struct InsertionSubrange {
    insertion: InsertionId,
    range: Range<usize>,
}

/// A unique identifier for a change in state applied on a given branch.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct OperationId {
    branch: BranchId,
    time: LamportTime,
}

/// An insertion is identified with the insert operation, and we
/// add the time so we can enforce a causal ordering.
type InsertionId = OperationId;

/// Causal ordering between events produced on different branches.
type LamportTime = u64;

/// Each replica can create new contexts independently.
#[derive(Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ContextId(ReplicaId, ContextCount);

/// Each replica can create new branches independently for each context
#[derive(Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct BranchId(ReplicaId, ContextId, BranchCount);

/// Assigned by our server on first connection.
type ReplicaId = u32;

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
struct DenseIndex {}

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
    time: LamportTime,
    position: Anchor,
    text: Rope,
}
