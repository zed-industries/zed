//! The fast slots for the primary strategy.
//!
//! They are faster, but fallible (in case the slots run out or if there's a collision with a
//! writer thread, this gives up and falls back to secondary strategy).
//!
//! They are based on hazard pointer ideas. To acquire one, the pointer is loaded, stored in the
//! slot and the debt is confirmed by loading it again and checking it is the same.
//!
//! # Orderings
//!
//! We ensure just one thing here. Since we do both the acquisition of the slot and the exchange of
//! the pointer in the writer with SeqCst, we are guaranteed to either see the change in case it
//! hits somewhere in between the two reads of the pointer, or to have successfully acquired it
//! before the change and before any cleanup of the old pointer happened (in which case we know the
//! writer will see our debt).

use core::cell::Cell;
use core::slice::Iter;
use core::sync::atomic::Ordering::*;

use super::Debt;

const DEBT_SLOT_CNT: usize = 8;

/// Thread-local information for the [`Slots`]
#[derive(Default)]
pub(super) struct Local {
    // The next slot in round-robin rotation. Heuristically tries to balance the load across them
    // instead of having all of them stuffed towards the start of the array which gets
    // unsuccessfully iterated through every time.
    offset: Cell<usize>,
}

/// Bunch of fast debt slots.
#[derive(Default)]
pub(super) struct Slots([Debt; DEBT_SLOT_CNT]);

impl Slots {
    /// Try to allocate one slot and get the pointer in it.
    ///
    /// Fails if there are no free slots.
    #[inline]
    pub(super) fn get_debt(&self, ptr: usize, local: &Local) -> Option<&Debt> {
        // Trick with offsets: we rotate through the slots (save the value from last time)
        // so successive leases are likely to succeed on the first attempt (or soon after)
        // instead of going through the list of already held ones.
        let offset = local.offset.get();
        let len = self.0.len();
        for i in 0..len {
            let i = (i + offset) % len;
            // Note: the indexing check is almost certainly optimised out because the len
            // is used above. And using .get_unchecked was actually *slower*.
            let slot = &self.0[i];
            if slot.0.load(Relaxed) == Debt::NONE {
                // We are allowed to split into the check and acquiring the debt. That's because we
                // are the only ones allowed to change NONE to something else. But we still need a
                // read-write operation with SeqCst on it :-(
                let old = slot.0.swap(ptr, SeqCst);
                debug_assert_eq!(Debt::NONE, old);
                local.offset.set(i + 1);
                return Some(&self.0[i]);
            }
        }
        None
    }
}

impl<'a> IntoIterator for &'a Slots {
    type Item = &'a Debt;

    type IntoIter = Iter<'a, Debt>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
