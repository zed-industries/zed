//! Lifetime management of `VMFuncRef`s inside of stores, and filling in their
//! trampolines.

use crate::Definition;
use crate::module::ModuleRegistry;
use crate::prelude::*;
use crate::runtime::HostFunc;
use crate::runtime::vm::{SendSyncPtr, VMArrayCallHostFuncContext, VMFuncRef};
use alloc::sync::Arc;
use core::ptr::NonNull;

/// An arena of `VMFuncRef`s.
///
/// Allows a store to pin and own funcrefs so that it can patch in trampolines
/// for `VMFuncRef`s that are missing a `wasm_call` trampoline and
/// need Wasm to supply it.
#[derive(Default)]
pub struct FuncRefs {
    /// A bump allocation arena where we allocate `VMFuncRef`s such
    /// that they are pinned and owned.
    bump: SendSyncBump,

    /// Pointers into `self.bump` for entries that need `wasm_call` field filled
    /// in.
    with_holes: Vec<SendSyncPtr<VMFuncRef>>,

    /// General-purpose storage of "function things" that need to live as long
    /// as the entire store.
    storage: Vec<Storage>,
}

/// Various items to place in `FuncRefs::storage`
///
/// Note that each field has its own heap-level indirection to be resistant to
/// `FuncRefs::storage` having its own backing storage reallocated.
enum Storage {
    /// Pinned arbitrary `Linker` definitions that must be kept alive for the
    /// entire duration of the store. This can include host functions, funcrefs
    /// inside them, etc.
    InstancePreDefinitions {
        #[expect(dead_code, reason = "only here to keep the original value alive")]
        defs: Arc<[Definition]>,
    },

    /// Pinned `VMFuncRef`s that had their `wasm_call` field
    /// pre-patched when constructing an `InstancePre`, and which we need to
    /// keep alive for our owning store's lifetime.
    InstancePreFuncRefs {
        #[expect(dead_code, reason = "only here to keep the original value alive")]
        funcs: Arc<[VMFuncRef]>,
    },

    /// A uniquely-owned host function within a `Store`. This comes about with
    /// `Func::new` or similar APIs. The `HostFunc` internally owns the
    /// `InstanceHandle` and that will get dropped when this `HostFunc` itself
    /// is dropped.
    ///
    /// Note that this contains the vmctx that the `VMFuncRef` points to for
    /// this host function.
    BoxHost {
        #[expect(dead_code, reason = "only here to keep the original value alive")]
        func: Box<HostFunc>,
    },

    /// A function is shared across possibly other stores, hence the `Arc`. This
    /// variant happens when a `Linker`-defined function is instantiated within
    /// a `Store` (e.g. via `Linker::get` or similar APIs). The `Arc` here
    /// indicates that there's some number of other stores holding this function
    /// too, so dropping this may not deallocate the underlying
    /// `InstanceHandle`.
    ///
    /// Note that this contains the vmctx that the `VMFuncRef` points to for
    /// this host function.
    ArcHost {
        #[expect(dead_code, reason = "only here to keep the original value alive")]
        func: Arc<HostFunc>,
    },
}

use send_sync_bump::SendSyncBump;
mod send_sync_bump {
    #[derive(Default)]
    pub struct SendSyncBump(bumpalo::Bump);

    impl SendSyncBump {
        pub fn alloc<T>(&mut self, val: T) -> &mut T {
            self.0.alloc(val)
        }
    }

    // Safety: We require `&mut self` on the only public method, which means it
    // is safe to send `&SendSyncBump` references across threads because they
    // can't actually do anything with it.
    unsafe impl Sync for SendSyncBump {}
}

impl FuncRefs {
    /// Push the given `VMFuncRef` into this arena, returning a
    /// pinned pointer to it.
    ///
    /// # Safety
    ///
    /// You may only access the return value on the same thread as this
    /// `FuncRefs` and only while the store holding this `FuncRefs` exists.
    /// Additionally the `vmctx` field of `func_ref` must be valid to read.
    pub unsafe fn push(
        &mut self,
        func_ref: VMFuncRef,
        modules: &ModuleRegistry,
    ) -> NonNull<VMFuncRef> {
        debug_assert!(func_ref.wasm_call.is_none());
        let func_ref = self.bump.alloc(func_ref);
        // SAFETY: it's a contract of this function itself that `func_ref` has a
        // valid vmctx field to read.
        let has_hole = unsafe { !try_fill(func_ref, modules) };
        let unpatched = SendSyncPtr::from(func_ref);
        if has_hole {
            self.with_holes.push(unpatched);
        }
        unpatched.as_non_null()
    }

    /// Patch any `VMFuncRef::wasm_call`s that need filling in.
    pub fn fill(&mut self, modules: &ModuleRegistry) {
        self.with_holes
            .retain_mut(|f| unsafe { !try_fill(f.as_mut(), modules) });
    }

    /// Reserves `amt` space for extra items in "storage" for this store.
    pub fn reserve_storage(&mut self, amt: usize) {
        self.storage.reserve(amt);
    }

    /// Push pre-patched `VMFuncRef`s from an `InstancePre`.
    ///
    /// This is used to ensure that the store itself persists the entire list of
    /// `funcs` for the entire lifetime of the store.
    pub fn push_instance_pre_func_refs(&mut self, funcs: Arc<[VMFuncRef]>) {
        self.storage.push(Storage::InstancePreFuncRefs { funcs });
    }

    /// Push linker definitions into storage, keeping them alive for the entire
    /// lifetime of the store.
    ///
    /// This is used to keep linker-defined functions' vmctx values alive, for
    /// example.
    pub fn push_instance_pre_definitions(&mut self, defs: Arc<[Definition]>) {
        self.storage.push(Storage::InstancePreDefinitions { defs });
    }

    /// Pushes a shared host function into this store.
    ///
    /// This will create a store-local `VMFuncRef` with a hole to fill in where
    /// the `wasm_call` will get filled in as needed.
    ///
    /// This function returns a `VMFuncRef` which is store-local and will have
    /// `wasm_call` filled in eventually if needed.
    ///
    /// # Safety
    ///
    /// You may only access the return value on the same thread as this
    /// `FuncRefs` and only while the store holding this `FuncRefs` exists.
    pub fn push_arc_host(
        &mut self,
        func: Arc<HostFunc>,
        modules: &ModuleRegistry,
    ) -> NonNull<VMFuncRef> {
        debug_assert!(func.func_ref().wasm_call.is_none());
        // SAFETY: the vmctx field in the funcref of `HostFunc` is safe to read.
        let ret = unsafe { self.push(func.func_ref().clone(), modules) };
        self.storage.push(Storage::ArcHost { func });
        ret
    }

    /// Same as `push_arc_host`, but for owned host functions.
    pub fn push_box_host(
        &mut self,
        func: Box<HostFunc>,
        modules: &ModuleRegistry,
    ) -> NonNull<VMFuncRef> {
        debug_assert!(func.func_ref().wasm_call.is_none());
        // SAFETY: the vmctx field in the funcref of `HostFunc` is safe to read.
        let ret = unsafe { self.push(func.func_ref().clone(), modules) };
        self.storage.push(Storage::BoxHost { func });
        ret
    }
}

/// Attempts to fill the `wasm_call` field of `func_ref` given `modules`
/// registered and returns `true` if the field was filled, `false` otherwise.
///
/// # Panics
///
/// Panics if `func_ref.wasm_call.is_some()`
///
/// # Safety
///
/// This relies on `func_ref` being a valid pointer with a valid `vmctx` field.
unsafe fn try_fill(func_ref: &mut VMFuncRef, modules: &ModuleRegistry) -> bool {
    debug_assert!(func_ref.wasm_call.is_none());

    // Debug assert that the vmctx is a `VMArrayCallHostFuncContext` as
    // that is the only kind that can have holes.
    //
    // SAFETY: the validity of `vmctx` is a contract of this function itself.
    unsafe {
        let _ = VMArrayCallHostFuncContext::from_opaque(func_ref.vmctx.as_non_null());
    }

    func_ref.wasm_call = modules
        .wasm_to_array_trampoline(func_ref.type_index)
        .map(|f| f.into());
    func_ref.wasm_call.is_some()
}
