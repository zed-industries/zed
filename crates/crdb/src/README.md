# CRDB: A conflict-free replicated database for code and markdown

Our goal is for this database to contain all the text inserted in Zed.

## Contexts

The database is divided into *contexts*, with each context containing a collection of *documents*.

### Contexts contain documents

These contexts and the documents are really just namespaces in a global table of document *fragments*. Each fragment is a sequence of one or more characters, which may or may not be visible in a given branch.

#### Documents with paths are files

Documents in a context can be associated with metadata. If a document is associated with a relative path, it represents a file. A context that contains files can be synchronized with a directory tree on the file system, much like a Git repository.

#### Conversations are also documents

Contexts can also be associated with conversations, which are special documents that embed other documents that represent messages. Messages are embedded via a mechanism called *portals*, which will be discussed further below.

### Contexts occupy a hierarchical namespace

For example, at genesis, zed.dev will contain the following channels:

#zed
    - This is where people get oriented about what Zed is all about. We'll link to it from our landing page.
#zed/staff
    - Here's where we talk about stuff private to the company, and host company-specific files.
#zed/insiders
    - Users we've worked with.
#zed/zed
    - This contains the actual source code for Zed.
    - It also has a conversation where potential contributors can engage with us and each other.
#zed/zed/debugger
    - A subcontext of zed/zed where we talk about and eventually implement a debugger. Associated with a different branch of zed/zed where the debugger is being built, but could also have multiple branches. Branches and contexts are independent.

## Versions

Our goal is for each context to allow an arbitrary number of branches to be created, where each branch can be edited independently from other branches. Synchronization between branches can be deferred to a time of the user's choosing, like Git, or branches can be kept in sync in real time. Every branch is associated with a version.

A version identifies a subset of all operations that have been performed in a context. If we start from an empty context and applied the operations in this subset in any causally valid order, we arrive at the unique state of the context described by the version.

### How we use versions

We use versions to query the database. Since we're storing every operation, it isn't feasible to always load every operation in a context into a given replica.

Instead, we can query the database to return a snapshot of how it appears at a specific version. To do that, given the sequence of fragments, we need to efficiently query all fragments within that sequence that are visible at a given version.

* We want to exclude fragments from subsequent and concurrent versions.
* We want to include fragments introduced before the given version, but only those that are still visible.

We maintain a B-tree index for all fragments.

When querying this B-tree, we only want to descend into subtrees that contain at least one fragment whose appearance causally precedes the target version. If a node only contains fragments that were concurrent or subsequent to our target version, we can skip it. To support this, we store the minimal version that causally precedes all fragments in a given subtree on the subtree's summary.

Once a fragment appears, it can also disappear due to being deleted or undone. We want to avoid descending into nodes of a tree whose fragments appeared prior to our target version, but all of which have since become hidden.

To support this, we make a number of decisions. First, we avoid ever reintroducing a fragment once it has been hidden. If we undo an operation that hid a fragment, we will introduce a new fragment pointing at the same insertion, but with an id associated with the undo.

For each hidden fragment, we'll maintain an optional minimal version at which that fragment was hidden. When summarizing fragments that are all hidden, we'll maintain a minimal version at which all the fragments were hidden. If not all fragments in the summary are hidden, this version won't exist.

When deciding whether to descend into a subtree, we will check if all of its fragments became hidden in some version. If that version precedes our target version, we can skip that subtree.

### Required operations on version vectors

#### Insertion

Versions represent a set of operation ids, and we advance to a new version by inserting an operation id, where each operation id is associated with a branch id.

#### Partial ordering

Versions also represent causality. If version B contains a superset of operations in version A, then it can be considered to be greater than version A. That is, B represents a state that is after A.

If versions A and B represent concurrent operations, then the result of their comparison is `None`.

#### Join

*Joining* two arbitrary versions A and B produces a version C such that C is considered to be greater than both A and B. Joining produces a version that descends from A and B in the causality graph.

#### Meet

*Meeting* two arbitrary versions A and B produces a version C such that A and B are considered to both be greater than C. Meeting produces a version representing A and B's common ancestor in the causality graph.

#### Clone

Since we'll use versions in a copy-on-write B-tree, they must be efficiently cloned.

### Representing version vectors

Versions must support efficient implementations of the operations described above, plus efficient storage.

Each operation id combines a branch id with a Lamport timestamp derived from a Lamport clock that is maintained for each branch. We use Lamport timestamps instead of per-branch sequence numbers to allow operation ids to be ordered with respect to causality, which we use in the meet implementation.

Versions represent a subset of operations by mapping branch ids to the maximum Lamport timestamp for that branch in that version.

Versions also maintain a set of maximal operation ids, such that if version B contains all the maximal operation ids of A, then we know that version B happens before version A without further examination. This set represents the set of concurrent operations for which no causally subsequent operations exist in the set of operations represented by the version.


The version graph is actually a CRDT.

```rust
struct Db {
    versions: OrderedMap<OperationId, Version>,
}

struct Version {
    /// A set of concurrent operations that causally dominate all
    /// other operations in this version.
    maximal_operations: SmallVec<[OperationId; 2]>, // TODO: Is this the right data type

    /// The maximum Lamport time observed for every branch in this
    /// version
    operations: OrderedMap<BranchId, LamportTime>,
}

struct OperationId {
    branch_id: BranchId,
    time: LamportTime,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.maximal_operations == other.maximal_operations {
            Some(Ordering::Equal)
        } else if self.maximal_operations.all(|op_id| other.contains(&op_id)) {
            Some(Ordering::Less)
        } else if other.maximal_operations.all(|op_id| self.contains(&op_id)) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl Version {
    pub fn observe(&mut self, operation: BranchId) {
        let operation_count = if let Some(prev_count) = self.operations.get(&branch) {
            let operation_count = prev_count + 1;
            self.operations.insert(branch, operation_count);
            operation_count
        } else {
            self.operations.insert(branch, 1);
            1
        };

        OperationId {
            branch,
            operation_count,
        }
    }

    /// Return a version that is >= both self and other.
    pub fn join(&self, other: &Self) -> Self {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) | Some(Ordering::Greater) => self.clone(),
            Some(Ordering::Less) => other.clone(),
            None => {
                // merge the operations and maximal operations
                // remove redundancy in maximal operations
            }
        }
    }

    /// Return a version that is <= both self and other.
    pub fn meet(&self, other: &Self) -> Self {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) | Some(Ordering::Greater) => other.clone(),
            Some(Ordering::Less) => self.clone(),
            None => {
                // intersect the operations
                // the maximal operation is the operation with the max lamport time
            }
        }
    }
}


#[test]
fn test_operations() {
    let branch_1 = todo!();
    let branch_2 = todo!();

    let mut version_a = Version::default();
    let mut version_b = version_a.clone();
    version_a.operation()
    version_b.operation()
    let met_version = version_a.meet(&version_b);

    assert!(met_version <= version_a);
    assert!(met_version <= version_b);
}

fn receive_op(&mut self, operation: &Operation) {
    let version = self.versions.observe(&operation.parents, operation.id)

    Fragment::new(self.version.clone());

}
```
