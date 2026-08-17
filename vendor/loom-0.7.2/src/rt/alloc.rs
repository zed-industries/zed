use crate::rt;
use crate::rt::{object, Location};

use tracing::trace;

/// Tracks an allocation
#[derive(Debug)]
pub(crate) struct Allocation {
    state: object::Ref<State>,
}

#[derive(Debug)]
pub(super) struct State {
    is_dropped: bool,
    allocated: Location,
}

/// Track a raw allocation
pub(crate) fn alloc(ptr: *mut u8, location: Location) {
    rt::execution(|execution| {
        let state = execution.objects.insert(State {
            is_dropped: false,
            allocated: location,
        });

        let allocation = Allocation { state };

        trace!(?allocation.state, ?ptr, %location, "alloc");

        let prev = execution.raw_allocations.insert(ptr as usize, allocation);
        assert!(prev.is_none(), "pointer already tracked");
    });
}

/// Track a raw deallocation
pub(crate) fn dealloc(ptr: *mut u8, location: Location) {
    let allocation =
        rt::execution(
            |execution| match execution.raw_allocations.remove(&(ptr as usize)) {
                Some(allocation) => {
                    trace!(state = ?allocation.state, ?ptr, %location, "dealloc");

                    allocation
                }
                None => panic!("pointer not tracked"),
            },
        );

    // Drop outside of the `rt::execution` block
    drop(allocation);
}

impl Allocation {
    pub(crate) fn new(location: Location) -> Allocation {
        rt::execution(|execution| {
            let state = execution.objects.insert(State {
                is_dropped: false,
                allocated: location,
            });

            trace!(?state, %location, "Allocation::new");

            Allocation { state }
        })
    }
}

impl Drop for Allocation {
    #[track_caller]
    fn drop(&mut self) {
        let location = location!();
        rt::execution(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            trace!(state = ?self.state, drop.location = %location, "Allocation::drop");

            state.is_dropped = true;
        });
    }
}

impl State {
    pub(super) fn check_for_leaks(&self, index: usize) {
        if !self.is_dropped {
            if self.allocated.is_captured() {
                panic!(
                    "Allocation leaked.\n  Allocated: {}\n      Index: {}",
                    self.allocated, index
                );
            } else {
                panic!("Allocation leaked.\n  Index: {}", index);
            }
        }
    }
}
