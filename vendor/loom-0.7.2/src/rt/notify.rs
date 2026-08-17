use crate::rt::object;
use crate::rt::{self, Access, Synchronize, VersionVec};

use std::sync::atomic::Ordering::{Acquire, Release};

use tracing::trace;

use super::Location;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Notify {
    state: object::Ref<State>,
}

#[derive(Debug)]
pub(super) struct State {
    /// If true, spurious notifications are possible
    spurious: bool,

    /// True if the notify woke up spuriously last time
    did_spur: bool,

    /// When true, notification is sequential consistent.
    seq_cst: bool,

    /// `true` if there is a pending notification to consume.
    notified: bool,

    /// Tracks access to the notify object
    last_access: Option<Access>,

    /// Causality transfers between threads
    synchronize: Synchronize,
}

impl Notify {
    pub(crate) fn new(seq_cst: bool, spurious: bool) -> Notify {
        super::execution(|execution| {
            let state = execution.objects.insert(State {
                spurious,
                did_spur: false,
                seq_cst,
                notified: false,
                last_access: None,
                synchronize: Synchronize::new(),
            });

            trace!(?state, ?seq_cst, ?spurious, "Notify::new");

            Notify { state }
        })
    }

    pub(crate) fn notify(self, location: Location) {
        self.state.branch_opaque(location);

        rt::execution(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            state
                .synchronize
                .sync_store(&mut execution.threads, Release);

            if state.seq_cst {
                execution.threads.seq_cst();
            }

            state.notified = true;

            let (active, inactive) = execution.threads.split_active();

            for thread in inactive {
                let obj = thread
                    .operation
                    .as_ref()
                    .map(|operation| operation.object());

                if obj == Some(self.state.erase()) {
                    trace!(state = ?self.state, thread = ?thread.id, "Notify::notify");

                    thread.unpark(active);
                }
            }
        });
    }

    pub(crate) fn wait(self, location: Location) {
        let (notified, spurious) = rt::execution(|execution| {
            let spurious = if self.state.get(&execution.objects).might_spur() {
                execution.path.branch_spurious()
            } else {
                false
            };

            let state = self.state.get_mut(&mut execution.objects);

            if spurious {
                state.did_spur = true;
            }

            trace!(state = ?self.state, notified = ?state.notified, ?spurious, "Notify::wait 1");
            dbg!((state.notified, spurious))
        });

        if spurious {
            rt::yield_now();
            return;
        }

        if notified {
            self.state.branch_opaque(location);
        } else {
            // This should become branch_disable
            self.state.branch_acquire(true, location)
        }

        // Thread was notified
        super::execution(|execution| {
            trace!(state = ?self.state, "Notify::wait 2");
            let state = self.state.get_mut(&mut execution.objects);

            assert!(state.notified);

            state.synchronize.sync_load(&mut execution.threads, Acquire);

            if state.seq_cst {
                // Establish sequential consistency between locks
                execution.threads.seq_cst();
            }

            state.notified = false;
        });
    }
}

impl State {
    pub(crate) fn might_spur(&self) -> bool {
        self.spurious && !self.did_spur
    }

    pub(crate) fn last_dependent_access(&self) -> Option<&Access> {
        self.last_access.as_ref()
    }

    pub(crate) fn set_last_access(&mut self, path_id: usize, version: &VersionVec) {
        Access::set_or_create(&mut self.last_access, path_id, version);
    }
}
