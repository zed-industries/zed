use core::{
    cell::UnsafeCell,
    marker::{PhantomData, PhantomPinned},
};

/// An opaque type.
///
/// This is used to avoid problems with e.g. getting references from
/// `CFArray` (we can't use `c_void` as the default type, as `&c_void` would
/// be incorrect).
#[repr(C)]
#[doc(hidden)]
#[allow(dead_code, unreachable_pub)]
pub struct Opaque {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}
