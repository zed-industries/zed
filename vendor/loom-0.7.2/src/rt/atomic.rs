//! An atomic cell
//!
//! See the CDSChecker paper for detailed explanation.
//!
//! # Modification order implications (figure 7)
//!
//! - Read-Read Coherence:
//!
//!   On `load`, all stores are iterated, finding stores that were read by
//!   actions in the current thread's causality. These loads happen-before the
//!   current load. The `modification_order` of these happen-before loads are
//!   joined into the current load's `modification_order`.
//!
//! - Write-Read Coherence:
//!
//!   On `load`, all stores are iterated, finding stores that happens-before the
//!   current thread's causality. The `modification_order` of these stores are
//!   joined into the current load's `modification_order`.
//!
//! - Read-Write Coherence:
//!
//!   On `store`, find all existing stores that were read in the current
//!   thread's causality. Join these stores' `modification_order` into the new
//!   store's modification order.
//!
//! - Write-Write Coherence:
//!
//!   The `modification_order` is initialized to the thread's causality. Any
//!   store that happened in the thread causality will be earlier in the
//!   modification order.
//!
//! - Seq-cst/MO Consistency:
//!
//! - Seq-cst Write-Read Coherence:
//!
//! - RMW/MO Consistency: Subsumed by Write-Write Coherence?
//!
//! - RMW Atomicity:
//!
//!
//! # Fence modification order implications (figure 9)
//!
//! - SC Fences Restrict RF:
//! - SC Fences Restrict RF (Collapsed Store):
//! - SC Fences Restrict RF (Collapsed Load):
//! - SC Fences Impose MO:
//! - SC Fences Impose MO (Collapsed 1st Store):
//! - SC Fences Impose MO (Collapsed 2st Store):
//!
//!
//! # Fence Synchronization implications (figure 10)
//!
//! - Fence Synchronization
//! - Fence Synchronization (Collapsed Store)
//! - Fence Synchronization (Collapsed Load)

use crate::rt::execution::Execution;
use crate::rt::location::{self, Location, LocationSet};
use crate::rt::object;
use crate::rt::{
    self, thread, Access, Numeric, Synchronize, VersionVec, MAX_ATOMIC_HISTORY, MAX_THREADS,
};

use std::cmp;
use std::marker::PhantomData;
use std::sync::atomic::Ordering;
use std::u16;

use tracing::trace;

#[derive(Debug)]
pub(crate) struct Atomic<T> {
    state: object::Ref<State>,
    _p: PhantomData<fn() -> T>,
}

#[derive(Debug)]
pub(super) struct State {
    /// Where the atomic was created
    created_location: Location,

    /// Transitive closure of all atomic loads from the cell.
    loaded_at: VersionVec,

    /// Location for the *last* time a thread atomically loaded from the cell.
    loaded_locations: LocationSet,

    /// Transitive closure of all **unsynchronized** loads from the cell.
    unsync_loaded_at: VersionVec,

    /// Location for the *last* time a thread read **synchronized** from the cell.
    unsync_loaded_locations: LocationSet,

    /// Transitive closure of all atomic stores to the cell.
    stored_at: VersionVec,

    /// Location for the *last* time a thread atomically stored to the cell.
    stored_locations: LocationSet,

    /// Version of the most recent **unsynchronized** mutable access to the
    /// cell.
    ///
    /// This includes the initialization of the cell as well as any calls to
    /// `get_mut`.
    unsync_mut_at: VersionVec,

    /// Location for the *last* time a thread `with_mut` from the cell.
    unsync_mut_locations: LocationSet,

    /// `true` when in a `with_mut` closure. If this is set, there can be no
    /// access to the cell.
    is_mutating: bool,

    /// Last time the atomic was accessed. This tracks the dependent access for
    /// the DPOR algorithm.
    last_access: Option<Access>,

    /// Last time the atomic was accessed for a store or rmw operation.
    last_non_load_access: Option<Access>,

    /// Currently tracked stored values. This is the `MAX_ATOMIC_HISTORY` most
    /// recent stores to the atomic cell in loom execution order.
    stores: [Store; MAX_ATOMIC_HISTORY],

    /// The total number of stores to the cell.
    cnt: u16,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum Action {
    /// Atomic load
    Load,

    /// Atomic store
    Store,

    /// Atomic read-modify-write
    Rmw,
}

#[derive(Debug)]
struct Store {
    /// The stored value. All atomic types can be converted to `u64`.
    value: u64,

    /// The causality of the thread when it stores the value.
    happens_before: VersionVec,

    /// Tracks the modification order. Order is tracked as a partially-ordered
    /// set.
    modification_order: VersionVec,

    /// Manages causality transfers between threads
    sync: Synchronize,

    /// Tracks when each thread first saw value
    first_seen: FirstSeen,

    /// True when the store was done with `SeqCst` ordering
    seq_cst: bool,
}

#[derive(Debug)]
struct FirstSeen([u16; MAX_THREADS]);

/// Implements atomic fence behavior
pub(crate) fn fence(ordering: Ordering) {
    rt::synchronize(|execution| match ordering {
        Ordering::Acquire => fence_acq(execution),
        Ordering::Release => fence_rel(execution),
        Ordering::AcqRel => fence_acqrel(execution),
        Ordering::SeqCst => fence_seqcst(execution),
        Ordering::Relaxed => panic!("there is no such thing as a relaxed fence"),
        order => unimplemented!("unimplemented ordering {:?}", order),
    });
}

fn fence_acq(execution: &mut Execution) {
    // Find all stores for all atomic objects and, if they have been read by
    // the current thread, establish an acquire synchronization.
    for state in execution.objects.iter_mut::<State>() {
        // Iterate all the stores
        for store in state.stores_mut() {
            if !store.first_seen.is_seen_by_current(&execution.threads) {
                continue;
            }

            store
                .sync
                .sync_load(&mut execution.threads, Ordering::Acquire);
        }
    }
}

fn fence_rel(execution: &mut Execution) {
    // take snapshot of cur view and record as rel view
    let active = execution.threads.active_mut();
    active.released = active.causality;
}

fn fence_acqrel(execution: &mut Execution) {
    fence_acq(execution);
    fence_rel(execution);
}

fn fence_seqcst(execution: &mut Execution) {
    fence_acqrel(execution);
    execution.threads.seq_cst_fence();
}

impl<T: Numeric> Atomic<T> {
    /// Create a new, atomic cell initialized with the provided value
    pub(crate) fn new(value: T, location: Location) -> Atomic<T> {
        rt::execution(|execution| {
            let state = State::new(&mut execution.threads, value.into_u64(), location);
            let state = execution.objects.insert(state);

            trace!(?state, "Atomic::new");

            Atomic {
                state,
                _p: PhantomData,
            }
        })
    }

    /// Loads a value from the atomic cell.
    pub(crate) fn load(&self, location: Location, ordering: Ordering) -> T {
        self.branch(Action::Load, location);

        super::synchronize(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            // If necessary, generate the list of stores to permute through
            if execution.path.is_traversed() {
                let mut seed = [0; MAX_ATOMIC_HISTORY];

                let n = state.match_load_to_stores(&execution.threads, &mut seed[..], ordering);

                execution.path.push_load(&seed[..n]);
            }

            // Get the store to return from this load.
            let index = execution.path.branch_load();

            trace!(state = ?self.state, ?ordering, "Atomic::load");

            T::from_u64(state.load(&mut execution.threads, index, location, ordering))
        })
    }

    /// Loads a value from the atomic cell without performing synchronization
    pub(crate) fn unsync_load(&self, location: Location) -> T {
        rt::execution(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            state
                .unsync_loaded_locations
                .track(location, &execution.threads);

            // An unsync load counts as a "read" access
            state.track_unsync_load(&execution.threads);

            trace!(state = ?self.state, "Atomic::unsync_load");

            // Return the value
            let index = index(state.cnt - 1);
            T::from_u64(state.stores[index].value)
        })
    }

    /// Stores a value into the atomic cell.
    pub(crate) fn store(&self, location: Location, val: T, ordering: Ordering) {
        self.branch(Action::Store, location);

        super::synchronize(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            state.stored_locations.track(location, &execution.threads);

            // An atomic store counts as a read access to the underlying memory
            // cell.
            state.track_store(&execution.threads);

            trace!(state = ?self.state, ?ordering, "Atomic::store");

            // Do the store
            state.store(
                &mut execution.threads,
                Synchronize::new(),
                val.into_u64(),
                ordering,
            );
        })
    }

    pub(crate) fn rmw<F, E>(
        &self,
        location: Location,
        success: Ordering,
        failure: Ordering,
        f: F,
    ) -> Result<T, E>
    where
        F: FnOnce(T) -> Result<T, E>,
    {
        self.branch(Action::Rmw, location);

        super::synchronize(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            // If necessary, generate the list of stores to permute through
            if execution.path.is_traversed() {
                let mut seed = [0; MAX_ATOMIC_HISTORY];

                let n = state.match_rmw_to_stores(&mut seed[..]);
                execution.path.push_load(&seed[..n]);
            }

            // Get the store to use for the read portion of the rmw operation.
            let index = execution.path.branch_load();

            trace!(state = ?self.state, ?success, ?failure, "Atomic::rmw");

            state
                .rmw(
                    &mut execution.threads,
                    index,
                    location,
                    success,
                    failure,
                    |num| f(T::from_u64(num)).map(T::into_u64),
                )
                .map(T::from_u64)
        })
    }

    /// Access a mutable reference to value most recently stored.
    ///
    /// `with_mut` must happen-after all stores to the cell.
    pub(crate) fn with_mut<R>(&mut self, location: Location, f: impl FnOnce(&mut T) -> R) -> R {
        let value = super::execution(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            state
                .unsync_mut_locations
                .track(location, &execution.threads);
            // Verify the mutation may happen
            state.track_unsync_mut(&execution.threads);
            state.is_mutating = true;

            trace!(state = ?self.state, "Atomic::with_mut");

            // Return the value of the most recent store
            let index = index(state.cnt - 1);
            T::from_u64(state.stores[index].value)
        });

        struct Reset<T: Numeric>(T, object::Ref<State>);

        impl<T: Numeric> Drop for Reset<T> {
            fn drop(&mut self) {
                super::execution(|execution| {
                    let state = self.1.get_mut(&mut execution.objects);

                    // Make sure the state is as expected
                    assert!(state.is_mutating);
                    state.is_mutating = false;

                    // The value may have been mutated, so it must be placed
                    // back.
                    let index = index(state.cnt - 1);
                    state.stores[index].value = T::into_u64(self.0);

                    if !std::thread::panicking() {
                        state.track_unsync_mut(&execution.threads);
                    }
                });
            }
        }

        // Unset on exit
        let mut reset = Reset(value, self.state);
        f(&mut reset.0)
    }

    fn branch(&self, action: Action, location: Location) {
        let r = self.state;
        r.branch_action(action, location);
        assert!(
            r.ref_eq(self.state),
            "Internal state mutated during branch. This is \
                usually due to a bug in the algorithm being tested writing in \
                an invalid memory location."
        );
    }
}

// ===== impl State =====

impl State {
    fn new(threads: &mut thread::Set, value: u64, location: Location) -> State {
        let mut state = State {
            created_location: location,
            loaded_at: VersionVec::new(),
            loaded_locations: LocationSet::new(),
            unsync_loaded_at: VersionVec::new(),
            unsync_loaded_locations: LocationSet::new(),
            stored_at: VersionVec::new(),
            stored_locations: LocationSet::new(),
            unsync_mut_at: VersionVec::new(),
            unsync_mut_locations: LocationSet::new(),
            is_mutating: false,
            last_access: None,
            last_non_load_access: None,
            stores: Default::default(),
            cnt: 0,
        };

        // All subsequent accesses must happen-after.
        state.track_unsync_mut(threads);

        // Store the initial thread
        //
        // The actual order shouldn't matter as operation on the atomic
        // **should** already include the thread causality resulting in the
        // creation of this atomic cell.
        //
        // This is verified using `cell`.
        state.store(threads, Synchronize::new(), value, Ordering::Release);

        state
    }

    fn load(
        &mut self,
        threads: &mut thread::Set,
        index: usize,
        location: Location,
        ordering: Ordering,
    ) -> u64 {
        self.loaded_locations.track(location, threads);
        // Validate memory safety
        self.track_load(threads);

        // Apply coherence rules
        self.apply_load_coherence(threads, index);

        let store = &mut self.stores[index];

        store.first_seen.touch(threads);
        store.sync.sync_load(threads, ordering);
        store.value
    }

    fn store(
        &mut self,
        threads: &mut thread::Set,
        mut sync: Synchronize,
        value: u64,
        ordering: Ordering,
    ) {
        let index = index(self.cnt);

        // Increment the count
        self.cnt += 1;

        // The modification order is initialized to the thread's current
        // causality. All reads / writes that happen before this store are
        // ordered before the store.
        let happens_before = threads.active().causality;

        // Starting with the thread's causality covers WRITE-WRITE coherence
        let mut modification_order = happens_before;

        // Apply coherence rules
        for i in 0..self.stores.len() {
            // READ-WRITE coherence
            if self.stores[i].first_seen.is_seen_by_current(threads) {
                let mo = self.stores[i].modification_order;
                modification_order.join(&mo);
            }
        }

        sync.sync_store(threads, ordering);

        let mut first_seen = FirstSeen::new();
        first_seen.touch(threads);

        // Track the store
        self.stores[index] = Store {
            value,
            happens_before,
            modification_order,
            sync,
            first_seen,
            seq_cst: is_seq_cst(ordering),
        };
    }

    fn rmw<E>(
        &mut self,
        threads: &mut thread::Set,
        index: usize,
        location: Location,
        success: Ordering,
        failure: Ordering,
        f: impl FnOnce(u64) -> Result<u64, E>,
    ) -> Result<u64, E> {
        self.loaded_locations.track(location, threads);

        // Track the load is happening in order to ensure correct
        // synchronization to the underlying cell.
        self.track_load(threads);

        // Apply coherence rules.
        self.apply_load_coherence(threads, index);

        self.stores[index].first_seen.touch(threads);

        let prev = self.stores[index].value;

        match f(prev) {
            Ok(next) => {
                self.stored_locations.track(location, threads);
                // Track a store operation happened
                self.track_store(threads);

                // Perform load synchronization using the `success` ordering.
                self.stores[index].sync.sync_load(threads, success);

                // Store the new value, initializing with the `sync` value from
                // the load. This is our (hacky) way to establish a release
                // sequence.
                let sync = self.stores[index].sync;
                self.store(threads, sync, next, success);

                Ok(prev)
            }
            Err(e) => {
                self.stores[index].sync.sync_load(threads, failure);
                Err(e)
            }
        }
    }

    fn apply_load_coherence(&mut self, threads: &mut thread::Set, index: usize) {
        for i in 0..self.stores.len() {
            // Skip if the is current.
            if index == i {
                continue;
            }

            // READ-READ coherence
            if self.stores[i].first_seen.is_seen_by_current(threads) {
                let mo = self.stores[i].modification_order;
                self.stores[index].modification_order.join(&mo);
            }

            // WRITE-READ coherence
            if self.stores[i].happens_before < threads.active().causality {
                let mo = self.stores[i].modification_order;
                self.stores[index].modification_order.join(&mo);
            }
        }
    }

    /// Track an atomic load
    fn track_load(&mut self, threads: &thread::Set) {
        assert!(!self.is_mutating, "atomic cell is in `with_mut` call");

        let current = &threads.active().causality;

        if let Some(mut_at) = current.ahead(&self.unsync_mut_at) {
            location::panic("Causality violation: Concurrent load and mut accesses.")
                .location("created", self.created_location)
                .thread("with_mut", mut_at, self.unsync_mut_locations[mut_at])
                .thread("load", threads.active_id(), self.loaded_locations[threads])
                .fire();
        }

        self.loaded_at.join(current);
    }

    /// Track an unsynchronized load
    fn track_unsync_load(&mut self, threads: &thread::Set) {
        assert!(!self.is_mutating, "atomic cell is in `with_mut` call");

        let current = &threads.active().causality;

        if let Some(mut_at) = current.ahead(&self.unsync_mut_at) {
            location::panic("Causality violation: Concurrent `unsync_load` and mut accesses.")
                .location("created", self.created_location)
                .thread("with_mut", mut_at, self.unsync_mut_locations[mut_at])
                .thread(
                    "unsync_load",
                    threads.active_id(),
                    self.unsync_loaded_locations[threads],
                )
                .fire();
        }

        if let Some(stored) = current.ahead(&self.stored_at) {
            location::panic("Causality violation: Concurrent `unsync_load` and atomic store.")
                .location("created", self.created_location)
                .thread("atomic store", stored, self.stored_locations[stored])
                .thread(
                    "unsync_load",
                    threads.active_id(),
                    self.unsync_loaded_locations[threads],
                )
                .fire();
        }

        self.unsync_loaded_at.join(current);
    }

    /// Track an atomic store
    fn track_store(&mut self, threads: &thread::Set) {
        assert!(!self.is_mutating, "atomic cell is in `with_mut` call");

        let current = &threads.active().causality;

        if let Some(mut_at) = current.ahead(&self.unsync_mut_at) {
            location::panic("Causality violation: Concurrent atomic store and mut accesses.")
                .location("created", self.created_location)
                .thread("with_mut", mut_at, self.unsync_mut_locations[mut_at])
                .thread(
                    "atomic store",
                    threads.active_id(),
                    self.stored_locations[threads],
                )
                .fire();
        }

        if let Some(loaded) = current.ahead(&self.unsync_loaded_at) {
            location::panic(
                "Causality violation: Concurrent atomic store and `unsync_load` accesses.",
            )
            .location("created", self.created_location)
            .thread("unsync_load", loaded, self.unsync_loaded_locations[loaded])
            .thread(
                "atomic store",
                threads.active_id(),
                self.stored_locations[threads],
            )
            .fire();
        }

        self.stored_at.join(current);
    }

    /// Track an unsynchronized mutation
    fn track_unsync_mut(&mut self, threads: &thread::Set) {
        assert!(!self.is_mutating, "atomic cell is in `with_mut` call");

        let current = &threads.active().causality;

        if let Some(loaded) = current.ahead(&self.loaded_at) {
            location::panic("Causality violation: Concurrent atomic load and unsync mut accesses.")
                .location("created", self.created_location)
                .thread("atomic load", loaded, self.loaded_locations[loaded])
                .thread(
                    "with_mut",
                    threads.active_id(),
                    self.unsync_mut_locations[threads],
                )
                .fire();
        }

        if let Some(loaded) = current.ahead(&self.unsync_loaded_at) {
            location::panic(
                "Causality violation: Concurrent `unsync_load` and unsync mut accesses.",
            )
            .location("created", self.created_location)
            .thread("unsync_load", loaded, self.unsync_loaded_locations[loaded])
            .thread(
                "with_mut",
                threads.active_id(),
                self.unsync_mut_locations[threads],
            )
            .fire();
        }

        if let Some(stored) = current.ahead(&self.stored_at) {
            location::panic(
                "Causality violation: Concurrent atomic store and unsync mut accesses.",
            )
            .location("created", self.created_location)
            .thread("atomic store", stored, self.stored_locations[stored])
            .thread(
                "with_mut",
                threads.active_id(),
                self.unsync_mut_locations[threads],
            )
            .fire();
        }

        if let Some(mut_at) = current.ahead(&self.unsync_mut_at) {
            location::panic("Causality violation: Concurrent unsync mut accesses.")
                .location("created", self.created_location)
                .thread("with_mut one", mut_at, self.unsync_mut_locations[mut_at])
                .thread(
                    "with_mut two",
                    threads.active_id(),
                    self.unsync_mut_locations[threads],
                )
                .fire();
        }

        self.unsync_mut_at.join(current);
    }

    /// Find all stores that could be returned by an atomic load.
    fn match_load_to_stores(
        &self,
        threads: &thread::Set,
        dst: &mut [u8],
        ordering: Ordering,
    ) -> usize {
        let mut n = 0;
        let cnt = self.cnt as usize;

        // We only need to consider loads as old as the **most** recent load
        // seen by each thread in the current causality.
        //
        // This probably isn't the smartest way to implement this, but someone
        // else can figure out how to improve on it if it turns out to be a
        // bottleneck.
        //
        // Add all stores **unless** a newer store has already been seen by the
        // current thread's causality.
        'outer: for i in 0..self.stores.len() {
            let store_i = &self.stores[i];

            if i >= cnt {
                // Not a real store
                continue;
            }

            for j in 0..self.stores.len() {
                let store_j = &self.stores[j];

                if i == j || j >= cnt {
                    continue;
                }

                let mo_i = store_i.modification_order;
                let mo_j = store_j.modification_order;

                // TODO: this sometimes fails
                assert_ne!(mo_i, mo_j);

                if mo_i < mo_j {
                    if store_j.first_seen.is_seen_by_current(threads) {
                        // Store `j` is newer, so don't store the current one.
                        continue 'outer;
                    }

                    if store_i.first_seen.is_seen_before_yield(threads) {
                        // Saw this load before the previous yield. In order to
                        // advance the model, don't return it again.
                        continue 'outer;
                    }

                    if is_seq_cst(ordering) && store_i.seq_cst && store_j.seq_cst {
                        // There is a newer SeqCst store
                        continue 'outer;
                    }
                }
            }

            // The load may return this store
            dst[n] = i as u8;
            n += 1;
        }

        n
    }

    fn match_rmw_to_stores(&self, dst: &mut [u8]) -> usize {
        let mut n = 0;
        let cnt = self.cnt as usize;

        // Unlike `match_load_to_stores`, rmw operations only load "newest"
        // stores, in terms of modification order.
        'outer: for i in 0..self.stores.len() {
            let store_i = &self.stores[i];

            if i >= cnt {
                // Not a real store
                continue;
            }

            for j in 0..self.stores.len() {
                let store_j = &self.stores[j];

                if i == j || j >= cnt {
                    continue;
                }

                let mo_i = store_i.modification_order;
                let mo_j = store_j.modification_order;

                assert_ne!(mo_i, mo_j);

                if mo_i < mo_j {
                    // There is a newer store.
                    continue 'outer;
                }
            }

            // The load may return this store
            dst[n] = i as u8;
            n += 1;
        }

        n
    }

    fn stores_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Store> {
        let (start, end) = range(self.cnt);
        let (two, one) = self.stores[..end].split_at_mut(start);

        one.iter_mut().chain(two.iter_mut())
    }

    /// Returns the last dependent access
    pub(super) fn last_dependent_access(&self, action: Action) -> Option<&Access> {
        match action {
            Action::Load => self.last_non_load_access.as_ref(),
            _ => self.last_access.as_ref(),
        }
    }

    /// Sets the last dependent access
    pub(super) fn set_last_access(&mut self, action: Action, path_id: usize, version: &VersionVec) {
        // Always set `last_access`
        Access::set_or_create(&mut self.last_access, path_id, version);

        match action {
            Action::Load => {}
            _ => {
                // Stores / RMWs
                Access::set_or_create(&mut self.last_non_load_access, path_id, version);
            }
        }
    }
}

// ===== impl Store =====

impl Default for Store {
    fn default() -> Store {
        Store {
            value: 0,
            happens_before: VersionVec::new(),
            modification_order: VersionVec::new(),
            sync: Synchronize::new(),
            first_seen: FirstSeen::new(),
            seq_cst: false,
        }
    }
}

// ===== impl FirstSeen =====

impl FirstSeen {
    fn new() -> FirstSeen {
        FirstSeen([u16::max_value(); MAX_THREADS])
    }

    fn touch(&mut self, threads: &thread::Set) {
        if self.0[threads.active_id().as_usize()] == u16::max_value() {
            self.0[threads.active_id().as_usize()] = threads.active_atomic_version();
        }
    }

    fn is_seen_by_current(&self, threads: &thread::Set) -> bool {
        for (thread_id, version) in threads.active().causality.versions(threads.execution_id()) {
            match self.0[thread_id.as_usize()] {
                u16::MAX => {}
                v if v <= version => return true,
                _ => {}
            }
        }

        false
    }

    fn is_seen_before_yield(&self, threads: &thread::Set) -> bool {
        let thread_id = threads.active_id();

        let last_yield = match threads.active().last_yield {
            Some(v) => v,
            None => return false,
        };

        match self.0[thread_id.as_usize()] {
            u16::MAX => false,
            v => v <= last_yield,
        }
    }
}

fn is_seq_cst(order: Ordering) -> bool {
    order == Ordering::SeqCst
}

fn range(cnt: u16) -> (usize, usize) {
    let start = index(cnt.saturating_sub(MAX_ATOMIC_HISTORY as u16));
    let mut end = index(cmp::min(cnt, MAX_ATOMIC_HISTORY as u16));

    if end == 0 {
        end = MAX_ATOMIC_HISTORY;
    }

    assert!(
        start <= end,
        "[loom internal bug] cnt = {}; start = {}; end = {}",
        cnt,
        start,
        end
    );

    (start, end)
}

fn index(cnt: u16) -> usize {
    cnt as usize % MAX_ATOMIC_HISTORY
}
