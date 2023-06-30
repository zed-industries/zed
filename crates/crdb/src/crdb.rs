mod dense_id;
mod messages;
mod operations;
#[cfg(test)]
mod test;

use anyhow::Result;
use dense_id::DenseId;
use futures::future::BoxFuture;
use operations::{CreateBranch, Operation};
use parking_lot::Mutex;
use rope::Rope;
use smallvec::{smallvec, SmallVec};
use std::{
    cmp::{self, Ordering},
    fmt::Debug,
    future::Future,
    ops::Range,
    path::Path,
    sync::Arc,
};
use sum_tree::{Bias, SumTree, TreeMap};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RepoId(Uuid);

type RevisionId = SmallVec<[OperationId; 2]>;

impl RepoId {
    fn new() -> Self {
        RepoId(Uuid::new_v4())
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReplicaId(u32);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct OperationCount(usize);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationId {
    replica_id: ReplicaId,
    operation_count: OperationCount,
}

impl OperationId {
    pub fn new(replica_id: ReplicaId) -> Self {
        Self {
            replica_id,
            operation_count: OperationCount::default(),
        }
    }

    pub fn tick(&mut self) -> OperationId {
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

#[derive(Clone)]
pub struct RoomName(Arc<str>);

#[derive(Clone)]
pub struct RoomToken(Arc<str>);

pub trait Request: 'static {
    type Response: 'static;
}

pub trait Message {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait ServerNetwork {
    fn on_request<H, F, R>(&self, handle_request: H)
    where
        H: 'static + Fn(R) -> F,
        F: 'static + Send + Sync + futures::Future<Output = Result<R::Response>>,
        R: Request;
}

pub trait ClientNetwork {
    fn request<R: Request>(&self, request: R) -> BoxFuture<Result<R::Response>>;
    fn broadcast<M: Message>(&self, room: RoomName, token: RoomToken, message: M);
}

struct Client<N> {
    db: Db,
    network: Arc<N>,
    repo_room_credentials: Arc<Mutex<collections::HashMap<RepoId, RoomCredentials>>>,
}

struct RoomCredentials {
    name: RoomName,
    token: RoomToken,
}

impl<N: ClientNetwork> Clone for Client<N> {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            network: self.network.clone(),
            repo_room_credentials: Default::default(),
        }
    }
}

impl<N: 'static + ClientNetwork> Client<N> {
    pub fn new(network: N) -> Self {
        let mut this = Self {
            db: Db::new(),
            network: Arc::new(network),
            repo_room_credentials: Default::default(),
        };
        this.db.on_local_operation({
            let this = this.clone();
            move |repo_id, operation| this.handle_local_operation(repo_id, operation)
        });
        this
    }

    pub fn create_repo(&self) -> Repo {
        let id = RepoId::new();
        let snapshot = RepoSnapshot::default();
        let repo = Repo {
            id,
            db: self.db.clone(),
        };
        self.db.snapshot.lock().repos.insert(id, snapshot);
        repo
    }

    pub fn clone_repo(&self, name: impl Into<Arc<str>>) -> impl Future<Output = Result<Repo>> {
        async move { todo!() }
    }

    pub fn publish_repo(
        &self,
        repo: &Repo,
        name: impl Into<Arc<str>>,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        let id = repo.id;
        let name = name.into();
        async move {
            this.network
                .request(messages::PublishRepo { id, name })
                .await?;
            Ok(())
        }
    }

    fn handle_local_operation(&self, repo_id: RepoId, operation: Operation) {
        if let Some(credentials) = self.repo_room_credentials.lock().get(&repo_id) {
            self.network.broadcast(
                credentials.name.clone(),
                credentials.token.clone(),
                operation,
            );
        }
    }
}

#[derive(Clone)]
struct Server {
    db: Db,
}

impl Server {
    async fn new(network: impl ServerNetwork) -> Self {
        let this = Self { db: Db::new() };
        // network.on_request({
        //     let this = this.clone();
        //     move |request| {
        //         let this = this.clone();
        //         async move { todo!() }
        //     }
        // });
        this
    }
}

#[derive(Clone)]
pub struct Db {
    snapshot: Arc<Mutex<DbSnapshot>>,
    local_operation_created: Option<Arc<dyn Fn(RepoId, Operation)>>,
}

impl Db {
    fn new() -> Self {
        Self {
            snapshot: Default::default(),
            local_operation_created: None,
        }
    }

    fn on_local_operation(&mut self, operation_created: impl 'static + Fn(RepoId, Operation)) {
        self.local_operation_created = Some(Arc::new(operation_created));
    }
}

#[derive(Clone)]
pub struct Repo {
    id: RepoId,
    db: Db,
}

impl Repo {
    fn create_empty_branch(&self, name: impl Into<Arc<str>>) -> Branch {
        let branch_id = self.update(|repo| repo.create_empty_branch(name));
        Branch {
            id: branch_id,
            repo: self.clone(),
        }
    }

    fn read<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&RepoSnapshot) -> T,
    {
        let snapshot = self.db.snapshot.lock();
        let repo = snapshot.repos.get(&self.id).expect("repo must exist");
        f(repo)
    }

    fn update<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut RepoSnapshot) -> (Operation, T),
    {
        self.db
            .snapshot
            .lock()
            .repos
            .update(&self.id, |repo| {
                let (operation, result) = f(repo);
                repo.operations.insert(operation.id(), operation.clone());
                if let Some(operation_created) = self.db.local_operation_created.as_ref() {
                    operation_created(self.id, operation);
                }
                result
            })
            .expect("repo must exist")
    }
}

#[derive(Clone)]
struct Branch {
    id: OperationId,
    repo: Repo,
}

impl Branch {
    pub fn create_document(&self) -> Document {
        self.update(|document_id, parent, revision| {
            let mut cursor = revision.document_fragments.cursor::<OperationId>();
            let mut new_document_fragments = cursor.slice(&document_id, Bias::Right, &());
            new_document_fragments.push(
                DocumentFragment {
                    document_id,
                    location: DenseId::min(),
                    insertion_id: document_id,
                    insertion_subrange: 0..0,
                    tombstones: Default::default(),
                    undo_count: 0,
                },
                &(),
            );
            new_document_fragments.append(cursor.suffix(&()), &());
            drop(cursor);

            revision.document_fragments = new_document_fragments;
            revision.document_metadata.insert(
                document_id,
                DocumentMetadata {
                    path: None,
                    last_change: document_id,
                },
            );

            let operation = Operation::CreateDocument(operations::CreateDocument {
                id: document_id,
                parent,
            });
            let document = Document {
                id: document_id,
                branch: self.clone(),
            };

            (operation, document)
        })
    }

    fn update<F, T>(&self, f: F) -> T
    where
        F: FnOnce(OperationId, RevisionId, &mut Revision) -> (Operation, T),
    {
        self.repo.update(|repo| {
            let head = repo
                .branches
                .get(&self.id)
                .expect("branch must exist")
                .head
                .clone();
            let mut revision = repo
                .revisions
                .get(&head)
                .expect("revision must exist")
                .clone();
            let operation_id = repo.last_operation_id.tick();
            let (operation, result) = f(operation_id, head.clone(), &mut revision);
            repo.branches
                .update(&self.id, |branch| branch.head = smallvec![operation_id]);
            repo.revisions.insert(smallvec![operation_id], revision);
            (operation, result)
        })
    }

    fn read<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&Revision) -> T,
    {
        self.repo.read(|repo| {
            let head = &repo.branches.get(&self.id).expect("branch must exist").head;
            let revision = repo.revisions.get(head).expect("revision must exist");
            f(revision)
        })
    }
}

#[derive(Clone, Default)]
struct DbSnapshot {
    repos: TreeMap<RepoId, RepoSnapshot>,
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
    tombstones: SmallVec<[Tombstone; 2]>,
    undo_count: u16,
}

impl DocumentFragment {
    fn len(&self) -> usize {
        self.insertion_subrange.end - self.insertion_subrange.start
    }

    fn visible(&self) -> bool {
        self.undo_count % 2 == 0
            && self
                .tombstones
                .iter()
                .all(|tombstone| tombstone.undo_count % 2 == 1)
    }
}

impl sum_tree::Item for DocumentFragment {
    type Summary = DocumentFragmentSummary;

    fn summary(&self) -> DocumentFragmentSummary {
        DocumentFragmentSummary {
            visible_len: if self.visible() { self.len() } else { 0 },
            hidden_len: if self.visible() { 0 } else { self.len() },
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
        debug_assert!(summary.max_document_id >= self.max_document_id);
        debug_assert!(
            summary.max_document_id > self.max_document_id
                || summary.max_location > self.max_location
        );

        self.visible_len += summary.visible_len;
        self.hidden_len += summary.hidden_len;
        self.max_document_id = summary.max_document_id;
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

impl InsertionFragment {
    fn new(fragment: &DocumentFragment) -> Self {
        Self {
            insertion_id: fragment.insertion_id,
            offset_in_insertion: fragment.insertion_subrange.start,
            fragment_location: fragment.location.clone(),
        }
    }
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

impl sum_tree::KeyedItem for InsertionFragment {
    type Key = InsertionFragmentSummary;

    fn key(&self) -> Self::Key {
        sum_tree::Item::summary(self)
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct InsertionFragmentSummary {
    max_insertion_id: OperationId,
    max_offset_in_insertion: usize,
}

impl sum_tree::Summary for InsertionFragmentSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        debug_assert!(summary.max_insertion_id >= self.max_insertion_id);
        debug_assert!(
            summary.max_insertion_id > self.max_insertion_id
                || summary.max_offset_in_insertion > self.max_offset_in_insertion
        );

        self.max_insertion_id = summary.max_insertion_id;
        self.max_offset_in_insertion = summary.max_offset_in_insertion;
    }
}

struct Document {
    branch: Branch,
    id: OperationId,
}

impl Document {
    pub fn edit<E, I, T>(&self, edits: E)
    where
        E: IntoIterator<IntoIter = I>,
        I: ExactSizeIterator<Item = (Range<usize>, T)>,
        T: Into<Arc<str>>,
    {
        self.branch.update(|operation_id, parent, revision| {
            let edits = edits.into_iter();
            let mut edit_op = operations::Edit {
                id: operation_id,
                parent: parent.clone(),
                edits: SmallVec::with_capacity(edits.len()),
            };
            let mut new_insertions = Vec::new();
            let mut insertion_offset = 0;

            let mut old_fragments = revision.document_fragments.cursor::<LocalEditDimension>();
            let mut new_fragments = old_fragments.slice(&self.id, Bias::Left, &());
            let document_visible_start = old_fragments.start().visible_len;
            let mut edits = edits
                .into_iter()
                .map(|(range, new_text)| {
                    (
                        (document_visible_start + range.start)
                            ..(document_visible_start + range.end),
                        new_text.into(),
                    )
                })
                .peekable();

            let mut new_ropes = RopeBuilder::new(
                revision.visible_text.cursor(0),
                revision.hidden_text.cursor(0),
            );
            new_fragments.append(
                old_fragments.slice(&(self.id, edits.peek().unwrap().0.start), Bias::Right, &()),
                &(),
            );

            new_ropes.append(
                new_fragments.summary().visible_len,
                new_fragments.summary().hidden_len,
            );

            let mut fragment_start = old_fragments.start().visible_len;
            for (range, new_text) in edits {
                let fragment_end = old_fragments.end(&()).visible_len;

                // If the current fragment ends before this range, then jump ahead to the first fragment
                // that extends past the start of this range, reusing any intervening fragments.
                if fragment_end < range.start {
                    // If the current fragment has been partially consumed, then consume the rest of it
                    // and advance to the next fragment before slicing.
                    if fragment_start > old_fragments.start().visible_len {
                        if fragment_end > fragment_start {
                            let mut suffix = old_fragments.item().unwrap().clone();
                            suffix.insertion_subrange.start +=
                                fragment_start - old_fragments.start().visible_len;
                            new_insertions
                                .push(sum_tree::Edit::Insert(InsertionFragment::new(&suffix)));
                            new_ropes.push_fragment(&suffix, suffix.visible());
                            new_fragments.push(suffix, &());
                        }
                        old_fragments.next(&());
                    }

                    let slice = old_fragments.slice(&(self.id, range.start), Bias::Right, &());
                    new_ropes.append(slice.summary().visible_len, slice.summary().hidden_len);
                    new_fragments.append(slice, &());
                    fragment_start = old_fragments.start().visible_len;
                }

                let start_fragment = old_fragments.item().and_then(|item| {
                    if item.document_id == self.id {
                        Some(item)
                    } else {
                        None
                    }
                });
                let edit_start = {
                    let start_fragment = start_fragment
                        .or_else(|| old_fragments.prev_item())
                        .unwrap();
                    Anchor {
                        insertion_id: start_fragment.insertion_id,
                        offset_in_insertion: start_fragment.insertion_subrange.start
                            + (range.start - old_fragments.start().visible_len),
                        bias: Bias::Right,
                    }
                };

                // Preserve any portion of the current fragment that precedes this range.
                if fragment_start < range.start {
                    let mut prefix = old_fragments.item().unwrap().clone();
                    let prefix_len = range.start - fragment_start;
                    prefix.insertion_subrange.start +=
                        fragment_start - old_fragments.start().visible_len;
                    prefix.insertion_subrange.end = prefix.insertion_subrange.start + prefix_len;
                    prefix.location =
                        DenseId::between(&new_fragments.summary().max_location, &prefix.location);
                    new_insertions.push(sum_tree::Edit::Insert(InsertionFragment::new(&prefix)));
                    new_ropes.push_fragment(&prefix, prefix.visible());
                    new_fragments.push(prefix, &());
                    fragment_start = range.start;
                }

                // Insert the new text before any existing fragments within the range.
                if !new_text.is_empty() {
                    let fragment = DocumentFragment {
                        document_id: self.id,
                        location: DenseId::between(
                            &new_fragments.summary().max_location,
                            start_fragment
                                .map_or(&DenseId::max(), |old_fragment| &old_fragment.location),
                        ),
                        insertion_id: edit_op.id,
                        insertion_subrange: insertion_offset..insertion_offset + new_text.len(),
                        tombstones: Default::default(),
                        undo_count: 0,
                    };
                    new_insertions.push(sum_tree::Edit::Insert(InsertionFragment::new(&fragment)));
                    new_ropes.push_str(new_text.as_ref());
                    new_fragments.push(fragment, &());
                    insertion_offset += new_text.len();
                }

                // Advance through every fragment that intersects this range, marking the intersecting
                // portions as deleted.
                while fragment_start < range.end {
                    let fragment = old_fragments.item().unwrap();
                    let fragment_end = old_fragments.end(&()).visible_len;
                    let mut intersection = fragment.clone();
                    let intersection_end = cmp::min(range.end, fragment_end);
                    if fragment.visible() {
                        let intersection_len = intersection_end - fragment_start;
                        intersection.insertion_subrange.start +=
                            fragment_start - old_fragments.start().visible_len;
                        intersection.insertion_subrange.end =
                            intersection.insertion_subrange.start + intersection_len;
                        intersection.location = DenseId::between(
                            &new_fragments.summary().max_location,
                            &intersection.location,
                        );
                        intersection.tombstones.push(Tombstone {
                            id: edit_op.id,
                            undo_count: 0,
                        });
                    }
                    if intersection.len() > 0 {
                        new_insertions.push(sum_tree::Edit::Insert(InsertionFragment::new(
                            &intersection,
                        )));
                        new_ropes.push_fragment(&intersection, fragment.visible());
                        new_fragments.push(intersection, &());
                        fragment_start = intersection_end;
                    }
                    if fragment_end <= range.end {
                        old_fragments.next(&());
                    }
                }

                let end_fragment = old_fragments
                    .item()
                    .and_then(|item| {
                        if item.document_id == self.id {
                            Some(item)
                        } else {
                            None
                        }
                    })
                    .or_else(|| old_fragments.prev_item())
                    .unwrap();
                let edit_end = Anchor {
                    insertion_id: end_fragment.insertion_id,
                    offset_in_insertion: end_fragment.insertion_subrange.start
                        + (range.end - old_fragments.start().visible_len),
                    bias: Bias::Left,
                };
                edit_op.edits.push((
                    AnchorRange {
                        document_id: self.id,
                        revision_id: parent.clone(),
                        start_insertion_id: edit_start.insertion_id,
                        start_offset_in_insertion: edit_start.offset_in_insertion,
                        start_bias: edit_start.bias,
                        end_insertion_id: edit_end.insertion_id,
                        end_offset_in_insertion: edit_end.offset_in_insertion,
                        end_bias: edit_end.bias,
                    },
                    new_text.clone(),
                ));
            }

            // If the current fragment has been partially consumed, then consume the rest of it
            // and advance to the next fragment before slicing.
            if fragment_start > old_fragments.start().visible_len {
                let fragment_end = old_fragments.end(&()).visible_len;
                if fragment_end > fragment_start {
                    let mut suffix = old_fragments.item().unwrap().clone();
                    let suffix_len = fragment_end - fragment_start;
                    suffix.insertion_subrange.start +=
                        fragment_start - old_fragments.start().visible_len;
                    suffix.insertion_subrange.end = suffix.insertion_subrange.start + suffix_len;
                    new_insertions.push(sum_tree::Edit::Insert(InsertionFragment::new(&suffix)));
                    new_ropes.push_fragment(&suffix, suffix.visible());
                    new_fragments.push(suffix, &());
                }
                old_fragments.next(&());
            }

            let suffix = old_fragments.suffix(&());
            new_ropes.append(suffix.summary().visible_len, suffix.summary().hidden_len);
            new_fragments.append(suffix, &());
            let (visible_text, hidden_text) = new_ropes.finish();
            drop(old_fragments);

            revision.document_fragments = new_fragments;
            revision.insertion_fragments.edit(new_insertions, &());
            revision.visible_text = visible_text;
            revision.hidden_text = hidden_text;

            (Operation::Edit(edit_op), ())
        })
    }

    fn text(&self) -> Rope {
        self.branch.read(|revision| {
            let mut fragments = revision.document_fragments.cursor::<LocalEditDimension>();
            fragments.seek(&self.id, Bias::Left, &());
            let start = fragments.start().visible_len;

            let mut next_doc_id = self.id;
            next_doc_id.operation_count.0 += 1;
            fragments.seek(&next_doc_id, Bias::Left, &());
            let end = fragments.start().visible_len;

            revision.visible_text.slice(start..end)
        })
    }
}

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

impl<'a> sum_tree::SeekTarget<'a, DocumentFragmentSummary, LocalEditDimension> for OperationId {
    fn cmp(&self, cursor_location: &LocalEditDimension, _: &()) -> Ordering {
        Ord::cmp(self, &cursor_location.max_document_id)
    }
}

impl<'a> sum_tree::SeekTarget<'a, DocumentFragmentSummary, LocalEditDimension> for usize {
    fn cmp(&self, cursor_location: &LocalEditDimension, _: &()) -> Ordering {
        Ord::cmp(self, &cursor_location.visible_len)
    }
}

impl<'a> sum_tree::SeekTarget<'a, DocumentFragmentSummary, LocalEditDimension>
    for (OperationId, usize)
{
    fn cmp(&self, cursor_location: &LocalEditDimension, _: &()) -> Ordering {
        Ord::cmp(
            self,
            &(cursor_location.max_document_id, cursor_location.visible_len),
        )
    }
}

#[derive(Copy, Clone, Debug)]
struct Anchor {
    insertion_id: OperationId,
    offset_in_insertion: usize,
    bias: Bias,
}

#[derive(Clone, Debug)]
pub struct AnchorRange {
    document_id: OperationId,
    revision_id: RevisionId,
    start_insertion_id: OperationId,
    start_offset_in_insertion: usize,
    start_bias: Bias,
    end_insertion_id: OperationId,
    end_offset_in_insertion: usize,
    end_bias: Bias,
}

struct RopeBuilder<'a> {
    old_visible_cursor: rope::Cursor<'a>,
    old_hidden_cursor: rope::Cursor<'a>,
    new_visible: Rope,
    new_hidden: Rope,
}

impl<'a> RopeBuilder<'a> {
    fn new(old_visible_cursor: rope::Cursor<'a>, old_hidden_cursor: rope::Cursor<'a>) -> Self {
        Self {
            old_visible_cursor,
            old_hidden_cursor,
            new_visible: Rope::new(),
            new_hidden: Rope::new(),
        }
    }

    fn append(&mut self, visible_len: usize, hidden_len: usize) {
        self.push(visible_len, true, true);
        self.push(hidden_len, false, false);
    }

    fn push_fragment(&mut self, fragment: &DocumentFragment, was_visible: bool) {
        self.push(fragment.len(), was_visible, fragment.visible())
    }

    fn push(&mut self, len: usize, was_visible: bool, is_visible: bool) {
        let text = if was_visible {
            self.old_visible_cursor
                .slice(self.old_visible_cursor.offset() + len as usize)
        } else {
            self.old_hidden_cursor
                .slice(self.old_hidden_cursor.offset() + len)
        };
        if is_visible {
            self.new_visible.append(text);
        } else {
            self.new_hidden.append(text);
        }
    }

    fn push_str(&mut self, text: &str) {
        self.new_visible.push(text);
    }

    fn finish(mut self) -> (Rope, Rope) {
        self.new_visible.append(self.old_visible_cursor.suffix());
        self.new_hidden.append(self.old_hidden_cursor.suffix());
        (self.new_visible, self.new_hidden)
    }
}

#[derive(Clone, Debug, Default)]
struct RepoSnapshot {
    last_operation_id: OperationId,
    branches: TreeMap<OperationId, BranchSnapshot>,
    operations: TreeMap<OperationId, Operation>,
    revisions: TreeMap<RevisionId, Revision>,
    name: Option<Arc<str>>,
}

impl RepoSnapshot {
    fn new(replica_id: ReplicaId) -> Self {
        Self {
            last_operation_id: OperationId::new(replica_id),
            branches: Default::default(),
            operations: Default::default(),
            revisions: Default::default(),
            name: None,
        }
    }

    fn create_empty_branch(&mut self, name: impl Into<Arc<str>>) -> (Operation, OperationId) {
        let name = name.into();
        let branch_id = self.last_operation_id.tick();
        self.branches.insert(
            branch_id,
            BranchSnapshot {
                name: name.clone(),
                head: smallvec![branch_id],
            },
        );
        self.revisions
            .insert(smallvec![branch_id], Default::default());
        let operation = Operation::CreateBranch(CreateBranch {
            id: branch_id,
            name,
            parent: Default::default(),
        });
        (operation, branch_id)
    }
}

#[derive(Clone, Debug)]
struct BranchSnapshot {
    name: Arc<str>,
    head: RevisionId,
}

#[derive(Default, Debug, Clone)]
struct Revision {
    document_metadata: TreeMap<OperationId, DocumentMetadata>,
    document_fragments: SumTree<DocumentFragment>,
    insertion_fragments: SumTree<InsertionFragment>,
    visible_text: Rope,
    hidden_text: Rope,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::TestNetwork;

    #[gpui::test]
    async fn test_repo() {
        let network = TestNetwork::default();
        let server = Server::new(network.server());

        let client_a = Client::new(network.client());
        let repo_a = client_a.create_repo();
        let branch_a = repo_a.create_empty_branch("main");

        let doc1 = branch_a.create_document();
        doc1.edit([(0..0, "abc")]);

        let doc2 = branch_a.create_document();
        doc2.edit([(0..0, "def")]);

        assert_eq!(doc1.text().to_string(), "abc");
        assert_eq!(doc2.text().to_string(), "def");

        client_a.publish_repo(&repo_a, "repo-1").await.unwrap();
        let db_b = Client::new(network.client());
        let repo_b = db_b.clone_repo("repo-1");
    }
}
