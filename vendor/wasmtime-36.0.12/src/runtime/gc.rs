#[cfg(feature = "gc")]
mod enabled;
#[cfg(feature = "gc")]
pub use enabled::*;

#[cfg(not(feature = "gc"))]
mod disabled;
#[cfg(not(feature = "gc"))]
pub use disabled::*;

mod noextern;
pub use noextern::NoExtern;

mod none_ref;
pub use none_ref::NoneRef;

use core::fmt;
use core::ops::Deref;

/// A common trait implemented by all garbage-collected reference types.
///
/// This is a sealed trait, and may not be implemented for any types outside of
/// the `wasmtime` crate.
pub trait GcRef: GcRefImpl {}

impl<T> GcRef for T where T: GcRefImpl {}

/// A trait implemented for GC references that are guaranteed to be rooted:
///
/// * [`Rooted<T>`][crate::Rooted]
/// * [`ManuallyRooted<T>`][crate::ManuallyRooted]
///
/// You can use this to abstract over the different kinds of rooted GC
/// references. Note that `Deref<Target = T>` is a supertrait for
/// `RootedGcRef<T>`, so all rooted GC references deref to their underlying `T`,
/// allowing you to call its methods.
///
/// This is a sealed trait, and may not be implemented for any types outside of
/// the `wasmtime` crate.
pub trait RootedGcRef<T>: RootedGcRefImpl<T> + Deref<Target = T>
where
    T: GcRef,
{
}

impl<T, U> RootedGcRef<T> for U
where
    T: GcRef,
    U: RootedGcRefImpl<T> + Deref<Target = T>,
{
}

/// An error returned when attempting to allocate a GC-managed object, but the
/// GC heap is out of memory.
///
/// This error wraps an inner `T` value -- which is the host value, if any, that
/// was passed to [`ExternRef::new`][crate::ExternRef::new] -- and you can
/// recover this value via the
/// [`into_inner`][crate::GcHeapOutOfMemory::into_inner] method. This lets you
/// try to allocate the `externref` again, after performing a GC to hopefully
/// free up space in the heap, or otherwise do whatever you want with the inner
/// value.
///
/// For errors that occur when attempting to allocate non-`externref` objects
/// when the GC heap is at capacity, the `T` type parameter is just the unit
/// type `()`.
pub struct GcHeapOutOfMemory<T> {
    inner: T,
    bytes_needed: u64,
}

impl<T> fmt::Debug for GcHeapOutOfMemory<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<T> fmt::Display for GcHeapOutOfMemory<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GC heap out of memory: no capacity for allocation of {} bytes",
            self.bytes_needed
        )
    }
}

impl<T> core::error::Error for GcHeapOutOfMemory<T> {}

impl<T> GcHeapOutOfMemory<T> {
    pub(crate) fn new(inner: T, bytes_needed: u64) -> Self {
        Self {
            inner,
            bytes_needed,
        }
    }

    #[cfg(feature = "gc")]
    pub(crate) fn bytes_needed(&self) -> u64 {
        self.bytes_needed
    }

    #[cfg(feature = "gc")]
    pub(crate) fn map_inner<U>(self, f: impl FnOnce(T) -> U) -> GcHeapOutOfMemory<U> {
        GcHeapOutOfMemory {
            inner: f(self.inner),
            bytes_needed: self.bytes_needed,
        }
    }

    /// Recover this error's inner host value.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Take this error's inner host value, but also retain this
    /// `GcHeapOutOfMemory` with `T` replaced with `()`.
    ///
    /// This allows you to both extract the inner `T` if necessary, and also
    /// pass the `GcHeapOutOfMemory` error to [`Store::gc`][crate::Store::gc]
    /// calls.
    pub fn take_inner(self) -> (T, GcHeapOutOfMemory<()>) {
        (self.inner, GcHeapOutOfMemory::new((), self.bytes_needed))
    }
}
