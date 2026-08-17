use crate::rt::{thread, VersionVec};

use std::sync::atomic::Ordering::{self, *};

/// A synchronization point between two threads.
///
/// Threads synchronize with this point using any of the available orderings. On
/// loads, the thread's causality is updated using the synchronization point's
/// stored causality. On stores, the synchronization point's causality is
/// updated with the threads.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Synchronize {
    happens_before: VersionVec,
}

impl Synchronize {
    pub fn new() -> Self {
        Synchronize {
            happens_before: VersionVec::new(),
        }
    }

    pub fn sync_load(&mut self, threads: &mut thread::Set, order: Ordering) {
        match order {
            Relaxed | Release => {
                // Nothing happens!
            }
            Acquire | AcqRel => {
                self.sync_acq(threads);
            }
            SeqCst => {
                self.sync_acq(threads);
                threads.seq_cst();
            }
            order => unimplemented!("unimplemented ordering {:?}", order),
        }
    }

    pub fn sync_store(&mut self, threads: &mut thread::Set, order: Ordering) {
        self.happens_before.join(&threads.active().released);
        match order {
            Relaxed | Acquire => {
                // Nothing happens!
            }
            Release | AcqRel => {
                self.sync_rel(threads);
            }
            SeqCst => {
                self.sync_rel(threads);
                threads.seq_cst();
            }
            order => unimplemented!("unimplemented ordering {:?}", order),
        }
    }

    fn sync_acq(&mut self, threads: &mut thread::Set) {
        threads.active_mut().causality.join(&self.happens_before);
    }

    fn sync_rel(&mut self, threads: &thread::Set) {
        self.happens_before.join(&threads.active().causality);
    }
}
