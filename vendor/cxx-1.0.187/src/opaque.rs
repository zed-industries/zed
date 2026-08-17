#![allow(missing_docs)]

use crate::void;
use core::cell::UnsafeCell;
use core::marker::{PhantomData, PhantomPinned};
use core::mem;
use core::panic::RefUnwindSafe;

// . size = 0
// . align = 1
// . ffi-safe
// . !Send
// . !Sync
// . !Unpin
// . not readonly
// . unwind-safe
#[repr(C, packed)]
pub struct Opaque {
    _private: [*const void; 0],
    _pinned: PhantomData<PhantomPinned>,
    _mutable: SyncUnsafeCell<PhantomData<()>>,
}

impl RefUnwindSafe for Opaque {}

// TODO: https://github.com/rust-lang/rust/issues/95439
#[repr(transparent)]
struct SyncUnsafeCell<T>(UnsafeCell<T>);

unsafe impl<T> Sync for SyncUnsafeCell<T> {}

const_assert_eq!(0, mem::size_of::<Opaque>());
const_assert_eq!(1, mem::align_of::<Opaque>());
