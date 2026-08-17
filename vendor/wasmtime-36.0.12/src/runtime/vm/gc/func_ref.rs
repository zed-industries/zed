//! Implementation of the side table for `funcref`s in the GC heap.
//!
//! The actual `VMFuncRef`s are kept in a side table, rather than inside the GC
//! heap, for the same reasons that an `externref`'s host data is kept in a side
//! table. We cannot trust any data coming from the GC heap, but `VMFuncRef`s
//! contain raw pointers, so if we stored `VMFuncRef`s inside the GC heap, we
//! wouldn't be able to use the raw pointers from any `VMFuncRef` we got out of
//! the heap. And that means we wouldn't be able to, for example, call a
//! `funcref` we got from inside the GC heap.

use crate::{
    hash_map::HashMap,
    type_registry::TypeRegistry,
    vm::{SendSyncPtr, VMFuncRef},
};
use wasmtime_environ::VMSharedTypeIndex;
use wasmtime_slab::{Id, Slab};

/// An identifier into the `FuncRefTable`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FuncRefTableId(Id);

impl FuncRefTableId {
    /// Convert this `FuncRefTableId` into its raw `u32` ID.
    pub fn into_raw(self) -> u32 {
        self.0.into_raw()
    }

    /// Create a `FuncRefTableId` from a raw `u32` ID.
    pub fn from_raw(raw: u32) -> Self {
        Self(Id::from_raw(raw))
    }
}

/// Side table mapping `FuncRefTableId`s that can be stored in the GC heap to
/// raw `VMFuncRef`s.
#[derive(Default)]
pub struct FuncRefTable {
    interned: HashMap<Option<SendSyncPtr<VMFuncRef>>, FuncRefTableId>,
    slab: Slab<Option<SendSyncPtr<VMFuncRef>>>,
}

impl FuncRefTable {
    /// Intern a `VMFuncRef` in the side table, returning an ID that can be
    /// stored in the GC heap.
    ///
    /// # Safety
    ///
    /// The given `func_ref` must point to a valid `VMFuncRef` and must remain
    /// valid for the duration of this table's lifetime.
    pub unsafe fn intern(&mut self, func_ref: Option<SendSyncPtr<VMFuncRef>>) -> FuncRefTableId {
        *self
            .interned
            .entry(func_ref)
            .or_insert_with(|| FuncRefTableId(self.slab.alloc(func_ref)))
    }

    /// Get the `VMFuncRef` associated with the given ID.
    ///
    /// Checks that the `VMFuncRef` is a subtype of the expected type.
    pub fn get_typed(
        &self,
        types: &TypeRegistry,
        id: FuncRefTableId,
        expected_ty: VMSharedTypeIndex,
    ) -> Option<SendSyncPtr<VMFuncRef>> {
        let f = self.slab.get(id.0).copied().expect("bad FuncRefTableId");

        if let Some(f) = f {
            // The safety contract for `intern` ensures that deref'ing `f` is safe.
            let actual_ty = unsafe { f.as_ref().type_index };

            // Ensure that the funcref actually is a subtype of the expected
            // type. This protects against GC heap corruption being leveraged in
            // attacks: if the attacker has a write gadget inside the GC heap, they
            // can overwrite a funcref ID to point to a different funcref, but this
            // assertion ensures that any calls to that wrong funcref at least
            // remain well-typed, which reduces the attack surface and maintains
            // memory safety.
            assert!(types.is_subtype(actual_ty, expected_ty));
        }

        f
    }

    /// Get the `VMFuncRef` associated with the given ID, without checking the
    /// type.
    ///
    /// Prefer `get_typed`. This method is only suitable for getting a
    /// `VMFuncRef` as an untyped `funcref` function reference, and never as a
    /// typed `(ref $some_func_type)` function reference.
    pub fn get_untyped(&self, id: FuncRefTableId) -> Option<SendSyncPtr<VMFuncRef>> {
        self.slab.get(id.0).copied().expect("bad FuncRefTableId")
    }
}
