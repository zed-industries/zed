//! Slots and global/thread local data for the Helping strategy.
//!
//! This is inspired (but not an exact copy) of
//! <https://pvk.ca/Blog/2020/07/07/flatter-wait-free-hazard-pointers/>. The debts are mostly
//! copies of the ones used by the hybrid strategy, but modified a bit. Just like in the hybrid
//! strategy, in case the slots run out or when the writer updates the value, the debts are paid by
//! incrementing the ref count (which is a little slower, but still wait-free/lock-free and still
//! in order of nanoseconds).
//!
//! ## Reader, the fast path
//!
//! * Publish an active address ‒ the address we'll be loading stuff from.
//! * Puts a generation into the control.
//! * Loads the pointer and puts it to the debt slot.
//! * Confirms by CaS-replacing the generation back to idle state.
//!
//! * Later, we pay it back by CaS-replacing it with the NO_DEPT (like any other slot).
//!
//! ## Writer, the non-colliding path
//!
//! * Replaces the pointer in the storage.
//! * The writer walks over all debts. It pays each debt that it is concerned with by bumping the
//!   reference and replacing the dept with NO_DEPT. The relevant reader will fail in the CaS
//!   (either because it finds NO_DEPT or other pointer in there) and knows the reference was
//!   bumped, so it needs to decrement it. Note that it is possible that someone also reuses the
//!   slot for the _same_ pointer. In that case that reader will set it to NO_DEPT and the newer
//!   reader will have a pre-paid debt, which is fine.
//!
//! ## The collision path
//!
//! The reservation of a slot is not atomic, therefore a writer can observe the reservation in
//! progress. But it doesn't want to wait for it to complete (it wants to be lock-free, which means
//! it needs to be able to resolve the situation on its own).
//!
//! The way it knows it is in progress of the reservation is by seeing a generation in there (it has
//! a distinct tag). In that case it'll try to:
//!
//! * First verify that the reservation is being done for the same address it modified, by reading
//!   and re-confirming the active_addr slot corresponding to the currently handled node. If it is
//!   for some other address, the writer doesn't have to be concerned and proceeds to the next slot.
//! * It does a full load. That is fine, because the writer must be on a different thread than the
//!   reader and therefore there is at least one free slot. Full load means paying the debt right
//!   away by incrementing the reference count.
//! * Then it tries to pass the already fully protected/paid pointer to the reader. It writes it to
//!   an envelope and CaS-replaces it into the control, instead of the generation (if it fails,
//!   someone has been faster and it rolls back). We need the envelope because the pointer itself
//!   doesn't have to be aligned to 4 bytes and we need the space for tags to distinguish the types
//!   of info in control; we can ensure the envelope is).
//! * The reader then finds the generation got replaced by a pointer to the envelope and uses that
//!   pointer inside the envelope. It aborts its own debt. This effectively exchanges the envelopes
//!   between the threads so each one has an envelope ready for future.
//!
//! ## ABA protection
//!
//! The generation as pre-reserving the slot allows the writer to make sure it is offering the
//! loaded pointer to the same reader and that the read value is new enough (and of the same type).
//!
//! This solves the general case, but there's also much less frequent but theoretical ABA problem
//! that could lead to UB, if left unsolved:
//!
//! * There is a collision on generation G.
//! * The writer loads a pointer, bumps it.
//! * In the meantime, all the 2^30 or 2^62 generations (depending on the usize width) generations
//!   wrap around.
//! * The writer stores the outdated and possibly different-typed pointer in there and the reader
//!   uses it.
//!
//! To mitigate that, every time the counter overflows we take the current node and un-assign it
//! from our current thread. We mark it as in "cooldown" and let it in there until there are no
//! writers messing with that node any more (if they are not on the node, they can't experience the
//! ABA problem on it). After that, we are allowed to use it again.
//!
//! This doesn't block the reader, it'll simply find *a* node next time ‒ this one, or possibly a
//! different (or new) one.
//!
//! # Orderings
//!
//! The linked lists/nodes are already provided for us. So we just need to make sure the debt
//! juggling is done right. We assume that the local node is ours to write to (others have only
//! limited right to write to certain fields under certain conditions) and that we are counted into
//! active writers while we dig through it on the writer end.
//!
//! We use SeqCst on a read-write operation both here at the very start of the sequence (storing
//! the generation into the control) and in the writer on the actual pointer. That establishes a
//! relation of what has happened first.
//!
//! After that we split the time into segments by read-write operations with AcqRel read-write
//! operations on the control. There's just one control in play for both threads so we don't need
//! SeqCst and the segments are understood by both the same way. The writer can sometimes use only
//! load-Acquire on that, because it needs to only read from data written by the reader. It'll
//! always see data from at least the segment before the observed control value and uses CaS to
//! send the results back, so it can't go into the past.
//!
//! There are two little gotchas:
//!
//! * When we read the address we should be loading from, we need to give up if the address does
//!   not match (we can't simply load from there, because it can be dangling by that point and we
//!   don't know its type, so we need to use our address for all loading ‒ and we just check they
//!   match). If we give up, we don't do that CaS into control, therefore we could have given up on
//!   newer address than the control we have read. For that reason, the address is also stored by
//!   reader with Release and we read it with Acquire, which'll bring an up to date version of
//!   control into our thread ‒ and we re-read that one to confirm the address is indeed between
//!   two same values holding the generation, therefore corresponding to it.
//! * The destructor doesn't have a SeqCst in the writer, because there was no write in there.
//!   That's OK. We need to ensure there are no new readers after the "change" we confirm in the
//!   writer and that change is the destruction ‒ by that time, the destroying thread has exclusive
//!   ownership and therefore there can be no new readers.

use core::cell::Cell;
use core::ptr;
use core::sync::atomic::Ordering::*;
use core::sync::atomic::{AtomicPtr, AtomicUsize};

use super::Debt;
use crate::RefCnt;

pub const REPLACEMENT_TAG: usize = 0b01;
pub const GEN_TAG: usize = 0b10;
pub const TAG_MASK: usize = 0b11;
pub const IDLE: usize = 0;

/// Thread local data for the helping strategy.
#[derive(Default)]
pub(super) struct Local {
    // The generation counter.
    generation: Cell<usize>,
}

// Make sure the pointers have 2 empty bits. Always.
#[derive(Default)]
#[repr(align(4))]
struct Handover(AtomicUsize);

/// The slots for the helping strategy.
pub(super) struct Slots {
    /// The control structure of the slot.
    ///
    /// Different threads signal what stage they are in in there. It can contain:
    ///
    /// * `IDLE` (nothing is happening, and there may or may not be an active debt).
    /// * a generation, tagged with GEN_TAG. The reader is trying to acquire a slot right now and a
    ///   writer might try to help out.
    /// * A replacement pointer, tagged with REPLACEMENT_TAG. This pointer points to an Handover,
    ///   containing an already protected value, provided by the writer for the benefit of the
    ///   reader. The reader should abort its own debt and use this instead. This indirection
    ///   (storing pointer to the envelope with the actual pointer) is to make sure there's a space
    ///   for the tag ‒ there is no guarantee the real pointer is aligned to at least 4 bytes, we
    ///   can however force that for the Handover type.
    control: AtomicUsize,
    /// A possibly active debt.
    slot: Debt,
    /// If there's a generation in control, this signifies what address the reader is trying to
    /// load from.
    active_addr: AtomicUsize,
    /// A place where a writer can put a replacement value.
    ///
    /// Note that this is simply an allocation, and every participating slot contributes one, but
    /// they may be passed around through the lifetime of the program. It is not accessed directly,
    /// but through the space_offer thing.
    ///
    handover: Handover,
    /// A pointer to a handover envelope this node currently owns.
    ///
    /// A writer makes a switch of its and readers handover when successfully storing a replacement
    /// in the control.
    space_offer: AtomicPtr<Handover>,
}

impl Default for Slots {
    fn default() -> Self {
        Slots {
            control: AtomicUsize::new(IDLE),
            slot: Debt::default(),
            // Doesn't matter yet
            active_addr: AtomicUsize::new(0),
            // Also doesn't matter
            handover: Handover::default(),
            // Here we would like it to point to our handover. But for that we need to be in place
            // in RAM (effectively pinned, though we use older Rust than Pin, possibly?), so not
            // yet. See init().
            space_offer: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

impl Slots {
    pub(super) fn slot(&self) -> &Debt {
        &self.slot
    }

    pub(super) fn get_debt(&self, ptr: usize, local: &Local) -> (usize, bool) {
        // Incrementing by 4 ensures we always have enough space for 2 bit of tags.
        let gen = local.generation.get().wrapping_add(4);
        debug_assert_eq!(gen & GEN_TAG, 0);
        local.generation.set(gen);
        // Signal the caller that the node should be sent to a cooldown.
        let discard = gen == 0;
        let gen = gen | GEN_TAG;
        // We will sync by the write to the control. But we also sync the value of the previous
        // generation/released slot. That way we may re-confirm in the writer that the reader is
        // not in between here and the compare_exchange below with a stale gen (eg. if we are in
        // here, the re-confirm there will load the NO_DEPT and we are fine).
        self.active_addr.store(ptr, SeqCst);

        // We are the only ones allowed to do the IDLE -> * transition and we never leave it in
        // anything else after an transaction, so this is OK. But we still need a load-store SeqCst
        // operation here to form a relation between this and the store of the actual pointer in
        // the writer thread :-(.
        let prev = self.control.swap(gen, SeqCst);
        debug_assert_eq!(IDLE, prev, "Left control in wrong state");

        (gen, discard)
    }

    pub(super) fn help<R, T>(&self, who: &Self, storage_addr: usize, replacement: &R)
    where
        T: RefCnt,
        R: Fn() -> T,
    {
        debug_assert_eq!(IDLE, self.control.load(Relaxed));
        // Also acquires the auxiliary data in other variables.
        let mut control = who.control.load(SeqCst);
        loop {
            match control & TAG_MASK {
                // Nothing to help with
                IDLE if control == IDLE => break,
                // Someone has already helped out with that, so we have nothing to do here
                REPLACEMENT_TAG => break,
                // Something is going on, let's have a better look.
                GEN_TAG => {
                    debug_assert!(
                        !ptr::eq(self, who),
                        "Refusing to help myself, makes no sense"
                    );
                    // Get the address that other thread is trying to load from. By that acquire,
                    // we also sync the control into our thread once more and reconfirm that the
                    // value of the active_addr is in between two same instances, therefore up to
                    // date to it.
                    let active_addr = who.active_addr.load(SeqCst);
                    if active_addr != storage_addr {
                        // Acquire for the same reason as on the top.
                        let new_control = who.control.load(SeqCst);
                        if new_control == control {
                            // The other thread is doing something, but to some other ArcSwap, so
                            // we don't care. Cool, done.
                            break;
                        } else {
                            // The control just changed under our hands, we don't know what to
                            // trust, so retry.
                            control = new_control;
                            continue;
                        }
                    }

                    // Now we know this work is for us. Try to create a replacement and offer it.
                    // This actually does a full-featured load under the hood, but we are currently
                    // idle and the load doesn't re-enter write, so that's all fine.
                    let replacement = replacement();
                    let replace_addr = T::as_ptr(&replacement) as usize;
                    // If we succeed in helping the other thread, we take their empty space in
                    // return for us that we pass to them. It's already there, the value is synced
                    // to us by Acquire on control.
                    let their_space = who.space_offer.load(SeqCst);
                    // Relaxed is fine, our own thread and nobody but us writes in here.
                    let my_space = self.space_offer.load(SeqCst);
                    // Relaxed is fine, we'll sync by the next compare-exchange. If we don't, the
                    // value won't ever be read anyway.
                    unsafe {
                        (*my_space).0.store(replace_addr, SeqCst);
                    }
                    // Ensured by the align annotation at the type.
                    assert_eq!(my_space as usize & TAG_MASK, 0);
                    let space_addr = (my_space as usize) | REPLACEMENT_TAG;
                    // Acquire on failure -> same reason as at the top, reading the value.
                    // Release on success -> we send data to that thread through here. Must be
                    // AcqRel, because success must be superset of failure. Also, load to get their
                    // space (it won't have changed, it does when the control is set to IDLE).
                    match who
                        .control
                        .compare_exchange(control, space_addr, SeqCst, SeqCst)
                    {
                        Ok(_) => {
                            // We have successfully sent our replacement out (Release) and got
                            // their space in return (Acquire on that load above).
                            self.space_offer.store(their_space, SeqCst);
                            // The ref count went with it, so forget about it here.
                            T::into_ptr(replacement);
                            // We have successfully helped out, so we are done.
                            break;
                        }
                        Err(new_control) => {
                            // Something has changed in between. Let's try again, nothing changed
                            // (the replacement will get dropped at the end of scope, we didn't do
                            // anything with the spaces, etc.
                            control = new_control;
                        }
                    }
                }
                _ => unreachable!("Invalid control value {:X}", control),
            }
        }
    }

    pub(super) fn init(&mut self) {
        *self.space_offer.get_mut() = &mut self.handover;
    }

    pub(super) fn confirm(&self, gen: usize, ptr: usize) -> Result<(), usize> {
        // Put the slot there and consider it acquire of a „lock“. For that we need swap, not store
        // only (we need Acquire and Acquire works only on loads). Release is to make sure control
        // is observable by the other thread (but that's probably not necessary anyway?)
        let prev = self.slot.0.swap(ptr, SeqCst);
        debug_assert_eq!(Debt::NONE, prev);

        // Confirm by writing to the control (or discover that we got helped). We stop anyone else
        // from helping by setting it to IDLE.
        let control = self.control.swap(IDLE, SeqCst);
        if control == gen {
            // Nobody interfered, we have our debt in place and can proceed.
            Ok(())
        } else {
            // Someone put a replacement in there.
            debug_assert_eq!(control & TAG_MASK, REPLACEMENT_TAG);
            let handover = (control & !TAG_MASK) as *mut Handover;
            let replacement = unsafe { &*handover }.0.load(SeqCst);
            // Make sure we advertise the right envelope when we set it to generation next time.
            self.space_offer.store(handover, SeqCst);
            // Note we've left the debt in place. The caller should pay it back (without ever
            // taking advantage of it) to make sure any extra is actually dropped (it is possible
            // someone provided the replacement *and* paid the debt and we need just one of them).
            Err(replacement)
        }
    }
}
