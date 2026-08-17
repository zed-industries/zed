use std::cell::UnsafeCell;
use std::ptr::{self, NonNull, addr_of_mut};
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};
#[cfg(not(feature = "loom"))]
use std::sync::atomic::fence;
use std::sync::atomic::{AtomicPtr, AtomicU8};

#[cfg(feature = "loom")]
use loom::sync::atomic::fence;

use super::collectible::{Collectible, Link};
use super::exit_guard::ExitGuard;
use super::{Epoch, Tag};

/// [`Collector`] is a garbage collector that reclaims thread-locally unreachable instances
/// when they are globally unreachable.
#[derive(Default)]
#[repr(align(128))]
pub(super) struct Collector {
    state: AtomicU8,
    announcement: Epoch,
    next_epoch_update: u8,
    has_garbage: bool,
    num_readers: u32,
    previous_instance_link: Option<NonNull<dyn Collectible>>,
    current_instance_link: Option<NonNull<dyn Collectible>>,
    next_instance_link: Option<NonNull<dyn Collectible>>,
    next_link: AtomicPtr<Collector>,
    link: Link,
}

/// Data stored in a [`CollectorRoot`] is shared among [`Collector`] instances.
#[derive(Default)]
pub(super) struct CollectorRoot {
    epoch: AtomicU8,
    chain_head: AtomicPtr<Collector>,
}

/// [`CollectorAnchor`] helps allocate and cleanup the thread-local [`Collector`].
struct CollectorAnchor;

impl Collector {
    /// The number of quiescent states before an epoch update is triggered.
    const CADENCE: u8 = 1_u8 << 7;

    /// Represents a quiescent state.
    const INACTIVE: u8 = Epoch::NUM_EPOCHS;

    /// Represents a terminated thread state.
    const INVALID: u8 = Epoch::NUM_EPOCHS << 1;

    #[inline]
    /// Accelerates garbage collection.
    pub(super) const fn accelerate(collector_ptr: NonNull<Collector>) {
        unsafe {
            (*collector_ptr.as_ptr()).next_epoch_update = 0;
        }
    }

    /// Returns the [`Collector`] attached to the current thread.
    #[inline]
    pub(super) fn current() -> NonNull<Collector> {
        LOCAL_COLLECTOR.with(|local_collector| {
            let local_collector_ptr = local_collector.get();
            unsafe {
                NonNull::new(*local_collector_ptr).unwrap_or_else(|| {
                    let collector_ptr = COLLECTOR_ANCHOR.with(CollectorAnchor::alloc);
                    (*local_collector_ptr) = collector_ptr.as_ptr();
                    collector_ptr
                })
            }
        })
    }

    /// Acknowledges a new [`Guard`](super::Guard) being instantiated.
    ///
    /// # Panics
    ///
    /// The method may panic if the number of readers has reached `u32::MAX`.
    #[inline]
    pub(super) fn new_guard(collector_ptr: NonNull<Collector>) {
        unsafe {
            if (*collector_ptr.as_ptr()).num_readers == 0 {
                debug_assert_eq!(
                    (*collector_ptr.as_ptr()).state.load(Relaxed) & Self::INACTIVE,
                    Self::INACTIVE
                );
                (*collector_ptr.as_ptr()).num_readers = 1;

                // The epoch value can be any number between the last time a guard was created in
                // the thread and the most recent value of `GLOBAL_TOOR.epoch`.
                let new_epoch = Epoch::from_u8(GLOBAL_ROOT.epoch.load(Relaxed));

                // Every epoch update, pointer loading, memory retirement, and memory
                // reclamation event is always placed between a pair `SeqCst` memory barrier events
                // (one in this method, and the other one in `scan`) where those `SeqCst` memory
                // barriers are globally ordered by definition. This property ensures that a retired
                // memory region cannot be reclaimed until any threads holding a pointer to the
                // region turn inactive, because, the reclaimer needs to wait for at least two
                // `SeqCst` barrier events in `scan` to reclaim the memory region, and the fact that
                // the other threads were able to load a valid pointer means that the thread was in
                // between the same `SeqCst` barrier event pair or an older one; if the former, one
                // of the two `scan` events must have observed that the thread was active (this
                // cannot be achieved by `Release-Acquire` relationships), preventing the global
                // epoch from advancing more than once; if the latter, trivial.
                if cfg!(feature = "loom")
                    || cfg!(miri)
                    || cfg!(not(any(target_arch = "x86", target_arch = "x86_64")))
                {
                    // What will happen after the fence strictly happens after the fence.
                    (*collector_ptr.as_ptr())
                        .state
                        .store(new_epoch.into(), Relaxed);
                    fence(SeqCst);
                } else {
                    // This special optimization is excerpted from
                    // [`crossbeam_epoch`](https://docs.rs/crossbeam-epoch/).
                    //
                    // The rationale behind the code is, it compiles to `lock xchg` that
                    // practically acts as a full memory barrier on `X86`, and is much faster than
                    // `mfence`.
                    (*collector_ptr.as_ptr())
                        .state
                        .swap(new_epoch.into(), SeqCst);
                }
                if (*collector_ptr.as_ptr()).announcement != new_epoch {
                    (*collector_ptr.as_ptr()).announcement = new_epoch;
                    let exit_guard = ExitGuard::new((), |()| {
                        Self::end_guard(collector_ptr);
                    });
                    Collector::epoch_updated(collector_ptr);
                    exit_guard.forget();
                }
            } else {
                debug_assert_eq!(
                    (*collector_ptr.as_ptr()).state.load(Relaxed) & Self::INACTIVE,
                    0
                );
                assert_ne!(
                    (*collector_ptr.as_ptr()).num_readers,
                    u32::MAX,
                    "Too many EBR guards"
                );
                (*collector_ptr.as_ptr()).num_readers += 1;
            }
        }
    }

    /// Acknowledges an existing [`Guard`](super::Guard) being dropped.
    #[inline]
    pub(super) fn end_guard(collector_ptr: NonNull<Collector>) {
        unsafe {
            debug_assert_eq!(
                (*collector_ptr.as_ptr()).state.load(Relaxed) & Self::INACTIVE,
                0
            );
            debug_assert_eq!(
                (*collector_ptr.as_ptr()).state.load(Relaxed),
                u8::from((*collector_ptr.as_ptr()).announcement)
            );

            if (*collector_ptr.as_ptr()).num_readers == 1 {
                (*collector_ptr.as_ptr()).num_readers = 0;
                if (*collector_ptr.as_ptr()).next_epoch_update == 0 {
                    Collector::scan(collector_ptr);
                    (*collector_ptr.as_ptr()).next_epoch_update = Self::CADENCE;
                } else if (*collector_ptr.as_ptr()).has_garbage
                    || Tag::into_tag(GLOBAL_ROOT.chain_head.load(Relaxed)) == Tag::Second
                {
                    (*collector_ptr.as_ptr()).next_epoch_update -= 1;
                }

                // `Release` is needed to prevent any previous load operations in this thread from
                // passing through.
                (*collector_ptr.as_ptr()).state.store(
                    u8::from((*collector_ptr.as_ptr()).announcement) | Self::INACTIVE,
                    Release,
                );
            } else {
                (*collector_ptr.as_ptr()).num_readers -= 1;
            }
        }
    }

    /// Returns the current epoch.
    #[inline]
    pub(super) fn current_epoch() -> Epoch {
        // It is called by an active `Guard` therefore it is after a `SeqCst` memory barrier. Each
        // epoch update is preceded by another `SeqCst` memory barrier, therefore those two events
        // are globally ordered. If the `SeqCst` event during the `Guard` creation happened before
        // the other `SeqCst` event, this will either load the last previous epoch value, or the
        // current value. If not, it is guaranteed that it reads the latest global epoch value.
        //
        // It is not possible to return the announced epoch here since the global epoch value is
        // rotated and the announced epoch may be outdated; this may lead to a situation where the
        // caller thinks that a new generation has been witnessed.
        Epoch::from_u8(GLOBAL_ROOT.epoch.load(Relaxed))
    }

    /// Returns `true` if the [`Collector`] has garbage.
    #[inline]
    pub(super) const fn has_garbage(collector_ptr: NonNull<Collector>) -> bool {
        unsafe { (*collector_ptr.as_ptr()).has_garbage }
    }

    /// Sets the garbage flag to allow this thread to advance the global epoch.
    #[inline]
    pub(super) const fn set_has_garbage(collector_ptr: NonNull<Collector>) {
        unsafe {
            (*collector_ptr.as_ptr()).has_garbage = true;
        }
    }

    /// Collects garbage instances.
    #[inline]
    pub(super) fn collect(collector_ptr: NonNull<Collector>, instance_ptr: *mut dyn Collectible) {
        unsafe {
            (*instance_ptr).set_next_ptr((*collector_ptr.as_ptr()).current_instance_link.take());
            (*collector_ptr.as_ptr()).current_instance_link = NonNull::new(instance_ptr);
            (*collector_ptr.as_ptr()).has_garbage = true;
        }
    }

    /// Passes its garbage instances to other threads.
    #[inline]
    pub(super) fn pass_garbage() -> bool {
        LOCAL_COLLECTOR.with(|local_collector| {
            let local_collector_ptr = local_collector.get();
            let collector_ptr = unsafe { *local_collector_ptr };
            if collector_ptr.is_null() {
                return true;
            }
            let collector = unsafe { &*collector_ptr };
            if collector.num_readers != 0 {
                return false;
            }
            if collector.has_garbage {
                // In case the thread state is marked `Invalid`, a `Release` guard is required since
                // any remaining garbage may be reclaimed by other threads.
                collector.state.fetch_or(Collector::INVALID, Release);
                unsafe {
                    *local_collector_ptr = ptr::null_mut();
                }
                mark_scan_enforced();
            }
            true
        })
    }

    /// Allocates a new [`Collector`].
    fn alloc() -> NonNull<Collector> {
        let mut boxed = Box::new(Collector::default());
        boxed.state.store(Self::INACTIVE, Relaxed);
        boxed.next_epoch_update = Self::CADENCE;

        let ptr = Box::into_raw(boxed);
        let mut current = GLOBAL_ROOT.chain_head.load(Relaxed);
        loop {
            unsafe {
                (*ptr)
                    .next_link
                    .store(Tag::unset_tag(current).cast_mut(), Relaxed);
            }

            // It keeps the tag intact.
            let tag = Tag::into_tag(current);
            let new = Tag::update_tag(ptr, tag).cast_mut();
            if let Err(actual) = GLOBAL_ROOT
                .chain_head
                .compare_exchange_weak(current, new, Release, Relaxed)
            {
                current = actual;
            } else {
                break;
            }
        }
        unsafe { NonNull::new_unchecked(ptr) }
    }

    /// Acknowledges a new global epoch.
    fn epoch_updated(collector_ptr: NonNull<Collector>) {
        unsafe {
            debug_assert_eq!(
                (*collector_ptr.as_ptr()).state.load(Relaxed) & Self::INACTIVE,
                0
            );
            debug_assert_eq!(
                (*collector_ptr.as_ptr()).state.load(Relaxed),
                u8::from((*collector_ptr.as_ptr()).announcement)
            );
            if (*collector_ptr.as_ptr()).has_garbage {
                let mut garbage_link = (*collector_ptr.as_ptr()).next_instance_link.take();
                (*collector_ptr.as_ptr()).next_instance_link =
                    (*collector_ptr.as_ptr()).previous_instance_link.take();
                (*collector_ptr.as_ptr()).previous_instance_link =
                    (*collector_ptr.as_ptr()).current_instance_link.take();
                (*collector_ptr.as_ptr()).has_garbage =
                    (*collector_ptr.as_ptr()).next_instance_link.is_some()
                        || (*collector_ptr.as_ptr()).previous_instance_link.is_some();
                while let Some(instance_ptr) = garbage_link.take() {
                    garbage_link = (*instance_ptr.as_ptr()).next_ptr();
                    let mut guard = ExitGuard::new(garbage_link, |mut garbage_link| {
                        while let Some(instance_ptr) = garbage_link.take() {
                            // Something went wrong during dropping and deallocating an instance.
                            garbage_link = (*instance_ptr.as_ptr()).next_ptr();

                            // Previous `drop_and_dealloc` may have accessed `self.current_instance_link`.
                            std::sync::atomic::compiler_fence(Acquire);
                            Collector::collect(collector_ptr, instance_ptr.as_ptr());
                        }
                    });

                    // The `drop` below may access `self.current_instance_link`.
                    std::sync::atomic::compiler_fence(Acquire);
                    drop(Box::from_raw(instance_ptr.as_ptr()));
                    garbage_link = guard.take();
                }
            }
            (*collector_ptr.as_ptr()).next_epoch_update = Self::CADENCE;
        }
    }

    /// Clears all the garbage instances for dropping the [`Collector`].
    fn clear_for_drop(collector_ptr: *mut Collector) {
        unsafe {
            loop {
                let garbage_containers = [
                    (*collector_ptr).previous_instance_link.take(),
                    (*collector_ptr).current_instance_link.take(),
                    (*collector_ptr).next_instance_link.take(),
                ];
                if !garbage_containers.iter().any(Option::is_some) {
                    break;
                }
                for mut link in garbage_containers {
                    while let Some(instance_ptr) = link {
                        link = (*instance_ptr.as_ptr()).next_ptr();
                        drop(Box::from_raw(instance_ptr.as_ptr()));
                    }
                }
            }
        }
    }

    /// Scans the [`Collector`] instances to update the global epoch.
    fn scan(collector_ptr: NonNull<Collector>) {
        unsafe {
            debug_assert_eq!(
                (*collector_ptr.as_ptr()).state.load(Relaxed) & Self::INVALID,
                0
            );

            if u8::from((*collector_ptr.as_ptr()).announcement) != GLOBAL_ROOT.epoch.load(Relaxed) {
                // No need for further processing if the announcement is not up-to-date.
                return;
            }

            // Only one thread that acquires the chain lock is allowed to scan the thread-local
            // collectors.
            let lock_result = Self::lock_chain();
            let Ok(mut current_collector_ptr) = lock_result else {
                return;
            };
            let _guard = ExitGuard::new((), |()| Self::unlock_chain());

            let known_epoch = (*collector_ptr.as_ptr()).state.load(Relaxed);
            let mut prev_collector_ptr: *mut Collector = ptr::null_mut();
            while !current_collector_ptr.is_null() {
                if ptr::eq(collector_ptr.as_ptr(), current_collector_ptr) {
                    prev_collector_ptr = current_collector_ptr;
                    current_collector_ptr = (*collector_ptr.as_ptr()).next_link.load(Acquire);
                    continue;
                }

                // `Acquire` is needed in case the other thread is inactive so that this thread
                // needs to reclaim memory for the thread.
                let collector_state = (*current_collector_ptr).state.load(Acquire);
                let next_collector_ptr = (*current_collector_ptr).next_link.load(Acquire);
                if (collector_state & Self::INVALID) != 0 {
                    // The collector is obsolete.
                    let result = if prev_collector_ptr.is_null() {
                        GLOBAL_ROOT
                            .chain_head
                            .fetch_update(Release, Relaxed, |p| {
                                let tag = Tag::into_tag(p);
                                debug_assert!(tag == Tag::First || tag == Tag::Both);
                                if ptr::eq(Tag::unset_tag(p), current_collector_ptr) {
                                    Some(Tag::update_tag(next_collector_ptr, tag).cast_mut())
                                } else {
                                    None
                                }
                            })
                            .is_ok()
                    } else {
                        (*prev_collector_ptr)
                            .next_link
                            .store(next_collector_ptr, Release);
                        true
                    };
                    if result {
                        Self::collect(collector_ptr, current_collector_ptr);
                        current_collector_ptr = next_collector_ptr;
                        continue;
                    }
                } else if (collector_state & Self::INACTIVE) == 0 && collector_state != known_epoch
                {
                    // Not ready for an epoch update.
                    return;
                }
                prev_collector_ptr = current_collector_ptr;
                current_collector_ptr = next_collector_ptr;
            }

            // A memory region can be retired after a `SeqCst` barrier in a `Guard`, and the memory
            // region can only be deallocated after the thread has observed three times of epoch
            // updates. This `SeqCst` fence ensures that the epoch update is strictly sequenced
            // after/before a `Guard`, enabling the event of the retirement of the memory region is
            // also globally ordered with epoch updates.
            fence(SeqCst);
            GLOBAL_ROOT
                .epoch
                .store(Epoch::from_u8(known_epoch).next().into(), Relaxed);
        }
    }

    /// Clears the [`Collector`] chain to if all are invalid.
    fn clear_chain() -> bool {
        let lock_result = Self::lock_chain();
        if let Ok(collector_head) = lock_result {
            let _guard = ExitGuard::new((), |()| Self::unlock_chain());
            unsafe {
                let mut current_collector_ptr = collector_head;
                while !current_collector_ptr.is_null() {
                    if ((*current_collector_ptr).state.load(Acquire) & Self::INVALID) == 0 {
                        return false;
                    }
                    current_collector_ptr = (*current_collector_ptr).next_link.load(Acquire);
                }

                // Reaching here means that there is no `Ptr` that possibly sees any garbage instances
                // in those `Collector` instances in the chain.
                let result = GLOBAL_ROOT.chain_head.fetch_update(Release, Relaxed, |p| {
                    if Tag::unset_tag(p) == collector_head {
                        let tag = Tag::into_tag(p);
                        debug_assert!(tag == Tag::First || tag == Tag::Both);
                        Some(Tag::update_tag(ptr::null::<Collector>(), tag).cast_mut())
                    } else {
                        None
                    }
                });

                if result.is_ok() {
                    let mut current_collector_ptr = collector_head;
                    while !current_collector_ptr.is_null() {
                        let next_collector_ptr = (*current_collector_ptr).next_link.load(Acquire);
                        drop(Box::from_raw(current_collector_ptr));
                        current_collector_ptr = next_collector_ptr;
                    }
                    return true;
                }
            }
        }
        false
    }

    /// Locks the chain.
    fn lock_chain() -> Result<*mut Collector, *mut Collector> {
        GLOBAL_ROOT
            .chain_head
            .fetch_update(Acquire, Acquire, |p| {
                let tag = Tag::into_tag(p);
                if tag == Tag::First || tag == Tag::Both {
                    None
                } else {
                    Some(Tag::update_tag(p, Tag::First).cast_mut())
                }
            })
            .map(|p| Tag::unset_tag(p).cast_mut())
    }

    /// Unlocks the chain.
    fn unlock_chain() {
        loop {
            let result = GLOBAL_ROOT.chain_head.fetch_update(Release, Relaxed, |p| {
                let tag = Tag::into_tag(p);
                debug_assert!(tag == Tag::First || tag == Tag::Both);
                let new_tag = if tag == Tag::First {
                    Tag::None
                } else {
                    // Retain the mark.
                    Tag::Second
                };
                Some(Tag::update_tag(p, new_tag).cast_mut())
            });
            if result.is_ok() {
                break;
            }
        }
    }
}

impl Drop for Collector {
    #[inline]
    fn drop(&mut self) {
        let collector_ptr = addr_of_mut!(*self);
        Self::clear_for_drop(collector_ptr);
    }
}

impl Collectible for Collector {
    #[inline]
    fn next_ptr(&self) -> Option<NonNull<dyn Collectible>> {
        self.link.next_ptr()
    }

    #[inline]
    fn set_next_ptr(&self, next_ptr: Option<NonNull<dyn Collectible>>) {
        self.link.set_next_ptr(next_ptr);
    }
}

impl CollectorAnchor {
    fn alloc(&self) -> NonNull<Collector> {
        let _: &CollectorAnchor = self;
        Collector::alloc()
    }
}

impl Drop for CollectorAnchor {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // `LOCAL_COLLECTOR` is the last thread-local variable to be dropped.
            LOCAL_COLLECTOR.with(|local_collector| {
                let local_collector_ptr = local_collector.get();
                let collector_ptr = *local_collector_ptr;
                if !collector_ptr.is_null() {
                    (*collector_ptr).state.fetch_or(Collector::INVALID, Release);
                }

                let mut temp_collector = Collector::default();
                temp_collector.state.store(Collector::INACTIVE, Relaxed);
                *local_collector_ptr = addr_of_mut!(temp_collector);
                if !Collector::clear_chain() {
                    mark_scan_enforced();
                }

                Collector::clear_for_drop(addr_of_mut!(temp_collector));
                *local_collector_ptr = ptr::null_mut();
            });
        }
    }
}

/// Marks the head of a chain to indicate that there is a potentially unreachable `Collector` in the
/// chain.
fn mark_scan_enforced() {
    // `Tag::Second` indicates that there is a garbage `Collector`.
    let _result = GLOBAL_ROOT.chain_head.fetch_update(Release, Relaxed, |p| {
        let new_tag = match Tag::into_tag(p) {
            Tag::None => Tag::Second,
            Tag::First => Tag::Both,
            Tag::Second | Tag::Both => return None,
        };
        Some(Tag::update_tag(p, new_tag).cast_mut())
    });
}

thread_local! {
    static LOCAL_COLLECTOR: UnsafeCell<*mut Collector> = const { UnsafeCell::new(ptr::null_mut()) };
    static COLLECTOR_ANCHOR: CollectorAnchor = const { CollectorAnchor };
}

/// The global and default [`CollectorRoot`].
static GLOBAL_ROOT: CollectorRoot = CollectorRoot {
    epoch: AtomicU8::new(0),
    chain_head: AtomicPtr::new(ptr::null_mut()),
};
