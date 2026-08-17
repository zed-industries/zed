use crate::rt::alloc::Allocation;
use crate::rt::{lazy_static, object, thread, Path};

use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;

use tracing::info;

pub(crate) struct Execution {
    /// Uniquely identifies an execution
    pub(super) id: Id,

    /// Execution path taken
    pub(crate) path: Path,

    pub(crate) threads: thread::Set,

    pub(crate) lazy_statics: lazy_static::Set,

    /// All loom aware objects part of this execution run.
    pub(super) objects: object::Store,

    /// Maps raw allocations to LeakTrack objects
    pub(super) raw_allocations: HashMap<usize, Allocation>,

    pub(crate) arc_objs: HashMap<*const (), std::sync::Arc<super::Arc>>,

    /// Maximum number of concurrent threads
    pub(super) max_threads: usize,

    pub(super) max_history: usize,

    /// Capture locations for significant events
    pub(crate) location: bool,

    /// Log execution output to STDOUT
    pub(crate) log: bool,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub(crate) struct Id(usize);

impl Execution {
    /// Create a new execution.
    ///
    /// This is only called at the start of a fuzz run. The same instance is
    /// reused across permutations.
    pub(crate) fn new(
        max_threads: usize,
        max_branches: usize,
        preemption_bound: Option<usize>,
        exploring: bool,
    ) -> Execution {
        let id = Id::new();
        let threads = thread::Set::new(id, max_threads);

        let preemption_bound =
            preemption_bound.map(|bound| bound.try_into().expect("preemption_bound too big"));

        Execution {
            id,
            path: Path::new(max_branches, preemption_bound, exploring),
            threads,
            lazy_statics: lazy_static::Set::new(),
            objects: object::Store::with_capacity(max_branches),
            raw_allocations: HashMap::new(),
            arc_objs: HashMap::new(),
            max_threads,
            max_history: 7,
            location: false,
            log: false,
        }
    }

    /// Create state to track a new thread
    pub(crate) fn new_thread(&mut self) -> thread::Id {
        let thread_id = self.threads.new_thread();
        let active_id = self.threads.active_id();

        let (active, new) = self.threads.active2_mut(thread_id);

        new.causality.join(&active.causality);
        new.dpor_vv.join(&active.dpor_vv);

        // Bump causality in order to ensure CausalCell accurately detects
        // incorrect access when first action.
        new.causality[thread_id] += 1;
        active.causality[active_id] += 1;

        thread_id
    }

    /// Resets the execution state for the next execution run
    pub(crate) fn step(self) -> Option<Self> {
        let id = Id::new();
        let max_threads = self.max_threads;
        let max_history = self.max_history;
        let location = self.location;
        let log = self.log;
        let mut path = self.path;
        let mut objects = self.objects;
        let mut lazy_statics = self.lazy_statics;
        let mut raw_allocations = self.raw_allocations;
        let mut arc_objs = self.arc_objs;

        let mut threads = self.threads;

        if !path.step() {
            return None;
        }

        objects.clear();
        lazy_statics.reset();
        raw_allocations.clear();
        arc_objs.clear();

        threads.clear(id);

        Some(Execution {
            id,
            path,
            threads,
            objects,
            lazy_statics,
            raw_allocations,
            arc_objs,
            max_threads,
            max_history,
            location,
            log,
        })
    }

    /// Returns `true` if a switch is required
    pub(crate) fn schedule(&mut self) -> bool {
        use crate::rt::path::Thread;

        // Implementation of the DPOR algorithm.

        let curr_thread = self.threads.active_id();

        for (th_id, th) in self.threads.iter() {
            let operation = match th.operation {
                Some(operation) => operation,
                None => continue,
            };

            if let Some(access) = self.objects.last_dependent_access(operation) {
                if access.happens_before(&th.dpor_vv) {
                    // The previous access happened before this access, thus
                    // there is no race.
                    continue;
                }

                // Get the point to backtrack to
                let point = access.path_id();

                // Track backtracking point
                self.path.backtrack(point, th_id);
            }
        }

        // It's important to avoid pre-emption as much as possible
        let mut initial = Some(self.threads.active_id());

        // If the thread is not runnable, then we can pick any arbitrary other
        // runnable thread.
        if !self.threads.active().is_runnable() {
            initial = None;

            for (i, th) in self.threads.iter() {
                if !th.is_runnable() {
                    continue;
                }

                if let Some(ref mut init) = initial {
                    if th.yield_count < self.threads[*init].yield_count {
                        *init = i;
                    }
                } else {
                    initial = Some(i)
                }
            }
        }

        let path_id = self.path.pos();

        let next = self.path.branch_thread(self.id, {
            self.threads.iter().map(|(i, th)| {
                if initial.is_none() && th.is_runnable() {
                    initial = Some(i);
                }

                if initial == Some(i) {
                    Thread::Active
                } else if th.is_yield() {
                    Thread::Yield
                } else if !th.is_runnable() {
                    Thread::Disabled
                } else {
                    Thread::Skip
                }
            })
        });

        let switched = Some(self.threads.active_id()) != next;

        self.threads.set_active(next);

        // There is no active thread. Unless all threads have terminated, the
        // test has deadlocked.
        if !self.threads.is_active() {
            let terminal = self.threads.iter().all(|(_, th)| th.is_terminated());

            assert!(
                terminal,
                "deadlock; threads = {:?}",
                self.threads
                    .iter()
                    .map(|(i, th)| { (i, th.state) })
                    .collect::<Vec<_>>()
            );

            return true;
        }

        // TODO: refactor
        if let Some(operation) = self.threads.active().operation {
            let threads = &mut self.threads;
            let th_id = threads.active_id();

            if let Some(access) = self.objects.last_dependent_access(operation) {
                threads.active_mut().dpor_vv.join(access.version());
            }

            threads.active_mut().dpor_vv[th_id] += 1;

            self.objects
                .set_last_access(operation, path_id, &threads.active().dpor_vv);
        }

        // Reactivate yielded threads, but only if the current active thread is
        // not yielded.
        for (id, th) in self.threads.iter_mut() {
            if th.is_yield() && Some(id) != next {
                th.set_runnable();
            }
        }

        if switched {
            info!("~~~~~~~~ THREAD {} ~~~~~~~~", self.threads.active_id());
        }

        curr_thread != self.threads.active_id()
    }

    /// Panics if any leaks were detected
    pub(crate) fn check_for_leaks(&self) {
        self.objects.check_for_leaks();
    }
}

impl fmt::Debug for Execution {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Execution")
            .field("path", &self.path)
            .field("threads", &self.threads)
            .finish()
    }
}

impl Id {
    pub(crate) fn new() -> Id {
        use std::sync::atomic::AtomicUsize;
        use std::sync::atomic::Ordering::Relaxed;

        // The number picked here is arbitrary. It is mostly to avoid collision
        // with "zero" to aid with debugging.
        static NEXT_ID: AtomicUsize = AtomicUsize::new(46_413_762);

        let next = NEXT_ID.fetch_add(1, Relaxed);

        Id(next)
    }
}
