use std::mem::offset_of;
use std::ops::Deref;
use std::ptr::{self, NonNull, addr_of};
use std::sync::atomic::Ordering::{self, Relaxed};

use super::collectible::{Collectible, Link};
use super::collector::Collector;

/// [`RefCounted`] stores an instance of type `T`, and a union of a link to the next
/// [`Collectible`] or the reference counter.
pub(super) struct RefCounted<T> {
    instance: T,
    next_or_refcnt: Link,
}

impl<T> RefCounted<T> {
    /// Creates a new [`RefCounted`] that allows ownership sharing.
    #[inline]
    pub(super) fn new_shared<F: FnOnce() -> T>(f: F) -> NonNull<RefCounted<T>> {
        // LLVM does not ensure in-place initialization: https://godbolt.org/z/GGar3oThh.
        let mut boxed = Box::new_uninit();
        boxed.write(Self {
            instance: f(),
            next_or_refcnt: Link::new_shared(),
        });
        unsafe { NonNull::new_unchecked(Box::into_raw(boxed.assume_init())) }
    }

    /// Creates a new [`RefCounted`] that disallows reference counting.
    ///
    /// The reference counter field is never used until the instance is retired.
    #[inline]
    pub(super) fn new_unique<F: FnOnce() -> T>(f: F) -> NonNull<RefCounted<T>> {
        let mut boxed = Box::new_uninit();
        boxed.write(Self {
            instance: f(),
            next_or_refcnt: Link::new_unique(),
        });
        unsafe { NonNull::new_unchecked(Box::into_raw(boxed.assume_init())) }
    }

    /// Tries to add a strong reference to the underlying instance.
    ///
    /// `order` must be as strong as `Acquire` for the caller to correctly validate the newest
    /// state of the pointer.
    #[inline]
    pub(super) fn try_add_ref(&self, order: Ordering) -> bool {
        self.next_or_refcnt
            .ref_cnt()
            .fetch_update(order, order, |r| {
                if r.addr() & 1 == 1 {
                    Some(r.map_addr(|addr| addr + 2))
                } else {
                    None
                }
            })
            .is_ok()
    }

    /// Returns a mutable reference to the instance if the number of owners is `1`.
    #[inline]
    pub(super) fn get_mut_shared(&mut self) -> Option<&mut T> {
        if self.next_or_refcnt.ref_cnt().load(Relaxed).addr() == 1 {
            Some(&mut self.instance)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the instance if it is uniquely owned.
    #[inline]
    pub(super) const fn get_mut_unique(&mut self) -> &mut T {
        &mut self.instance
    }

    /// Adds a strong reference to the underlying instance.
    #[inline]
    pub(super) fn add_ref(&self) {
        let mut current = self.next_or_refcnt.ref_cnt().load(Relaxed);
        loop {
            debug_assert_eq!(current.addr() & 1, 1);
            debug_assert!(current.addr() <= usize::MAX - 2, "reference count overflow");
            match self.next_or_refcnt.ref_cnt().compare_exchange_weak(
                current,
                current.map_addr(|addr| addr + 2),
                Relaxed,
                Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => {
                    current = actual;
                }
            }
        }
    }

    /// Drops a strong reference to the underlying instance.
    ///
    /// Returns `true` if it the last reference was dropped.
    #[inline]
    pub(super) fn drop_ref(&self) -> bool {
        // It does not have to be a load-acquire as everything's synchronized via the global
        // epoch.
        let mut current = self.next_or_refcnt.ref_cnt().load(Relaxed);
        loop {
            debug_assert_ne!(current.addr(), 0);
            match self.next_or_refcnt.ref_cnt().compare_exchange_weak(
                current,
                current.map_addr(|addr| addr.saturating_sub(2)),
                Relaxed,
                Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => {
                    current = actual;
                }
            }
        }
        current.addr() == 1
    }

    /// Returns a pointer to the instance.
    #[inline]
    pub(super) const fn inst_ptr(self_ptr: *const Self) -> *const T {
        if self_ptr.is_null() {
            ptr::null()
        } else {
            unsafe { addr_of!((*self_ptr).instance) }
        }
    }

    /// Returns a non-null pointer to the instance.
    #[inline]
    pub(super) const fn inst_non_null_ptr(self_ptr: NonNull<Self>) -> NonNull<T> {
        let offset = offset_of!(Self, instance);
        unsafe { self_ptr.cast::<u8>().add(offset).cast::<T>() }
    }

    /// Passes a pointer to [`RefCounted`] to the garbage collector.
    #[inline]
    pub(super) fn pass_to_collector(ptr: *mut Self) {
        // The lifetime of the pointer is extended to `'static`; it is not illegal since the safe
        // public API does not permit to create a `RefCounted` of a non `'static` type, and those
        // that allow to do so are all unsafe methods (e.g., `{Owned, Shard}::new_unchecked`) where
        // the doc explicitly states that the pointed-to value may be dropped at an arbitrary time.
        #[allow(clippy::transmute_ptr_to_ptr)]
        let ptr = unsafe {
            std::mem::transmute::<*mut (dyn Collectible + '_), *mut (dyn Collectible + 'static)>(
                ptr,
            )
        };
        Collector::collect(Collector::current(), ptr);
    }
}

impl<T> Deref for RefCounted<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl<T> Collectible for RefCounted<T> {
    #[inline]
    fn next_ptr(&self) -> Option<NonNull<dyn Collectible>> {
        self.next_or_refcnt.next_ptr()
    }

    #[inline]
    fn set_next_ptr(&self, next_ptr: Option<NonNull<dyn Collectible>>) {
        self.next_or_refcnt.set_next_ptr(next_ptr);
    }
}
