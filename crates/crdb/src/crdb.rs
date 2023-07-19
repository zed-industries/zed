mod dense_id;
mod messages;
mod operations;
mod sync;
#[cfg(test)]
mod test;

use anyhow::{anyhow, Result};
use collections::{btree_map, BTreeMap, Bound, HashMap};
use dense_id::DenseId;
use futures::{channel::mpsc, future::BoxFuture, FutureExt, StreamExt};
use messages::{MessageEnvelope, Operation, RequestEnvelope};
use operations::CreateBranch;
use parking_lot::{Mutex, RwLock};
use rope::Rope;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::{
    any::{Any, TypeId},
    cmp::{self, Ordering},
    fmt::{self, Debug, Display},
    future::Future,
    ops::Range,
    path::Path,
    sync::Arc,
};
use sum_tree::{Bias, SumTree, TreeMap};
use util::ResultExt;
use uuid::Uuid;

const CHUNK_SIZE: usize = 64;

#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct RepoId(Uuid);

impl RepoId {
    fn new() -> Self {
        RepoId(Uuid::new_v4())
    }
}

impl Display for RepoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_hyphenated())
    }
}

type RevisionId = SmallVec<[OperationId; 2]>;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ReplicaId(u32);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OperationCount(usize);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OperationId {
    pub replica_id: ReplicaId,
    pub operation_count: OperationCount,
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

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomName(Arc<str>);

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomToken(Arc<str>);

#[derive(Clone)]
pub struct User {
    login: Arc<str>,
}

pub trait Request: Message + Into<RequestEnvelope> {
    type Response: Message;
}

pub trait Message: 'static + Send {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self>
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
}

impl<T> Message for T
where
    T: 'static + Send + Serialize + for<'a> Deserialize<'a>,
{
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(serde_bare::from_slice(&bytes)?)
    }

    fn to_bytes(&self) -> Vec<u8> {
        serde_bare::to_vec(self).unwrap()
    }
}

pub trait ServerNetwork: 'static + Send + Sync {
    fn handle_requests<H, F>(&self, handle_request: H)
    where
        H: 'static + Send + Fn(User, Vec<u8>) -> Result<F>,
        F: 'static + Send + futures::Future<Output = Result<Vec<u8>>>;
    fn create_room(&self, room: &RoomName) -> BoxFuture<Result<()>>;
    fn grant_room_access(&self, room: &RoomName, user: &str) -> RoomToken;
}

pub trait ClientNetwork: 'static + Send + Sync {
    type Room: ClientRoom;

    fn request(&self, request: Vec<u8>) -> BoxFuture<Result<Vec<u8>>>;
    fn room(&self, credentials: RoomCredentials) -> Self::Room;
}

pub trait ClientRoom: 'static + Send + Sync {
    fn connect(&mut self) -> BoxFuture<Result<()>>;
    fn broadcast(&self, message: Vec<u8>);
    fn handle_messages(&self, handle_message: impl 'static + Send + Fn(Vec<u8>));
}

pub trait Executor: 'static + Send + Sync {
    fn spawn<F>(&self, future: F)
    where
        F: 'static + Send + Future<Output = ()>;
}

struct Client<E, N: ClientNetwork> {
    db: Db,
    network: Arc<N>,
    checkouts: Arc<Mutex<HashMap<RepoId, Checkout<E, N>>>>,
    executor: Arc<E>,
}

struct Checkout<E, N: ClientNetwork> {
    repo: Repo,
    network_room: Arc<N::Room>,
    operations_tx: mpsc::UnboundedSender<Operation>,
    message_handlers:
        Arc<RwLock<HashMap<TypeId, Box<dyn Send + Sync + Fn(Client<E, N>, RepoId, Box<dyn Any>)>>>>,
}

impl<E, N: ClientNetwork> Clone for Checkout<E, N> {
    fn clone(&self) -> Self {
        Self {
            repo: self.repo.clone(),
            network_room: self.network_room.clone(),
            operations_tx: self.operations_tx.clone(),
            message_handlers: self.message_handlers.clone(),
        }
    }
}

impl<E: Executor, N: ClientNetwork> Checkout<E, N> {
    fn new(client: Client<E, N>, repo: Repo, network_room: N::Room) -> Self {
        let (operations_tx, operations_rx) = mpsc::unbounded();
        let this = Self {
            repo: repo.clone(),
            network_room: Arc::new(network_room),
            operations_tx,
            message_handlers: Default::default(),
        };

        {
            let handlers = this.message_handlers.clone();
            let client = client.clone();
            this.network_room.handle_messages(move |message| {
                if let Some(envelope) =
                    serde_bare::from_slice::<MessageEnvelope>(&message).log_err()
                {
                    let message = envelope.unwrap();
                    if let Some(handler) = handlers.read().get(&message.as_ref().type_id()) {
                        handler(client.clone(), repo.id, message);
                    }
                };
            });
        }

        client.executor.spawn({
            let this = this.clone();
            let client = client.clone();
            async move {
                this.sync(&client).await.expect("network is infallible");
                let mut operations_rx = operations_rx.ready_chunks(CHUNK_SIZE);
                while let Some(operations) = operations_rx.next().await {
                    client
                        .request(messages::PublishOperations {
                            repo_id: this.repo.id,
                            operations,
                        })
                        .await
                        .expect("network is infallible");
                }
            }
        });

        this
    }

    fn handle_messages<M: Message, H>(&self, handle_message: H)
    where
        M: Message,
        H: 'static + Fn(Client<E, N>, RepoId, M) + Send + Sync,
    {
        self.message_handlers.write().insert(
            TypeId::of::<M>(),
            Box::new(move |client, repo_id, message| {
                handle_message(client, repo_id, *message.downcast().unwrap())
            }),
        );
    }

    fn broadcast<M: Message>(&self, message: &M) {
        self.network_room.broadcast(message.to_bytes());
    }

    fn broadcast_operation(&self, operation: Operation) {
        self.broadcast(&operation);
        self.operations_tx.unbounded_send(operation).unwrap();
    }

    async fn sync(&self, client: &Client<E, N>) -> Result<()> {
        let response = client
            .request(messages::SyncRepo {
                id: self.repo.id,
                max_operation_ids: self.repo.read(|repo| (&repo.max_operation_ids).into()),
            })
            .await?;

        let operations = self
            .repo
            .read(|snapshot| snapshot.operations_since(&(&response.max_operation_ids).into()));

        for chunk in operations.chunks(CHUNK_SIZE) {
            client
                .request(messages::PublishOperations {
                    repo_id: self.repo.id,
                    operations: chunk.to_vec(),
                })
                .await?;
        }

        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomCredentials {
    name: RoomName,
    token: RoomToken,
}

impl<E, N: ClientNetwork> Clone for Client<E, N> {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            network: self.network.clone(),
            checkouts: self.checkouts.clone(),
            executor: self.executor.clone(),
        }
    }
}

impl<E: Executor, N: ClientNetwork> Client<E, N> {
    pub fn new(executor: E, network: N) -> Self {
        let mut this = Self {
            db: Db::new(),
            network: Arc::new(network),
            checkouts: Default::default(),
            executor: Arc::new(executor),
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
        let this = self.clone();
        let name = name.into();
        async move {
            let response = this.request(messages::CloneRepo { name }).await?;
            let repo_id = response.repo_id;
            let repo = Repo {
                id: repo_id,
                db: this.db.clone(),
            };
            this.db
                .snapshot
                .lock()
                .repos
                .insert(repo_id, Default::default());

            let checkout = Checkout::new(
                this.clone(),
                repo.clone(),
                this.network.room(response.credentials),
            );
            checkout.handle_messages(Self::handle_remote_operation);
            this.checkouts.lock().insert(repo_id, checkout);

            Ok(repo)
        }
    }

    pub fn publish_repo(
        &self,
        repo: &Repo,
        name: impl Into<Arc<str>>,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        let name = name.into();
        let repo = repo.clone();
        async move {
            let response = this
                .request(messages::PublishRepo { id: repo.id, name })
                .await?;
            let checkout = Checkout::new(
                this.clone(),
                repo.clone(),
                this.network.room(response.credentials),
            );
            checkout.handle_messages(Self::handle_remote_operation);
            this.checkouts.lock().insert(repo.id, checkout);

            Ok(())
        }
    }

    fn handle_local_operation(&self, repo_id: RepoId, operation: Operation) {
        if let Some(checkout) = self.checkouts.lock().get(&repo_id) {
            checkout.broadcast_operation(operation);
        }
    }

    fn handle_remote_operation(self, repo_id: RepoId, operation: Operation) {
        let repo = self.db.repo(repo_id).expect("repo must exist");
        repo.apply_operations([operation]);
    }

    fn request<R: Request>(&self, request: R) -> BoxFuture<Result<R::Response>> {
        let envelope: RequestEnvelope = request.into();
        let response = self.network.request(envelope.to_bytes());
        async { Ok(R::Response::from_bytes(response.await?)?) }.boxed()
    }
}

struct Server<N> {
    db: Db,
    network: Arc<N>,
    request_handlers: Arc<
        RwLock<
            BTreeMap<
                TypeId,
                Box<
                    dyn Send + Sync + Fn(User, Box<dyn Any>) -> BoxFuture<'static, Result<Vec<u8>>>,
                >,
            >,
        >,
    >,
    repo_ids_by_name: Arc<Mutex<BTreeMap<Arc<str>, RepoId>>>,
    next_replica_ids_by_repo_id: Arc<Mutex<BTreeMap<RepoId, ReplicaId>>>,
}

impl<N: ServerNetwork> Clone for Server<N> {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            network: self.network.clone(),
            repo_ids_by_name: self.repo_ids_by_name.clone(),
            request_handlers: self.request_handlers.clone(),
            next_replica_ids_by_repo_id: self.next_replica_ids_by_repo_id.clone(),
        }
    }
}

impl<N: ServerNetwork> Server<N> {
    fn new(network: N) -> Self {
        let network = Arc::new(network);
        let this = Self {
            db: Db::new(),
            network: network.clone(),
            request_handlers: Default::default(),
            repo_ids_by_name: Default::default(),
            next_replica_ids_by_repo_id: Default::default(),
        };

        this.handle_requests(Self::handle_publish_repo);
        this.handle_requests(Self::handle_clone_repo);
        this.handle_requests(Self::handle_sync_repo);
        this.handle_requests(Self::handle_publish_operations);
        let request_handlers = this.request_handlers.clone();

        network.handle_requests(move |user, request_bytes| {
            let envelope = RequestEnvelope::from_bytes(request_bytes)?;
            let request = envelope.unwrap();
            let request_handlers = request_handlers.read();
            let request_handler = request_handlers
                .get(&request.as_ref().type_id())
                .ok_or_else(|| anyhow!("no request handler"))?;
            let response = (request_handler)(user, request);
            Ok(response)
        });

        this
    }

    fn handle_requests<F, Fut, R>(&self, handle_request: F)
    where
        F: 'static + Send + Sync + Fn(Self, User, R) -> Fut,
        Fut: 'static + Send + Future<Output = Result<R::Response>>,
        R: Request,
    {
        let request_handlers = self.request_handlers.clone();

        request_handlers.write().insert(
            TypeId::of::<R>(),
            Box::new({
                let this = self.clone();
                move |user, request| {
                    let request = *request.downcast::<R>().unwrap();
                    let response = handle_request(this.clone(), user, request);
                    async move {
                        let response = response.await;
                        response.map(|response| response.to_bytes())
                    }
                    .boxed()
                }
            }),
        );
    }

    async fn handle_publish_repo(
        self,
        user: User,
        request: messages::PublishRepo,
    ) -> Result<messages::PublishRepoResponse> {
        // TODO: handle repositories that had already been published.
        match self.repo_ids_by_name.lock().entry(request.name.clone()) {
            btree_map::Entry::Occupied(_) => return Err(anyhow!("repo name taken")),
            btree_map::Entry::Vacant(entry) => {
                let mut db = self.db.snapshot.lock();
                db.repos.insert(request.id, Default::default());
                entry.insert(request.id);
            }
        }
        self.next_replica_ids_by_repo_id
            .lock()
            .insert(request.id, ReplicaId(1));

        let name = RoomName(request.id.to_string().into());
        self.network.create_room(&name).await?;
        let token = self.network.grant_room_access(&name, user.login.as_ref());

        Ok(messages::PublishRepoResponse {
            credentials: RoomCredentials { name, token },
        })
    }

    async fn handle_clone_repo(
        self,
        user: User,
        request: messages::CloneRepo,
    ) -> Result<messages::CloneRepoResponse> {
        let repo_id = *self
            .repo_ids_by_name
            .lock()
            .get(&request.name)
            .ok_or_else(|| anyhow!("repo not found"))?;
        let name = RoomName(repo_id.to_string().into());
        let token = self.network.grant_room_access(&name, user.login.as_ref());
        let replica_id = {
            let mut next_replica_ids = self.next_replica_ids_by_repo_id.lock();
            let next_replica_id = next_replica_ids.get_mut(&repo_id).unwrap();
            let replica_id = *next_replica_id;
            next_replica_id.0 += 1;
            replica_id
        };
        Ok(messages::CloneRepoResponse {
            repo_id,
            replica_id,
            credentials: RoomCredentials { name, token },
        })
    }

    async fn handle_sync_repo(
        self,
        _user: User,
        request: messages::SyncRepo,
    ) -> Result<messages::SyncRepoResponse> {
        let repo = self
            .db
            .repo(request.id)
            .ok_or_else(|| anyhow!("repo not found"))?;

        repo.read(|snapshot| {
            Ok(messages::SyncRepoResponse {
                operations: snapshot.operations_since(&(&request.max_operation_ids).into()),
                max_operation_ids: (&snapshot.max_operation_ids).into(),
            })
        })
    }

    async fn handle_publish_operations(
        self,
        _user: User,
        request: messages::PublishOperations,
    ) -> Result<()> {
        let repo = self
            .db
            .repo(request.repo_id)
            .ok_or_else(|| anyhow!("repo not found"))?;
        repo.apply_operations(request.operations);
        Ok(())
    }
}

#[derive(Clone)]
pub struct Db {
    snapshot: Arc<Mutex<DbSnapshot>>,
    local_operation_created: Option<Arc<dyn Send + Sync + Fn(RepoId, Operation)>>,
}

impl Db {
    fn new() -> Self {
        Self {
            snapshot: Default::default(),
            local_operation_created: None,
        }
    }

    fn on_local_operation(
        &mut self,
        operation_created: impl 'static + Send + Sync + Fn(RepoId, Operation),
    ) {
        self.local_operation_created = Some(Arc::new(operation_created));
    }

    fn repo(&self, id: RepoId) -> Option<Repo> {
        self.snapshot
            .lock()
            .repos
            .contains_key(&id)
            .then_some(Repo {
                id,
                db: self.clone(),
            })
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
                let replica_id = operation.id().replica_id;
                let count = operation.id().operation_count;
                if repo.max_operation_ids.get(&replica_id).copied() < Some(count) {
                    repo.max_operation_ids.insert(replica_id, count);
                }

                if let Some(local_operation_created) = self.db.local_operation_created.as_ref() {
                    local_operation_created(self.id, operation);
                }

                result
            })
            .expect("repo must exist")
    }

    fn apply_operations(&self, operations: impl IntoIterator<Item = Operation>) {
        self.db
            .snapshot
            .lock()
            .repos
            .update(&self.id, |repo| todo!());
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnchorRange {
    document_id: OperationId,
    revision_id: RevisionId,
    start_insertion_id: OperationId,
    start_offset_in_insertion: usize,
    #[serde(with = "bias_serialization")]
    start_bias: Bias,
    end_insertion_id: OperationId,
    end_offset_in_insertion: usize,
    #[serde(with = "bias_serialization")]
    end_bias: Bias,
}

mod bias_serialization {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use sum_tree::Bias;

    pub fn serialize<S>(field: &Bias, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match field {
            Bias::Left => "left".serialize(serializer),
            Bias::Right => "right".serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Bias, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "left" => Ok(Bias::Left),
            "right" => Ok(Bias::Right),
            _ => Err(serde::de::Error::custom("invalid bias")),
        }
    }
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
    max_operation_ids: TreeMap<ReplicaId, OperationCount>,
}

impl RepoSnapshot {
    fn new(replica_id: ReplicaId) -> Self {
        Self {
            last_operation_id: OperationId::new(replica_id),
            ..Default::default()
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

    fn operations_since(&self, version: &TreeMap<ReplicaId, OperationCount>) -> Vec<Operation> {
        let mut new_operations = Vec::new();
        for (replica_id, end_op_count) in self.max_operation_ids.iter() {
            let end_op = OperationId {
                replica_id: *replica_id,
                operation_count: *end_op_count,
            };
            if let Some(start_op_count) = version.get(&replica_id) {
                let start_op = OperationId {
                    replica_id: *replica_id,
                    operation_count: *start_op_count,
                };
                new_operations.extend(
                    self.operations
                        .range((Bound::Excluded(&start_op), Bound::Included(&end_op)))
                        .map(|(_, op)| op.clone()),
                );
            } else {
                let start_op = OperationId::new(*replica_id);
                new_operations.extend(
                    self.operations
                        .range((Bound::Included(&start_op), Bound::Included(&end_op)))
                        .map(|(_, op)| op.clone()),
                );
            }
        }
        new_operations
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
    use gpui::executor::{Background, Deterministic};

    use super::*;
    use crate::test::TestNetwork;

    #[gpui::test]
    async fn test_repo(deterministic: Arc<Deterministic>) {
        let network = TestNetwork::new(deterministic.build_background());
        let server = Server::new(network.server());

        let client_a = Client::new(deterministic.build_background(), network.client("client-a"));
        let repo_a = client_a.create_repo();
        let branch_a = repo_a.create_empty_branch("main");

        let doc1 = branch_a.create_document();
        doc1.edit([(0..0, "abc")]);

        let doc2 = branch_a.create_document();
        doc2.edit([(0..0, "def")]);

        assert_eq!(doc1.text().to_string(), "abc");
        assert_eq!(doc2.text().to_string(), "def");

        client_a.publish_repo(&repo_a, "repo-1").await.unwrap();
        let db_b = Client::new(deterministic.build_background(), network.client("client-b"));
        let repo_b = db_b.clone_repo("repo-1").await.unwrap();
    }

    impl Executor for Arc<Background> {
        fn spawn<F>(&self, future: F)
        where
            F: 'static + Send + Future<Output = ()>,
        {
            Background::spawn(self, future).detach();
        }
    }
}
