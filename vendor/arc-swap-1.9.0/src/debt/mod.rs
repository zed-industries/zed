//! Debt handling.
//!
//! A debt is a reference count of a smart pointer that is owed. This module provides a lock-free
//! storage for debts.
//!
//! Each thread has its own node with bunch of slots. Only that thread can allocate debts in there,
//! but others are allowed to inspect and pay them. The nodes form a linked list for the reason of
//! inspection. The nodes are never removed (even after the thread terminates), but if the thread
//! gives it up, another (new) thread can claim it.
//!
//! The writers walk the whole chain and pay the debts (by bumping the ref counts) of the just
//! removed pointer.
//!
//! Each node has some fast (but fallible) nodes and a fallback node, with different algorithms to
//! claim them (see the relevant submodules).

use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::*;

pub(crate) use self::list::{LocalNode, Node};
use super::RefCnt;

mod fast;
mod helping;
mod list;

/// One debt slot.
///
/// It may contain an „owed“ reference count.
#[derive(Debug)]
pub(crate) struct Debt(pub(crate) AtomicUsize);

impl Debt {
    /// The value of pointer `3` should be pretty safe, for two reasons:
    ///
    /// * It's an odd number, but the pointers we have are likely aligned at least to the word size,
    ///   because the data at the end of the `Arc` has the counters.
    /// * It's in the very first page where NULL lives, so it's not mapped.
    pub(crate) const NONE: usize = 0b11;
}

impl Default for Debt {
    fn default() -> Self {
        Debt(AtomicUsize::new(Self::NONE))
    }
}

impl Debt {
    /// Tries to pay the given debt.
    ///
    /// If the debt is still there, for the given pointer, it is paid and `true` is returned. If it
    /// is empty or if there's some other pointer, it is not paid and `false` is returned, meaning
    /// the debt was paid previously by someone else.
    ///
    /// # Notes
    ///
    /// * It is possible that someone paid the debt and then someone else put a debt for the same
    ///   pointer in there. This is fine, as we'll just pay the debt for that someone else.
    /// * This relies on the fact that the same pointer must point to the same object and
    ///   specifically to the same type ‒ the caller provides the type, it's destructor, etc.
    /// * It also relies on the fact the same thing is not stuffed both inside an `Arc` and `Rc` or
    ///   something like that, but that sounds like a reasonable assumption. Someone storing it
    ///   through `ArcSwap<T>` and someone else with `ArcSwapOption<T>` will work.
    #[inline]
    pub(crate) fn pay<T: RefCnt>(&self, ptr: *const T::Base) -> bool {
        self.0
            // On failure, we have observed that the debt has been paid, but we need to establish a
            // happens-before relationship with that debt being paid before we do anything that
            // relies on the Arc's strong counter being incremented, so we need to Acquire.
            // Arc does not sufficiently take care of this, as the memory model permits its Drop's
            // `fetch_sub` to observe itself to be last remaining instance before the `fetch_add`s
            // from the debt being repaid, and only after that observation does Arc have an Acquire
            // fence.
            //
            // Unfortunately, we need SeqCst on the success
            .compare_exchange(ptr as usize, Self::NONE, SeqCst, Acquire)
            .is_ok()
    }

    /// Pays all the debts on the given pointer and the storage.
    pub(crate) fn pay_all<T, R>(ptr: *const T::Base, storage_addr: usize, replacement: R)
    where
        T: RefCnt,
        R: Fn() -> T,
    {
        LocalNode::with(|local| {
            let val = unsafe { T::from_ptr(ptr) };
            // Pre-pay one ref count that can be safely put into a debt slot to pay it.
            T::inc(&val);

            Node::traverse::<(), _>(|node| {
                // Make the cooldown trick know we are poking into this node.
                let _reservation = node.reserve_writer();

                local.help(node, storage_addr, &replacement);

                let all_slots = node
                    .fast_slots()
                    .chain(core::iter::once(node.helping_slot()));
                for slot in all_slots {
                    // Note: Release is enough even here. That makes sure the increment is
                    // visible to whoever might acquire on this slot and can't leak below this.
                    // And we are the ones doing decrements anyway.
                    if slot.pay::<T>(ptr) {
                        // Pre-pay one more, for another future slot
                        T::inc(&val);
                    }
                }

                None
            });
            // Implicit dec by dropping val in here, pair for the above
        })
    }
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;

    /// Checks the assumption that arcs to ZSTs have different pointer values.
    #[test]
    fn arc_zst() {
        struct A;
        struct B;

        let a = Arc::new(A);
        let b = Arc::new(B);

        let aref: &A = &a;
        let bref: &B = &b;

        let aptr = aref as *const _ as usize;
        let bptr = bref as *const _ as usize;
        assert_ne!(aptr, bptr);
    }
}
