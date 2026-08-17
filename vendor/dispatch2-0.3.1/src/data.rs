#![allow(unused_imports)]
use core::ptr::{self, NonNull};

use crate::generated::_dispatch_data_empty;
use crate::DispatchRetained;

dispatch_object!(
    /// Dispatch data.
    #[doc(alias = "dispatch_data_t")]
    #[doc(alias = "dispatch_data_s")]
    pub struct DispatchData;
);

impl DispatchData {
    // TODO: Expose this once possible in MSRV.
    // pub const EMPTY: &Self = _dispatch_data_empty;

    /// Get an empty [`DispatchData`].
    #[inline]
    pub fn empty() -> &'static Self {
        // SAFETY: The static is valid.
        unsafe { &_dispatch_data_empty }
    }

    /// Creates a dispatch data object with a copy of the given contiguous
    /// buffer of memory.
    #[cfg(feature = "block2")]
    #[inline]
    pub fn from_bytes(data: &[u8]) -> DispatchRetained<Self> {
        // TODO: Autogenerate?
        const DISPATCH_DATA_DESTRUCTOR_DEFAULT: crate::dispatch_block_t = ptr::null_mut();

        let ptr = NonNull::new(data.as_ptr().cast_mut()).unwrap().cast();

        // We don't care which queue ends up running the destructor.
        let queue = None;

        // SAFETY: Buffer pointer is valid for the given number of bytes.
        //
        // The destructor is DISPATCH_DATA_DESTRUCTOR_DEFAULT, which indicates
        // the buffer should be copied, so it's safe to keep after the end of
        // this function.
        unsafe { Self::new(ptr, data.len(), queue, DISPATCH_DATA_DESTRUCTOR_DEFAULT) }
    }

    /// Creates a dispatch data object with a reference to the given
    /// contiguous buffer of memory.
    #[cfg(feature = "block2")]
    #[inline]
    pub fn from_static_bytes(data: &'static [u8]) -> DispatchRetained<Self> {
        block2::global_block! {
            static NOOP_BLOCK = || {}
        }

        let ptr = NonNull::new(data.as_ptr().cast_mut()).unwrap().cast();

        // We don't care which queue ends up running the destructor.
        let queue = None;

        let destructor = (&*NOOP_BLOCK as *const block2::Block<_>).cast_mut();

        // SAFETY: Buffer pointer is valid for the given number of bytes.
        // Queue handle is valid, and the destructor is a NULL value which
        // indicates the buffer should be copied.
        unsafe { Self::new(ptr, data.len(), queue, destructor) }
    }

    /// Creates a dispatch data object with ownership of the given contiguous
    /// buffer of memory.
    #[cfg(feature = "alloc")]
    #[cfg(feature = "block2")]
    #[inline]
    pub fn from_boxed(data: alloc::boxed::Box<[u8]>) -> DispatchRetained<Self> {
        let data_len = data.len();
        let raw = alloc::boxed::Box::into_raw(data);
        let ptr = NonNull::new(raw).unwrap().cast();

        let destructor = block2::RcBlock::new(move || {
            // SAFETY: The fat pointer (plus size) was retrieved from
            // `Box::into_raw()`, and its ownership was *not* consumed by
            // dispatch_data_create().
            let _ = unsafe { alloc::boxed::Box::<[u8]>::from_raw(raw) };
        });
        let destructor = block2::RcBlock::as_ptr(&destructor);

        // We don't care which queue ends up running the destructor.
        // Box<[u8]> is sendable, so it's fine for us to potentially pass it
        // to a different thread.
        let queue = None;

        // SAFETY: Buffer pointer is valid for the given number of bytes.
        //
        // The destructor is valid and correctly destroys the buffer.
        unsafe { Self::new(ptr, data_len, queue, destructor) }
    }

    /// Copy all the non-contiguous parts of the data into a contiguous
    /// [`Vec`][std::vec::Vec].
    ///
    /// # Examples
    ///
    /// ```
    /// use dispatch2::DispatchData;
    ///
    /// let data = DispatchData::from_bytes(b"foo");
    /// assert_eq!(data.to_vec(), b"foo");
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg(feature = "block2")]
    #[cfg(feature = "objc2")]
    #[inline]
    pub fn to_vec(&self) -> alloc::vec::Vec<u8> {
        let contents = core::cell::RefCell::new(alloc::vec::Vec::new());
        let block = block2::RcBlock::new(
            |_region, _offset, buffer: NonNull<core::ffi::c_void>, size| {
                // SAFETY: Dispatch guarantees that the slice is valid.
                let buffer =
                    unsafe { core::slice::from_raw_parts(buffer.cast::<u8>().as_ptr(), size) };
                contents.borrow_mut().extend_from_slice(buffer);
                1
            },
        );

        let block = block2::RcBlock::as_ptr(&block);
        // SAFETY: Transmute from return type `u8` to `bool` is safe, since we
        // only ever return `1` / `true`.
        // TODO: Fix the need for this in `block2`.
        let block = unsafe {
            core::mem::transmute::<
                *mut block2::Block<
                    dyn Fn(NonNull<DispatchData>, usize, NonNull<core::ffi::c_void>, usize) -> u8,
                >,
                *mut block2::Block<
                    dyn Fn(NonNull<DispatchData>, usize, NonNull<core::ffi::c_void>, usize) -> bool,
                >,
            >(block)
        };

        // SAFETY: The block is implemented correctly.
        unsafe { self.apply(block) };
        contents.take()
    }
}

#[cfg(test)]
#[cfg(feature = "alloc")]
#[cfg(feature = "block2")]
mod tests {
    use core::time::Duration;
    use std::{
        boxed::Box,
        sync::{Arc, Condvar, Mutex},
    };

    use block2::RcBlock;

    use crate::DispatchObject;

    use super::*;

    // Intentionally not Send + Sync, as `DispatchData` can contain things
    // like `NSData`.
    static_assertions::assert_not_impl_any!(DispatchData: Send, Sync);

    #[test]
    fn create() {
        let data = DispatchData::from_bytes(b"foo");
        assert_eq!(data.size(), 3);

        let data = DispatchData::from_static_bytes(b"foo");
        assert_eq!(data.size(), 3);

        let data = DispatchData::from_boxed(Box::from(b"foo" as &[u8]));
        assert_eq!(data.size(), 3);
    }

    #[test]
    fn concat() {
        let data1 = DispatchData::from_bytes(b"foo");
        let data2 = DispatchData::from_boxed(Box::from(b"bar" as &[u8]));
        let extended = data1.concat(&data2).concat(&data2);
        assert_eq!(extended.to_vec(), "foobarbar".as_bytes());
    }

    // Test destruction, and that it still works when we add a finalizer to the data.
    #[test]
    fn with_finalizer() {
        #[derive(Debug)]
        struct State {
            has_run_destructor: bool,
            has_run_finalizer: bool,
        }

        let state = State {
            has_run_destructor: false,
            has_run_finalizer: false,
        };
        let pair = Arc::new((Mutex::new(state), Condvar::new()));

        let pair2 = Arc::clone(&pair);
        let destructor = RcBlock::new(move || {
            let (lock, cvar) = &*pair2;
            lock.lock().unwrap().has_run_destructor = true;
            cvar.notify_one();
        });

        // SAFETY: The pointers are correct.
        let data = unsafe {
            let data = b"xyz";
            DispatchData::new(
                NonNull::new(data.as_ptr().cast_mut()).unwrap().cast(),
                data.len(),
                None,
                RcBlock::as_ptr(&destructor),
            )
        };

        let pair3 = Arc::clone(&pair);
        data.set_finalizer(move || {
            let (lock, cvar) = &*pair3;
            lock.lock().unwrap().has_run_finalizer = true;
            cvar.notify_one();
        });

        let (lock, cvar) = &*pair;
        let lock = lock.lock().unwrap();

        // Verify the destructor hasn't run yet.
        let (lock, res) = cvar.wait_timeout(lock, Duration::from_millis(10)).unwrap();
        assert!(res.timed_out());
        assert!(!lock.has_run_destructor);
        assert!(!lock.has_run_finalizer);

        let data2 = data.clone();
        drop(data);

        // Still not yet, the second reference is still alive.
        let (lock, res) = cvar.wait_timeout(lock, Duration::from_millis(10)).unwrap();
        assert!(res.timed_out());
        assert!(!lock.has_run_destructor);
        assert!(!lock.has_run_finalizer);

        let data3 = data2.concat(&DispatchData::from_bytes(b"foo"));
        drop(data2);

        // Still not yet, the reference is kept alive by the new data.
        let (lock, res) = cvar.wait_timeout(lock, Duration::from_millis(10)).unwrap();
        assert!(res.timed_out());
        assert!(!lock.has_run_destructor);
        assert!(!lock.has_run_finalizer);

        drop(data3);

        // Has run now!
        let (lock, res) = cvar.wait_timeout(lock, Duration::from_millis(10)).unwrap();
        assert!(!res.timed_out());
        assert!(lock.has_run_destructor);
        assert!(lock.has_run_finalizer);
    }
}
