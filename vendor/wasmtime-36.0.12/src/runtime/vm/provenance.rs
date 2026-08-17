//! Helpers related to pointer provenance for Wasmtime and its runtime.
//!
//! This module encapsulates the efforts and lengths that Wasmtime goes to in
//! order to properly respect pointer provenance in Rust with respect to unsafe
//! code. Wasmtime has a nontrivial amount of `unsafe` code and when/where
//! pointers are valid is something we need to be particularly careful about.
//! All safe Rust does not need to worry about this module and only the unsafe
//! runtime bits need to worry about it.
//!
//! In general Wasmtime does not work with Rust's strict pointer provenance
//! rules. The primary reason for this is that Cranelift does not have the
//! concept of a pointer type meaning that backends cannot know what values are
//! pointers and what aren't. This isn't a huge issue for ISAs like x64 but for
//! an ISA like Pulley Bytecode it means that the Pulley interpreter cannot
//! respect strict provenance.
//!
//! > **Aside**: an example of how Pulley can't respect pointer provenance is
//! > consider a wasm load. The wasm load will add a wasm address to the base
//! > address of the host. In this situation what actually needs to happen is
//! > that the base address of the host is a pointer which is byte-offset'd by
//! > the wasm address. Cranelift IR has no knowledge of which value is
//! > the wasm address and which is the host address. This means that Cranelift
//! > can freely commute the operands of the addition. This means that when
//! > executing Pulley doesn't know which values are addresses and which aren't.
//!
//! This isn't the end of the world for Wasmtime, however, it just means that
//! when we run in MIRI we are restricted to "permissive provenance" or "exposed
//! provenance". The tl;dr; of exposed provenance is that at certain points we
//! declare a pointer as "this is now exposed". That converts a pointer to the
//! `usize` address and then semantically (just for rustc/llvm mostly) indicates
//! that the provenance of the pointer is added to a global list of provenances.
//! Later on Wasmtime will execute an operation to convert a `usize` back into a
//! pointer which will pick "the most appropriate provenance" from said global
//! list of provenances.
//!
//! In practice we expect that at runtime all of these provenance-related ops
//! are noops and compile away to nothing. The only practical effect that's
//! expected is that some optimizations may be hindered in LLVM occasionally or
//! something like that which is by-and-large what we want to happen. Note that
//! another practical consequence of not working with "strict provenance" means
//! that Wasmtime is incompatible with platforms such as CHERI where exposed
//! provenance is not available.

use crate::vm::SendSyncPtr;
use core::fmt;
use core::marker;
use core::num::NonZeroUsize;
use core::ptr::NonNull;
use core::sync::atomic::AtomicUsize;
use wasmtime_environ::VMSharedTypeIndex;

/// A pointer that is used by compiled code, or in other words is accessed
/// outside of Rust.
///
/// This is intended to be the fundamental data type used to share
/// pointers-to-things with compiled wasm compiled code for example. An example
/// of this is that the `VMMemoryDefinition` type, which compiled code reads to
/// learn about linear memory, uses a `VmPtr<u8>` to represent the base pointer
/// of linear memory.
///
/// This type is pointer-sized and typed-like-a-pointer. This is additionally
/// like a `NonNull<T>` in that it's never a null pointer (and
/// `Option<VmPtr<T>>` is pointer-sized). This pointer auto-infers
/// `Send` and `Sync` based on `T`. Note the lack of `T: ?Sized` bounds in this
/// type additionally, meaning that it only works with sized types. That's
/// intentional as compiled code should not be interacting with dynamically
/// sized types in Rust.
///
/// This type serves two major purposes with respect to provenance and safety:
///
/// * Primarily this type is the only pointer type that implements `VmSafe`, the
///   marker trait below. That forces all pointers shared with compiled code to
///   use this type.
///
/// * This type represents a pointer with "exposed provenance". Once a value of
///   this type is created the original pointer's provenance will be marked as
///   exposed. This operation may hinder optimizations around the use of said
///   pointer in that case.
///
/// This type is expected to be used not only when sending pointers to compiled
/// code (e.g. `VMContext`) but additionally for any data at rest which shares
/// pointers with compiled code (for example the base of linear memory or
/// pointers stored within `VMContext` itself).
///
/// In general usage of this type should be minimized to only where absolutely
/// necessary when sharing data structures with compiled code. Prefer to use
/// `NonNull` or `SendSyncPtr` where possible.
#[repr(transparent)]
pub struct VmPtr<T> {
    ptr: NonZeroUsize,
    _marker: marker::PhantomData<SendSyncPtr<T>>,
}

impl<T> VmPtr<T> {
    /// View this pointer as a [`SendSyncPtr<T>`].
    ///
    /// This operation will convert the storage at-rest to a native pointer on
    /// the host. This is effectively an integer-to-pointer operation which will
    /// assume that the original pointer's provenance was previously exposed.
    /// In typical operation this means that Wasmtime will initialize data
    /// structures by creating an instance of `VmPtr`, exposing provenance.
    /// Later on this type will be handed back to Wasmtime or read from its
    /// location at-rest in which case provenance will be "re-acquired".
    pub fn as_send_sync(&self) -> SendSyncPtr<T> {
        SendSyncPtr::from(self.as_non_null())
    }

    /// Similar to `as_send_sync`, but returns a `NonNull<T>`.
    pub fn as_non_null(&self) -> NonNull<T> {
        let ptr = core::ptr::with_exposed_provenance_mut(self.ptr.get());
        unsafe { NonNull::new_unchecked(ptr) }
    }

    /// Similar to `as_send_sync`, but returns a `*mut T`.
    pub fn as_ptr(&self) -> *mut T {
        self.as_non_null().as_ptr()
    }
}

// `VmPtr<T>`, like raw pointers, is trivially `Clone`/`Copy`.
impl<T> Clone for VmPtr<T> {
    fn clone(&self) -> VmPtr<T> {
        *self
    }
}

impl<T> Copy for VmPtr<T> {}

// Forward debugging to `SendSyncPtr<T>` which renders the address.
impl<T> fmt::Debug for VmPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_send_sync().fmt(f)
    }
}

// Constructor from `NonNull<T>`
impl<T> From<NonNull<T>> for VmPtr<T> {
    fn from(ptr: NonNull<T>) -> VmPtr<T> {
        VmPtr {
            ptr: unsafe { NonZeroUsize::new_unchecked(ptr.as_ptr().expose_provenance()) },
            _marker: marker::PhantomData,
        }
    }
}

// Constructor from `SendSyncPtr<T>`
impl<T> From<SendSyncPtr<T>> for VmPtr<T> {
    fn from(ptr: SendSyncPtr<T>) -> VmPtr<T> {
        ptr.as_non_null().into()
    }
}

/// A custom "marker trait" used to tag types that are safe to share with
/// compiled wasm code.
///
/// The intention of this trait is to be used as a bound in a few core locations
/// in Wasmtime, such as `Instance::vmctx_plus_offset_mut`, and otherwise not
/// present very often. The purpose of this trait is to ensure that all types
/// stored to be shared with compiled code have a known layout and are
/// guaranteed to be "safe" to share with compiled wasm code.
///
/// This is an `unsafe` trait as it's generally not safe to share anything with
/// compiled code and it is used to invite extra scrutiny to manual `impl`s of
/// this trait. Types which implement this marker trait must satisfy at least
/// the following requirements.
///
/// * The ABI of `Self` must be well-known and defined. This means that the type
///   can interoperate with compiled code. For example `u8` is well defined as
///   is a `#[repr(C)]` structure. Types lacking `#[repr(C)]` or other types
///   like Rust tuples do not satisfy this requirement.
///
/// * For types which contain pointers the pointer's provenance is guaranteed to
///   have been exposed when the type is constructed. This is satisfied where
///   the only pointer that implements this trait is `VmPtr<T>` above which is
///   explicitly used to indicate exposed provenance. Notably `*mut T` and
///   `NonNull<T>` do not implement this trait, and intentionally so.
///
/// * For composite structures (e.g. `struct`s in Rust) all member fields must
///   satisfy the above criteria. All fields must have defined layouts and
///   pointers must be `VmPtr<T>`.
///
/// * Newtype or wrapper types around primitives that are used by value must be
///   `#[repr(transparent)]` to ensure they aren't considered aggregates by the
///   compile to match the ABI of the primitive type.
///
/// In this module a number of impls are provided for the primitives of Rust,
/// for example integers. Additionally some basic pointer-related impls are
/// provided for `VmPtr<T>` above. More impls can be found in `vmcontext.rs`
/// where there are manual impls for all `VM*` data structures which are shared
/// with compiled code.
pub unsafe trait VmSafe {}

// Implementations for primitive types. Note that atomics are included here as
// some atomic values are shared with compiled code. Rust's atomics are
// guaranteed to have the same memory representation as their primitive.
unsafe impl VmSafe for u8 {}
unsafe impl VmSafe for u16 {}
unsafe impl VmSafe for u32 {}
unsafe impl VmSafe for u64 {}
unsafe impl VmSafe for u128 {}
unsafe impl VmSafe for usize {}
unsafe impl VmSafe for i8 {}
unsafe impl VmSafe for i16 {}
unsafe impl VmSafe for i32 {}
unsafe impl VmSafe for i64 {}
unsafe impl VmSafe for i128 {}
unsafe impl VmSafe for isize {}
unsafe impl VmSafe for AtomicUsize {}
#[cfg(target_has_atomic = "64")]
unsafe impl VmSafe for core::sync::atomic::AtomicU64 {}

// This is a small `u32` wrapper defined in `wasmtime-environ`, so impl the
// vm-safe-ness here.
unsafe impl VmSafe for VMSharedTypeIndex {}

// Core implementations for `VmPtr`. Notably `VMPtr<T>` requires that `T` also
// implements `VmSafe`. Additionally an `Option` wrapper is allowed as that's
// just a nullable pointer.
unsafe impl<T: VmSafe> VmSafe for VmPtr<T> {}
unsafe impl<T: VmSafe> VmSafe for Option<VmPtr<T>> {}
