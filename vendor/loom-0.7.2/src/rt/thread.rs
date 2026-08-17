use crate::rt::execution;
use crate::rt::object::Operation;
use crate::rt::vv::VersionVec;

use std::{any::Any, collections::HashMap, fmt, ops};

use super::Location;
pub(crate) struct Thread {
    pub id: Id,

    /// If the thread is runnable, blocked, or terminated.
    pub state: State,

    /// True if the thread is in a critical section
    pub critical: bool,

    /// The operation the thread is about to take
    pub(super) operation: Option<Operation>,

    /// Tracks observed causality
    pub causality: VersionVec,

    /// Tracks the view of the lastest release fence
    pub released: VersionVec,

    /// Tracks DPOR relations
    pub dpor_vv: VersionVec,

    /// Version at which the thread last yielded
    pub last_yield: Option<u16>,

    /// Number of times the thread yielded
    pub yield_count: usize,

    locals: LocalMap,

    /// `tracing` span used to associate diagnostics with the current thread.
    span: tracing::Span,
}

#[derive(Debug)]
pub(crate) struct Set {
    /// Unique execution identifier
    execution_id: execution::Id,

    /// Set of threads
    threads: Vec<Thread>,

    /// Currently scheduled thread.
    ///
    /// `None` signifies that no thread is runnable.
    active: Option<usize>,

    /// Sequential consistency causality. All sequentially consistent operations
    /// synchronize with this causality.
    pub seq_cst_causality: VersionVec,

    /// `tracing` span used as the parent for new thread spans.
    iteration_span: tracing::Span,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub(crate) struct Id {
    execution_id: execution::Id,
    id: usize,
}

impl Id {
    /// Returns an integer ID unique to this current execution (for use in
    /// [`thread::ThreadId`]'s `Debug` impl)
    pub(crate) fn public_id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum State {
    Runnable { unparked: bool },
    Blocked(#[allow(dead_code)] Location),
    Yield,
    Terminated,
}

type LocalMap = HashMap<LocalKeyId, LocalValue>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct LocalKeyId(usize);

struct LocalValue(Option<Box<dyn Any>>);

impl Thread {
    fn new(id: Id, parent_span: &tracing::Span) -> Thread {
        Thread {
            id,
            span: tracing::info_span!(parent: parent_span.id(), "thread", id = id.id),
            state: State::Runnable { unparked: false },
            critical: false,
            operation: None,
            causality: VersionVec::new(),
            released: VersionVec::new(),
            dpor_vv: VersionVec::new(),
            last_yield: None,
            yield_count: 0,
            locals: HashMap::new(),
        }
    }

    pub(crate) fn is_runnable(&self) -> bool {
        matches!(self.state, State::Runnable { .. })
    }

    pub(crate) fn set_runnable(&mut self) {
        self.state = State::Runnable { unparked: false };
    }

    pub(crate) fn set_blocked(&mut self, location: Location) {
        self.state = State::Blocked(location);
    }

    pub(crate) fn is_blocked(&self) -> bool {
        matches!(self.state, State::Blocked(..))
    }

    pub(crate) fn is_yield(&self) -> bool {
        matches!(self.state, State::Yield)
    }

    pub(crate) fn set_yield(&mut self) {
        self.state = State::Yield;
        self.last_yield = Some(self.causality[self.id]);
        self.yield_count += 1;
    }

    pub(crate) fn is_terminated(&self) -> bool {
        matches!(self.state, State::Terminated)
    }

    pub(crate) fn set_terminated(&mut self) {
        self.state = State::Terminated;
    }

    pub(crate) fn drop_locals(&mut self) -> Box<dyn std::any::Any> {
        let mut locals = Vec::with_capacity(self.locals.len());

        // run the Drop impls of any mock thread-locals created by this thread.
        for local in self.locals.values_mut() {
            locals.push(local.0.take());
        }

        Box::new(locals)
    }

    pub(crate) fn unpark(&mut self, unparker: &Thread) {
        self.causality.join(&unparker.causality);
        self.set_unparked();
    }

    /// Unpark a thread's state. If it is already runnable, store the unpark for
    /// a future call to `park`.
    fn set_unparked(&mut self) {
        if self.is_blocked() || self.is_yield() {
            self.set_runnable();
        } else if self.is_runnable() {
            self.state = State::Runnable { unparked: true }
        }
    }
}

impl fmt::Debug for Thread {
    // Manual debug impl is necessary because thread locals are represented as
    // `dyn Any`, which does not implement `Debug`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Thread")
            .field("id", &self.id)
            .field("state", &self.state)
            .field("critical", &self.critical)
            .field("operation", &self.operation)
            .field("causality", &self.causality)
            .field("released", &self.released)
            .field("dpor_vv", &self.dpor_vv)
            .field("last_yield", &self.last_yield)
            .field("yield_count", &self.yield_count)
            .field("locals", &format_args!("[..locals..]"))
            .finish()
    }
}

impl Set {
    /// Create an empty thread set.
    ///
    /// The set may contain up to `max_threads` threads.
    pub(crate) fn new(execution_id: execution::Id, max_threads: usize) -> Set {
        let mut threads = Vec::with_capacity(max_threads);
        // Capture the current iteration's span to be used as each thread
        // span's parent.
        let iteration_span = tracing::Span::current();
        // Push initial thread
        threads.push(Thread::new(Id::new(execution_id, 0), &iteration_span));

        Set {
            execution_id,
            threads,
            active: Some(0),
            seq_cst_causality: VersionVec::new(),
            iteration_span,
        }
    }

    pub(crate) fn execution_id(&self) -> execution::Id {
        self.execution_id
    }

    /// Create a new thread
    pub(crate) fn new_thread(&mut self) -> Id {
        assert!(self.threads.len() < self.max());

        // Get the identifier for the thread about to be created
        let id = self.threads.len();

        // Push the thread onto the stack
        self.threads.push(Thread::new(
            Id::new(self.execution_id, id),
            &self.iteration_span,
        ));

        Id::new(self.execution_id, id)
    }

    pub(crate) fn max(&self) -> usize {
        self.threads.capacity()
    }

    pub(crate) fn is_active(&self) -> bool {
        self.active.is_some()
    }

    pub(crate) fn is_complete(&self) -> bool {
        if self.active.is_none() {
            // All threads should be terminated
            for thread in &self.threads {
                assert!(
                    thread.is_terminated(),
                    "thread not terminated; {:#?}",
                    thread
                );
            }

            true
        } else {
            false
        }
    }

    pub(crate) fn active_id(&self) -> Id {
        Id::new(self.execution_id, self.active.unwrap())
    }

    pub(crate) fn active(&self) -> &Thread {
        &self.threads[self.active.unwrap()]
    }

    pub(crate) fn set_active(&mut self, id: Option<Id>) {
        tracing::dispatcher::get_default(|subscriber| {
            if let Some(span_id) = self.active().span.id() {
                subscriber.exit(&span_id)
            }

            if let Some(span_id) = id.and_then(|id| self.threads.get(id.id)?.span.id()) {
                subscriber.enter(&span_id);
            }
        });
        self.active = id.map(Id::as_usize);
    }

    pub(crate) fn active_mut(&mut self) -> &mut Thread {
        &mut self.threads[self.active.unwrap()]
    }

    /// Get the active thread and second thread
    pub(crate) fn active2_mut(&mut self, other: Id) -> (&mut Thread, &mut Thread) {
        let active = self.active.unwrap();
        let other = other.id;

        if other >= active {
            let (l, r) = self.threads.split_at_mut(other);

            (&mut l[active], &mut r[0])
        } else {
            let (l, r) = self.threads.split_at_mut(active);

            (&mut r[0], &mut l[other])
        }
    }

    pub(crate) fn active_causality_inc(&mut self) {
        let id = self.active_id();
        self.active_mut().causality.inc(id);
    }

    pub(crate) fn active_atomic_version(&self) -> u16 {
        let id = self.active_id();
        self.active().causality[id]
    }

    pub(crate) fn unpark(&mut self, id: Id) {
        if id == self.active_id() {
            // The thread is unparking itself. We don't have to join its
            // causality with the unparker's causality in this case, since the
            // thread *is* the unparker. Just unpark its state.
            self.active_mut().set_unparked();
            return;
        }

        // Synchronize memory
        let (active, th) = self.active2_mut(id);
        th.unpark(active);
    }

    /// Insert a point of sequential consistency
    /// TODO
    /// - Deprecate SeqCst accesses and allow SeqCst fences only. The semantics of SeqCst accesses
    ///   is complex and difficult to implement correctly. On the other hand, SeqCst fence has
    ///   well-understood and clear semantics in the absence of SeqCst accesses, and can be used
    ///   for enforcing the read-after-write (RAW) ordering which is probably what the user want to
    ///   achieve with SeqCst.
    /// - Revisit the other uses of this function. They probably don't require sequential
    ///   consistency. E.g. see https://en.cppreference.com/w/cpp/named_req/Mutex
    ///
    /// References
    /// - The "scfix" paper, which proposes a memory model called RC11 that fixes SeqCst
    ///   semantics. of C11. https://plv.mpi-sws.org/scfix/
    /// - Some fixes from the "scfix" paper has been incorporated into C/C++20:
    ///   http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p0668r5.html
    /// - The "promising semantics" paper, which propose an intuitive semantics of SeqCst fence in
    ///   the absence of SC accesses. https://sf.snu.ac.kr/promise-concurrency/
    pub(crate) fn seq_cst(&mut self) {
        // The previous implementation of sequential consistency was incorrect (though it's correct
        // for `fence(SeqCst)`-only scenario; use `seq_cst_fence` for `fence(SeqCst)`).
        // As a quick fix, just disable it. This may fail to model correct code,
        // but will not silently allow bugs.
    }

    pub(crate) fn seq_cst_fence(&mut self) {
        self.threads[self.active.unwrap()]
            .causality
            .join(&self.seq_cst_causality);
        self.seq_cst_causality
            .join(&self.threads[self.active.unwrap()].causality);
    }

    pub(crate) fn clear(&mut self, execution_id: execution::Id) {
        self.iteration_span = tracing::Span::current();
        self.threads.clear();
        self.threads
            .push(Thread::new(Id::new(execution_id, 0), &self.iteration_span));

        self.execution_id = execution_id;
        self.active = Some(0);
        self.seq_cst_causality = VersionVec::new();
    }

    pub(crate) fn iter(&self) -> impl ExactSizeIterator<Item = (Id, &Thread)> + '_ {
        let execution_id = self.execution_id;
        self.threads
            .iter()
            .enumerate()
            .map(move |(id, thread)| (Id::new(execution_id, id), thread))
    }

    pub(crate) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (Id, &mut Thread)> + '_ {
        let execution_id = self.execution_id;
        self.threads
            .iter_mut()
            .enumerate()
            .map(move |(id, thread)| (Id::new(execution_id, id), thread))
    }

    /// Split the set of threads into the active thread and an iterator of all
    /// other threads.
    pub(crate) fn split_active(&mut self) -> (&mut Thread, impl Iterator<Item = &mut Thread>) {
        let active = self.active.unwrap();
        let (one, two) = self.threads.split_at_mut(active);
        let (active, two) = two.split_at_mut(1);

        let iter = one.iter_mut().chain(two.iter_mut());

        (&mut active[0], iter)
    }

    pub(crate) fn local<T: 'static>(
        &mut self,
        key: &'static crate::thread::LocalKey<T>,
    ) -> Option<Result<&T, AccessError>> {
        self.active_mut()
            .locals
            .get(&LocalKeyId::new(key))
            .map(|local_value| local_value.get())
    }

    pub(crate) fn local_init<T: 'static>(
        &mut self,
        key: &'static crate::thread::LocalKey<T>,
        value: T,
    ) {
        assert!(self
            .active_mut()
            .locals
            .insert(LocalKeyId::new(key), LocalValue::new(value))
            .is_none())
    }
}

impl ops::Index<Id> for Set {
    type Output = Thread;

    fn index(&self, index: Id) -> &Thread {
        &self.threads[index.id]
    }
}

impl ops::IndexMut<Id> for Set {
    fn index_mut(&mut self, index: Id) -> &mut Thread {
        &mut self.threads[index.id]
    }
}

impl Id {
    pub(crate) fn new(execution_id: execution::Id, id: usize) -> Id {
        Id { execution_id, id }
    }

    pub(crate) fn as_usize(self) -> usize {
        self.id
    }
}

impl From<Id> for usize {
    fn from(src: Id) -> usize {
        src.id
    }
}

impl fmt::Display for Id {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(fmt)
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Id({})", self.id)
    }
}

impl LocalKeyId {
    fn new<T>(key: &'static crate::thread::LocalKey<T>) -> Self {
        Self(key as *const _ as usize)
    }
}

impl LocalValue {
    fn new<T: 'static>(value: T) -> Self {
        Self(Some(Box::new(value)))
    }

    fn get<T: 'static>(&self) -> Result<&T, AccessError> {
        self.0
            .as_ref()
            .ok_or(AccessError { _private: () })
            .map(|val| {
                val.downcast_ref::<T>()
                    .expect("local value must downcast to expected type")
            })
    }
}

/// An error returned by [`LocalKey::try_with`](struct.LocalKey.html#method.try_with).
pub struct AccessError {
    _private: (),
}

impl fmt::Debug for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccessError").finish()
    }
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt("already destroyed", f)
    }
}
