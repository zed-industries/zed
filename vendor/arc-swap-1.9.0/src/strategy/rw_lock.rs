use core::sync::atomic::{AtomicPtr, Ordering};

use std::sync::RwLock;

use super::sealed::{CaS, InnerStrategy, Protected};
use crate::as_raw::AsRaw;
use crate::ref_cnt::RefCnt;

impl<T: RefCnt> Protected<T> for T {
    #[inline]
    fn from_inner(ptr: T) -> Self {
        ptr
    }

    #[inline]
    fn into_inner(self) -> T {
        self
    }
}

impl<T: RefCnt> InnerStrategy<T> for RwLock<()> {
    type Protected = T;
    unsafe fn load(&self, storage: &AtomicPtr<T::Base>) -> T {
        let _guard = self.read().expect("We don't panic in here");
        let ptr = storage.load(Ordering::Acquire);
        let ptr = T::from_ptr(ptr as *const T::Base);
        T::inc(&ptr);

        ptr
    }

    unsafe fn wait_for_readers(&self, _: *const T::Base, _: &AtomicPtr<T::Base>) {
        // By acquiring the write lock, we make sure there are no read locks present across it.
        drop(self.write().expect("We don't panic in here"));
    }
}

impl<T: RefCnt> CaS<T> for RwLock<()> {
    unsafe fn compare_and_swap<C: AsRaw<T::Base>>(
        &self,
        storage: &AtomicPtr<T::Base>,
        current: C,
        new: T,
    ) -> Self::Protected {
        let _lock = self.write();
        let cur = current.as_raw();
        let new = T::into_ptr(new);
        let swapped = storage.compare_exchange(cur, new, Ordering::AcqRel, Ordering::Relaxed);
        let old = match swapped {
            Ok(old) => old,
            Err(old) => old,
        };
        let old = T::from_ptr(old as *const T::Base);
        if swapped.is_err() {
            // If the new didn't go in, we need to destroy it and increment count in the old that
            // we just duplicated
            T::inc(&old);
            drop(T::from_ptr(new));
        }
        drop(current);
        old
    }
}
