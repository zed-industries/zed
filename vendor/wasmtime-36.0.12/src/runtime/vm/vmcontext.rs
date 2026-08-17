//! This file declares `VMContext` and several related structs which contain
//! fields that compiled wasm code accesses directly.

mod vm_host_func_context;

pub use self::vm_host_func_context::VMArrayCallHostFuncContext;
use crate::prelude::*;
use crate::runtime::vm::{GcStore, InterpreterRef, VMGcRef, VmPtr, VmSafe, f32x4, f64x2, i8x16};
use crate::store::StoreOpaque;
use crate::vm::stack_switching::VMStackChain;
use core::cell::UnsafeCell;
use core::ffi::c_void;
use core::fmt;
use core::marker;
use core::mem::{self, MaybeUninit};
use core::ops::Range;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use wasmtime_environ::{
    BuiltinFunctionIndex, DefinedGlobalIndex, DefinedMemoryIndex, DefinedTableIndex,
    DefinedTagIndex, Unsigned, VMCONTEXT_MAGIC, VMSharedTypeIndex, WasmHeapTopType, WasmValType,
};

/// A function pointer that exposes the array calling convention.
///
/// Regardless of the underlying Wasm function type, all functions using the
/// array calling convention have the same Rust signature.
///
/// Arguments:
///
/// * Callee `vmctx` for the function itself.
///
/// * Caller's `vmctx` (so that host functions can access the linear memory of
///   their Wasm callers).
///
/// * A pointer to a buffer of `ValRaw`s where both arguments are passed into
///   this function, and where results are returned from this function.
///
/// * The capacity of the `ValRaw` buffer. Must always be at least
///   `max(len(wasm_params), len(wasm_results))`.
///
/// Return value:
///
/// * `true` if this call succeeded.
/// * `false` if this call failed and a trap was recorded in TLS.
pub type VMArrayCallNative = unsafe extern "C" fn(
    NonNull<VMOpaqueContext>,
    NonNull<VMContext>,
    NonNull<ValRaw>,
    usize,
) -> bool;

/// An opaque function pointer which might be `VMArrayCallNative` or it might be
/// pulley bytecode. Requires external knowledge to determine what kind of
/// function pointer this is.
#[repr(transparent)]
pub struct VMArrayCallFunction(VMFunctionBody);

/// A function pointer that exposes the Wasm calling convention.
///
/// In practice, different Wasm function types end up mapping to different Rust
/// function types, so this isn't simply a type alias the way that
/// `VMArrayCallFunction` is. However, the exact details of the calling
/// convention are left to the Wasm compiler (e.g. Cranelift or Winch). Runtime
/// code never does anything with these function pointers except shuffle them
/// around and pass them back to Wasm.
#[repr(transparent)]
pub struct VMWasmCallFunction(VMFunctionBody);

/// An imported function.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VMFunctionImport {
    /// Function pointer to use when calling this imported function from Wasm.
    pub wasm_call: VmPtr<VMWasmCallFunction>,

    /// Function pointer to use when calling this imported function with the
    /// "array" calling convention that `Func::new` et al use.
    pub array_call: VmPtr<VMArrayCallFunction>,

    /// The VM state associated with this function.
    ///
    /// For Wasm functions defined by core wasm instances this will be `*mut
    /// VMContext`, but for lifted/lowered component model functions this will
    /// be a `VMComponentContext`, and for a host function it will be a
    /// `VMHostFuncContext`, etc.
    pub vmctx: VmPtr<VMOpaqueContext>,
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMFunctionImport {}

#[cfg(test)]
mod test_vmfunction_import {
    use super::VMFunctionImport;
    use core::mem::offset_of;
    use std::mem::size_of;
    use wasmtime_environ::{HostPtr, Module, VMOffsets};

    #[test]
    fn check_vmfunction_import_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMFunctionImport>(),
            usize::from(offsets.size_of_vmfunction_import())
        );
        assert_eq!(
            offset_of!(VMFunctionImport, wasm_call),
            usize::from(offsets.vmfunction_import_wasm_call())
        );
        assert_eq!(
            offset_of!(VMFunctionImport, array_call),
            usize::from(offsets.vmfunction_import_array_call())
        );
        assert_eq!(
            offset_of!(VMFunctionImport, vmctx),
            usize::from(offsets.vmfunction_import_vmctx())
        );
    }
}

/// A placeholder byte-sized type which is just used to provide some amount of type
/// safety when dealing with pointers to JIT-compiled function bodies. Note that it's
/// deliberately not Copy, as we shouldn't be carelessly copying function body bytes
/// around.
#[repr(C)]
pub struct VMFunctionBody(u8);

// SAFETY: this structure is never read and is safe to pass to jit code.
unsafe impl VmSafe for VMFunctionBody {}

#[cfg(test)]
mod test_vmfunction_body {
    use super::VMFunctionBody;
    use std::mem::size_of;

    #[test]
    fn check_vmfunction_body_offsets() {
        assert_eq!(size_of::<VMFunctionBody>(), 1);
    }
}

/// The fields compiled code needs to access to utilize a WebAssembly table
/// imported from another instance.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VMTableImport {
    /// A pointer to the imported table description.
    pub from: VmPtr<VMTableDefinition>,

    /// A pointer to the `VMContext` that owns the table description.
    pub vmctx: VmPtr<VMContext>,

    /// The table index, within `vmctx`, this definition resides at.
    pub index: DefinedTableIndex,
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMTableImport {}

#[cfg(test)]
mod test_vmtable {
    use super::VMTableImport;
    use core::mem::offset_of;
    use std::mem::size_of;
    use wasmtime_environ::component::{Component, VMComponentOffsets};
    use wasmtime_environ::{HostPtr, Module, VMOffsets};

    #[test]
    fn check_vmtable_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMTableImport>(),
            usize::from(offsets.size_of_vmtable_import())
        );
        assert_eq!(
            offset_of!(VMTableImport, from),
            usize::from(offsets.vmtable_import_from())
        );
        assert_eq!(
            offset_of!(VMTableImport, vmctx),
            usize::from(offsets.vmtable_import_vmctx())
        );
        assert_eq!(
            offset_of!(VMTableImport, index),
            usize::from(offsets.vmtable_import_index())
        );
    }

    #[test]
    fn ensure_sizes_match() {
        // Because we use `VMTableImport` for recording tables used by components, we
        // want to make sure that the size calculations between `VMOffsets` and
        // `VMComponentOffsets` stay the same.
        let module = Module::new();
        let vm_offsets = VMOffsets::new(HostPtr, &module);
        let component = Component::default();
        let vm_component_offsets = VMComponentOffsets::new(HostPtr, &component);
        assert_eq!(
            vm_offsets.size_of_vmtable_import(),
            vm_component_offsets.size_of_vmtable_import()
        );
    }
}

/// The fields compiled code needs to access to utilize a WebAssembly linear
/// memory imported from another instance.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VMMemoryImport {
    /// A pointer to the imported memory description.
    pub from: VmPtr<VMMemoryDefinition>,

    /// A pointer to the `VMContext` that owns the memory description.
    pub vmctx: VmPtr<VMContext>,

    /// The index of the memory in the containing `vmctx`.
    pub index: DefinedMemoryIndex,
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMMemoryImport {}

#[cfg(test)]
mod test_vmmemory_import {
    use super::VMMemoryImport;
    use core::mem::offset_of;
    use std::mem::size_of;
    use wasmtime_environ::{HostPtr, Module, VMOffsets};

    #[test]
    fn check_vmmemory_import_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMMemoryImport>(),
            usize::from(offsets.size_of_vmmemory_import())
        );
        assert_eq!(
            offset_of!(VMMemoryImport, from),
            usize::from(offsets.vmmemory_import_from())
        );
        assert_eq!(
            offset_of!(VMMemoryImport, vmctx),
            usize::from(offsets.vmmemory_import_vmctx())
        );
        assert_eq!(
            offset_of!(VMMemoryImport, index),
            usize::from(offsets.vmmemory_import_index())
        );
    }
}

/// The fields compiled code needs to access to utilize a WebAssembly global
/// variable imported from another instance.
///
/// Note that unlike with functions, tables, and memories, `VMGlobalImport`
/// doesn't include a `vmctx` pointer. Globals are never resized, and don't
/// require a `vmctx` pointer to access.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VMGlobalImport {
    /// A pointer to the imported global variable description.
    pub from: VmPtr<VMGlobalDefinition>,

    /// A pointer to the context that owns the global.
    ///
    /// Exactly what's stored here is dictated by `kind` below. This is `None`
    /// for `VMGlobalKind::Host`, it's a `VMContext` for
    /// `VMGlobalKind::Instance`, and it's `VMComponentContext` for
    /// `VMGlobalKind::ComponentFlags`.
    pub vmctx: Option<VmPtr<VMOpaqueContext>>,

    /// The kind of global, and extra location information in addition to
    /// `vmctx` above.
    pub kind: VMGlobalKind,
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMGlobalImport {}

/// The kinds of globals that Wasmtime has.
#[derive(Debug, Copy, Clone)]
#[repr(C, u32)]
pub enum VMGlobalKind {
    /// Host globals, stored in a `StoreOpaque`.
    Host(DefinedGlobalIndex),
    /// Instance globals, stored in `VMContext`s
    Instance(DefinedGlobalIndex),
    /// Flags for a component instance, stored in `VMComponentContext`.
    #[cfg(feature = "component-model")]
    ComponentFlags(wasmtime_environ::component::RuntimeComponentInstanceIndex),
}

// SAFETY: the above enum is repr(C) and stores nothing else
unsafe impl VmSafe for VMGlobalKind {}

#[cfg(test)]
mod test_vmglobal_import {
    use super::VMGlobalImport;
    use core::mem::offset_of;
    use std::mem::size_of;
    use wasmtime_environ::{HostPtr, Module, VMOffsets};

    #[test]
    fn check_vmglobal_import_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMGlobalImport>(),
            usize::from(offsets.size_of_vmglobal_import())
        );
        assert_eq!(
            offset_of!(VMGlobalImport, from),
            usize::from(offsets.vmglobal_import_from())
        );
    }
}

/// The fields compiled code needs to access to utilize a WebAssembly
/// tag imported from another instance.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VMTagImport {
    /// A pointer to the imported tag description.
    pub from: VmPtr<VMTagDefinition>,

    /// The instance that owns this tag.
    pub vmctx: VmPtr<VMContext>,

    /// The index of the tag in the containing `vmctx`.
    pub index: DefinedTagIndex,
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMTagImport {}

#[cfg(test)]
mod test_vmtag_import {
    use super::VMTagImport;
    use core::mem::{offset_of, size_of};
    use wasmtime_environ::{HostPtr, Module, VMOffsets};

    #[test]
    fn check_vmtag_import_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMTagImport>(),
            usize::from(offsets.size_of_vmtag_import())
        );
        assert_eq!(
            offset_of!(VMTagImport, from),
            usize::from(offsets.vmtag_import_from())
        );
    }
}

/// The fields compiled code needs to access to utilize a WebAssembly linear
/// memory defined within the instance, namely the start address and the
/// size in bytes.
#[derive(Debug)]
#[repr(C)]
pub struct VMMemoryDefinition {
    /// The start address.
    pub base: VmPtr<u8>,

    /// The current logical size of this linear memory in bytes.
    ///
    /// This is atomic because shared memories must be able to grow their length
    /// atomically. For relaxed access, see
    /// [`VMMemoryDefinition::current_length()`].
    pub current_length: AtomicUsize,
}

// SAFETY: the above definition has `repr(C)` and each field individually
// implements `VmSafe`, which satisfies the requirements of this trait.
unsafe impl VmSafe for VMMemoryDefinition {}

impl VMMemoryDefinition {
    /// Return the current length (in bytes) of the [`VMMemoryDefinition`] by
    /// performing a relaxed load; do not use this function for situations in
    /// which a precise length is needed. Owned memories (i.e., non-shared) will
    /// always return a precise result (since no concurrent modification is
    /// possible) but shared memories may see an imprecise value--a
    /// `current_length` potentially smaller than what some other thread
    /// observes. Since Wasm memory only grows, this under-estimation may be
    /// acceptable in certain cases.
    #[inline]
    pub fn current_length(&self) -> usize {
        self.current_length.load(Ordering::Relaxed)
    }

    /// Return a copy of the [`VMMemoryDefinition`] using the relaxed value of
    /// `current_length`; see [`VMMemoryDefinition::current_length()`].
    #[inline]
    pub unsafe fn load(ptr: *mut Self) -> Self {
        let other = unsafe { &*ptr };
        VMMemoryDefinition {
            base: other.base,
            current_length: other.current_length().into(),
        }
    }
}

#[cfg(test)]
mod test_vmmemory_definition {
    use super::VMMemoryDefinition;
    use core::mem::offset_of;
    use std::mem::size_of;
    use wasmtime_environ::{HostPtr, Module, PtrSize, VMOffsets};

    #[test]
    fn check_vmmemory_definition_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMMemoryDefinition>(),
            usize::from(offsets.ptr.size_of_vmmemory_definition())
        );
        assert_eq!(
            offset_of!(VMMemoryDefinition, base),
            usize::from(offsets.ptr.vmmemory_definition_base())
        );
        assert_eq!(
            offset_of!(VMMemoryDefinition, current_length),
            usize::from(offsets.ptr.vmmemory_definition_current_length())
        );
        /* TODO: Assert that the size of `current_length` matches.
        assert_eq!(
            size_of::<VMMemoryDefinition::current_length>(),
            usize::from(offsets.size_of_vmmemory_definition_current_length())
        );
        */
    }
}

/// The fields compiled code needs to access to utilize a WebAssembly table
/// defined within the instance.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct VMTableDefinition {
    /// Pointer to the table data.
    pub base: VmPtr<u8>,

    /// The current number of elements in the table.
    pub current_elements: usize,
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMTableDefinition {}

#[cfg(test)]
mod test_vmtable_definition {
    use super::VMTableDefinition;
    use core::mem::offset_of;
    use std::mem::size_of;
    use wasmtime_environ::{HostPtr, Module, VMOffsets};

    #[test]
    fn check_vmtable_definition_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMTableDefinition>(),
            usize::from(offsets.size_of_vmtable_definition())
        );
        assert_eq!(
            offset_of!(VMTableDefinition, base),
            usize::from(offsets.vmtable_definition_base())
        );
        assert_eq!(
            offset_of!(VMTableDefinition, current_elements),
            usize::from(offsets.vmtable_definition_current_elements())
        );
    }
}

/// The storage for a WebAssembly global defined within the instance.
///
/// TODO: Pack the globals more densely, rather than using the same size
/// for every type.
#[derive(Debug)]
#[repr(C, align(16))]
pub struct VMGlobalDefinition {
    storage: [u8; 16],
    // If more elements are added here, remember to add offset_of tests below!
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMGlobalDefinition {}

#[cfg(test)]
mod test_vmglobal_definition {
    use super::VMGlobalDefinition;
    use std::mem::{align_of, size_of};
    use wasmtime_environ::{HostPtr, Module, PtrSize, VMOffsets};

    #[test]
    fn check_vmglobal_definition_alignment() {
        assert!(align_of::<VMGlobalDefinition>() >= align_of::<i32>());
        assert!(align_of::<VMGlobalDefinition>() >= align_of::<i64>());
        assert!(align_of::<VMGlobalDefinition>() >= align_of::<f32>());
        assert!(align_of::<VMGlobalDefinition>() >= align_of::<f64>());
        assert!(align_of::<VMGlobalDefinition>() >= align_of::<[u8; 16]>());
        assert!(align_of::<VMGlobalDefinition>() >= align_of::<[f32; 4]>());
        assert!(align_of::<VMGlobalDefinition>() >= align_of::<[f64; 2]>());
    }

    #[test]
    fn check_vmglobal_definition_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMGlobalDefinition>(),
            usize::from(offsets.ptr.size_of_vmglobal_definition())
        );
    }

    #[test]
    fn check_vmglobal_begins_aligned() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(offsets.vmctx_globals_begin() % 16, 0);
    }

    #[test]
    #[cfg(feature = "gc")]
    fn check_vmglobal_can_contain_gc_ref() {
        assert!(size_of::<crate::runtime::vm::VMGcRef>() <= size_of::<VMGlobalDefinition>());
    }
}

impl VMGlobalDefinition {
    /// Construct a `VMGlobalDefinition`.
    pub fn new() -> Self {
        Self { storage: [0; 16] }
    }

    /// Create a `VMGlobalDefinition` from a `ValRaw`.
    ///
    /// # Unsafety
    ///
    /// This raw value's type must match the given `WasmValType`.
    pub unsafe fn from_val_raw(
        store: &mut StoreOpaque,
        wasm_ty: WasmValType,
        raw: ValRaw,
    ) -> Result<Self> {
        let mut global = Self::new();
        unsafe {
            match wasm_ty {
                WasmValType::I32 => *global.as_i32_mut() = raw.get_i32(),
                WasmValType::I64 => *global.as_i64_mut() = raw.get_i64(),
                WasmValType::F32 => *global.as_f32_bits_mut() = raw.get_f32(),
                WasmValType::F64 => *global.as_f64_bits_mut() = raw.get_f64(),
                WasmValType::V128 => global.set_u128(raw.get_v128()),
                WasmValType::Ref(r) => match r.heap_type.top() {
                    WasmHeapTopType::Extern => {
                        let r = VMGcRef::from_raw_u32(raw.get_externref());
                        global.init_gc_ref(store.gc_store_mut()?, r.as_ref())
                    }
                    WasmHeapTopType::Any => {
                        let r = VMGcRef::from_raw_u32(raw.get_anyref());
                        global.init_gc_ref(store.gc_store_mut()?, r.as_ref())
                    }
                    WasmHeapTopType::Func => *global.as_func_ref_mut() = raw.get_funcref().cast(),
                    WasmHeapTopType::Cont => *global.as_func_ref_mut() = raw.get_funcref().cast(), // TODO(#10248): temporary hack.
                    WasmHeapTopType::Exn => {
                        let r = VMGcRef::from_raw_u32(raw.get_exnref());
                        global.init_gc_ref(store.gc_store_mut()?, r.as_ref())
                    }
                },
            }
        }
        Ok(global)
    }

    /// Get this global's value as a `ValRaw`.
    ///
    /// # Unsafety
    ///
    /// This global's value's type must match the given `WasmValType`.
    pub unsafe fn to_val_raw(
        &self,
        store: &mut StoreOpaque,
        wasm_ty: WasmValType,
    ) -> Result<ValRaw> {
        unsafe {
            Ok(match wasm_ty {
                WasmValType::I32 => ValRaw::i32(*self.as_i32()),
                WasmValType::I64 => ValRaw::i64(*self.as_i64()),
                WasmValType::F32 => ValRaw::f32(*self.as_f32_bits()),
                WasmValType::F64 => ValRaw::f64(*self.as_f64_bits()),
                WasmValType::V128 => ValRaw::v128(self.get_u128()),
                WasmValType::Ref(r) => match r.heap_type.top() {
                    WasmHeapTopType::Extern => ValRaw::externref(match self.as_gc_ref() {
                        Some(r) => store.gc_store_mut()?.clone_gc_ref(r).as_raw_u32(),
                        None => 0,
                    }),
                    WasmHeapTopType::Any => ValRaw::anyref({
                        match self.as_gc_ref() {
                            Some(r) => store.gc_store_mut()?.clone_gc_ref(r).as_raw_u32(),
                            None => 0,
                        }
                    }),
                    WasmHeapTopType::Exn => ValRaw::exnref({
                        match self.as_gc_ref() {
                            Some(r) => store.gc_store_mut()?.clone_gc_ref(r).as_raw_u32(),
                            None => 0,
                        }
                    }),
                    WasmHeapTopType::Func => ValRaw::funcref(self.as_func_ref().cast()),
                    WasmHeapTopType::Cont => todo!(), // FIXME: #10248 stack switching support.
                },
            })
        }
    }

    /// Return a reference to the value as an i32.
    pub unsafe fn as_i32(&self) -> &i32 {
        unsafe { &*(self.storage.as_ref().as_ptr().cast::<i32>()) }
    }

    /// Return a mutable reference to the value as an i32.
    pub unsafe fn as_i32_mut(&mut self) -> &mut i32 {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<i32>()) }
    }

    /// Return a reference to the value as a u32.
    pub unsafe fn as_u32(&self) -> &u32 {
        unsafe { &*(self.storage.as_ref().as_ptr().cast::<u32>()) }
    }

    /// Return a mutable reference to the value as an u32.
    pub unsafe fn as_u32_mut(&mut self) -> &mut u32 {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<u32>()) }
    }

    /// Return a reference to the value as an i64.
    pub unsafe fn as_i64(&self) -> &i64 {
        unsafe { &*(self.storage.as_ref().as_ptr().cast::<i64>()) }
    }

    /// Return a mutable reference to the value as an i64.
    pub unsafe fn as_i64_mut(&mut self) -> &mut i64 {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<i64>()) }
    }

    /// Return a reference to the value as an u64.
    pub unsafe fn as_u64(&self) -> &u64 {
        unsafe { &*(self.storage.as_ref().as_ptr().cast::<u64>()) }
    }

    /// Return a mutable reference to the value as an u64.
    pub unsafe fn as_u64_mut(&mut self) -> &mut u64 {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<u64>()) }
    }

    /// Return a reference to the value as an f32.
    pub unsafe fn as_f32(&self) -> &f32 {
        unsafe { &*(self.storage.as_ref().as_ptr().cast::<f32>()) }
    }

    /// Return a mutable reference to the value as an f32.
    pub unsafe fn as_f32_mut(&mut self) -> &mut f32 {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<f32>()) }
    }

    /// Return a reference to the value as f32 bits.
    pub unsafe fn as_f32_bits(&self) -> &u32 {
        unsafe { &*(self.storage.as_ref().as_ptr().cast::<u32>()) }
    }

    /// Return a mutable reference to the value as f32 bits.
    pub unsafe fn as_f32_bits_mut(&mut self) -> &mut u32 {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<u32>()) }
    }

    /// Return a reference to the value as an f64.
    pub unsafe fn as_f64(&self) -> &f64 {
        unsafe { &*(self.storage.as_ref().as_ptr().cast::<f64>()) }
    }

    /// Return a mutable reference to the value as an f64.
    pub unsafe fn as_f64_mut(&mut self) -> &mut f64 {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<f64>()) }
    }

    /// Return a reference to the value as f64 bits.
    pub unsafe fn as_f64_bits(&self) -> &u64 {
        unsafe { &*(self.storage.as_ref().as_ptr().cast::<u64>()) }
    }

    /// Return a mutable reference to the value as f64 bits.
    pub unsafe fn as_f64_bits_mut(&mut self) -> &mut u64 {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<u64>()) }
    }

    /// Gets the underlying 128-bit vector value.
    //
    // Note that vectors are stored in little-endian format while other types
    // are stored in native-endian format.
    pub unsafe fn get_u128(&self) -> u128 {
        unsafe { u128::from_le(*(self.storage.as_ref().as_ptr().cast::<u128>())) }
    }

    /// Sets the 128-bit vector values.
    //
    // Note that vectors are stored in little-endian format while other types
    // are stored in native-endian format.
    pub unsafe fn set_u128(&mut self, val: u128) {
        unsafe {
            *self.storage.as_mut().as_mut_ptr().cast::<u128>() = val.to_le();
        }
    }

    /// Return a reference to the value as u128 bits.
    pub unsafe fn as_u128_bits(&self) -> &[u8; 16] {
        unsafe { &*(self.storage.as_ref().as_ptr().cast::<[u8; 16]>()) }
    }

    /// Return a mutable reference to the value as u128 bits.
    pub unsafe fn as_u128_bits_mut(&mut self) -> &mut [u8; 16] {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<[u8; 16]>()) }
    }

    /// Return a reference to the global value as a borrowed GC reference.
    pub unsafe fn as_gc_ref(&self) -> Option<&VMGcRef> {
        let raw_ptr = self.storage.as_ref().as_ptr().cast::<Option<VMGcRef>>();
        let ret = unsafe { (*raw_ptr).as_ref() };
        assert!(cfg!(feature = "gc") || ret.is_none());
        ret
    }

    /// Initialize a global to the given GC reference.
    pub unsafe fn init_gc_ref(&mut self, gc_store: &mut GcStore, gc_ref: Option<&VMGcRef>) {
        assert!(cfg!(feature = "gc") || gc_ref.is_none());

        let dest = unsafe {
            &mut *(self
                .storage
                .as_mut()
                .as_mut_ptr()
                .cast::<MaybeUninit<Option<VMGcRef>>>())
        };

        gc_store.init_gc_ref(dest, gc_ref)
    }

    /// Write a GC reference into this global value.
    pub unsafe fn write_gc_ref(&mut self, gc_store: &mut GcStore, gc_ref: Option<&VMGcRef>) {
        assert!(cfg!(feature = "gc") || gc_ref.is_none());

        let dest = unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<Option<VMGcRef>>()) };
        assert!(cfg!(feature = "gc") || dest.is_none());

        gc_store.write_gc_ref(dest, gc_ref)
    }

    /// Return a reference to the value as a `VMFuncRef`.
    pub unsafe fn as_func_ref(&self) -> *mut VMFuncRef {
        unsafe { *(self.storage.as_ref().as_ptr().cast::<*mut VMFuncRef>()) }
    }

    /// Return a mutable reference to the value as a `VMFuncRef`.
    pub unsafe fn as_func_ref_mut(&mut self) -> &mut *mut VMFuncRef {
        unsafe { &mut *(self.storage.as_mut().as_mut_ptr().cast::<*mut VMFuncRef>()) }
    }
}

#[cfg(test)]
mod test_vmshared_type_index {
    use super::VMSharedTypeIndex;
    use std::mem::size_of;
    use wasmtime_environ::{HostPtr, Module, VMOffsets};

    #[test]
    fn check_vmshared_type_index() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMSharedTypeIndex>(),
            usize::from(offsets.size_of_vmshared_type_index())
        );
    }
}

/// A WebAssembly tag defined within the instance.
///
#[derive(Debug)]
#[repr(C)]
pub struct VMTagDefinition {
    /// Function signature's type id.
    pub type_index: VMSharedTypeIndex,
}

impl VMTagDefinition {
    pub fn new(type_index: VMSharedTypeIndex) -> Self {
        Self { type_index }
    }
}

// SAFETY: the above structure is repr(C) and only contains VmSafe
// fields.
unsafe impl VmSafe for VMTagDefinition {}

#[cfg(test)]
mod test_vmtag_definition {
    use super::VMTagDefinition;
    use std::mem::size_of;
    use wasmtime_environ::{HostPtr, Module, PtrSize, VMOffsets};

    #[test]
    fn check_vmtag_definition_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMTagDefinition>(),
            usize::from(offsets.ptr.size_of_vmtag_definition())
        );
    }

    #[test]
    fn check_vmtag_begins_aligned() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(offsets.vmctx_tags_begin() % 16, 0);
    }
}

/// The VM caller-checked "funcref" record, for caller-side signature checking.
///
/// It consists of function pointer(s), a type id to be checked by the
/// caller, and the vmctx closure associated with this function.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct VMFuncRef {
    /// Function pointer for this funcref if being called via the "array"
    /// calling convention that `Func::new` et al use.
    pub array_call: VmPtr<VMArrayCallFunction>,

    /// Function pointer for this funcref if being called via the calling
    /// convention we use when compiling Wasm.
    ///
    /// Most functions come with a function pointer that we can use when they
    /// are called from Wasm. The notable exception is when we `Func::wrap` a
    /// host function, and we don't have a Wasm compiler on hand to compile a
    /// Wasm-to-native trampoline for the function. In this case, we leave
    /// `wasm_call` empty until the function is passed as an import to Wasm (or
    /// otherwise exposed to Wasm via tables/globals). At this point, we look up
    /// a Wasm-to-native trampoline for the function in the Wasm's compiled
    /// module and use that fill in `VMFunctionImport::wasm_call`. **However**
    /// there is no guarantee that the Wasm module has a trampoline for this
    /// function's signature. The Wasm module only has trampolines for its
    /// types, and if this function isn't of one of those types, then the Wasm
    /// module will not have a trampoline for it. This is actually okay, because
    /// it means that the Wasm cannot actually call this function. But it does
    /// mean that this field needs to be an `Option` even though it is non-null
    /// the vast vast vast majority of the time.
    pub wasm_call: Option<VmPtr<VMWasmCallFunction>>,

    /// Function signature's type id.
    pub type_index: VMSharedTypeIndex,

    /// The VM state associated with this function.
    ///
    /// The actual definition of what this pointer points to depends on the
    /// function being referenced: for core Wasm functions, this is a `*mut
    /// VMContext`, for host functions it is a `*mut VMHostFuncContext`, and for
    /// component functions it is a `*mut VMComponentContext`.
    pub vmctx: VmPtr<VMOpaqueContext>,
    // If more elements are added here, remember to add offset_of tests below!
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMFuncRef {}

impl VMFuncRef {
    /// Invokes the `array_call` field of this `VMFuncRef` with the supplied
    /// arguments.
    ///
    /// This will invoke the function pointer in the `array_call` field with:
    ///
    /// * the `callee` vmctx as `self.vmctx`
    /// * the `caller` as `caller` specified here
    /// * the args pointer as `args_and_results`
    /// * the args length as `args_and_results`
    ///
    /// The `args_and_results` area must be large enough to both load all
    /// arguments from and store all results to.
    ///
    /// Returns whether a trap was recorded in TLS for raising.
    ///
    /// # Unsafety
    ///
    /// This method is unsafe because it can be called with any pointers. They
    /// must all be valid for this wasm function call to proceed. For example
    /// the `caller` must be valid machine code if `pulley` is `None` or it must
    /// be valid bytecode if `pulley` is `Some`. Additionally `args_and_results`
    /// must be large enough to handle all the arguments/results for this call.
    ///
    /// Note that the unsafety invariants to maintain here are not currently
    /// exhaustively documented.
    #[inline]
    pub unsafe fn array_call(
        me: NonNull<VMFuncRef>,
        pulley: Option<InterpreterRef<'_>>,
        caller: NonNull<VMContext>,
        args_and_results: NonNull<[ValRaw]>,
    ) -> bool {
        match pulley {
            Some(vm) => unsafe { Self::array_call_interpreted(me, vm, caller, args_and_results) },
            None => unsafe { Self::array_call_native(me, caller, args_and_results) },
        }
    }

    unsafe fn array_call_interpreted(
        me: NonNull<VMFuncRef>,
        vm: InterpreterRef<'_>,
        caller: NonNull<VMContext>,
        args_and_results: NonNull<[ValRaw]>,
    ) -> bool {
        // If `caller` is actually a `VMArrayCallHostFuncContext` then skip the
        // interpreter, even though it's available, as `array_call` will be
        // native code.
        unsafe {
            if me.as_ref().vmctx.as_non_null().as_ref().magic
                == wasmtime_environ::VM_ARRAY_CALL_HOST_FUNC_MAGIC
            {
                return Self::array_call_native(me, caller, args_and_results);
            }
            vm.call(
                me.as_ref().array_call.as_non_null().cast(),
                me.as_ref().vmctx.as_non_null(),
                caller,
                args_and_results,
            )
        }
    }

    #[inline]
    unsafe fn array_call_native(
        me: NonNull<VMFuncRef>,
        caller: NonNull<VMContext>,
        args_and_results: NonNull<[ValRaw]>,
    ) -> bool {
        unsafe {
            union GetNativePointer {
                native: VMArrayCallNative,
                ptr: NonNull<VMArrayCallFunction>,
            }
            let native = GetNativePointer {
                ptr: me.as_ref().array_call.as_non_null(),
            }
            .native;
            native(
                me.as_ref().vmctx.as_non_null(),
                caller,
                args_and_results.cast(),
                args_and_results.len(),
            )
        }
    }
}

#[cfg(test)]
mod test_vm_func_ref {
    use super::VMFuncRef;
    use core::mem::offset_of;
    use std::mem::size_of;
    use wasmtime_environ::{HostPtr, Module, PtrSize, VMOffsets};

    #[test]
    fn check_vm_func_ref_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            size_of::<VMFuncRef>(),
            usize::from(offsets.ptr.size_of_vm_func_ref())
        );
        assert_eq!(
            offset_of!(VMFuncRef, array_call),
            usize::from(offsets.ptr.vm_func_ref_array_call())
        );
        assert_eq!(
            offset_of!(VMFuncRef, wasm_call),
            usize::from(offsets.ptr.vm_func_ref_wasm_call())
        );
        assert_eq!(
            offset_of!(VMFuncRef, type_index),
            usize::from(offsets.ptr.vm_func_ref_type_index())
        );
        assert_eq!(
            offset_of!(VMFuncRef, vmctx),
            usize::from(offsets.ptr.vm_func_ref_vmctx())
        );
    }
}

macro_rules! define_builtin_array {
    (
        $(
            $( #[$attr:meta] )*
            $name:ident( $( $pname:ident: $param:ident ),* ) $( -> $result:ident )?;
        )*
    ) => {
        /// An array that stores addresses of builtin functions. We translate code
        /// to use indirect calls. This way, we don't have to patch the code.
        #[repr(C)]
        #[allow(improper_ctypes_definitions, reason = "__m128i known not FFI-safe")]
        pub struct VMBuiltinFunctionsArray {
            $(
                $name: unsafe extern "C" fn(
                    $(define_builtin_array!(@ty $param)),*
                ) $( -> define_builtin_array!(@ty $result))?,
            )*
        }

        impl VMBuiltinFunctionsArray {
            pub const INIT: VMBuiltinFunctionsArray = VMBuiltinFunctionsArray {
                $(
                    $name: crate::runtime::vm::libcalls::raw::$name,
                )*
            };

            /// Helper to call `expose_provenance()` on all contained pointers.
            ///
            /// This is required to be called at least once before entering wasm
            /// to inform the compiler that these function pointers may all be
            /// loaded/stored and used on the "other end" to reacquire
            /// provenance in Pulley. Pulley models hostcalls with a host
            /// pointer as the first parameter that's a function pointer under
            /// the hood, and this call ensures that the use of the function
            /// pointer is considered valid.
            pub fn expose_provenance(&self) -> NonNull<Self>{
                $(
                    (self.$name as *mut u8).expose_provenance();
                )*
                NonNull::from(self)
            }
        }
    };

    (@ty u32) => (u32);
    (@ty u64) => (u64);
    (@ty f32) => (f32);
    (@ty f64) => (f64);
    (@ty u8) => (u8);
    (@ty i8x16) => (i8x16);
    (@ty f32x4) => (f32x4);
    (@ty f64x2) => (f64x2);
    (@ty bool) => (bool);
    (@ty pointer) => (*mut u8);
    (@ty vmctx) => (NonNull<VMContext>);
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMBuiltinFunctionsArray {}

wasmtime_environ::foreach_builtin_function!(define_builtin_array);

const _: () = {
    assert!(
        mem::size_of::<VMBuiltinFunctionsArray>()
            == mem::size_of::<usize>() * (BuiltinFunctionIndex::len() as usize)
    )
};

/// Structure that holds all mutable context that is shared across all instances
/// in a store, for example data related to fuel or epochs.
///
/// `VMStoreContext`s are one-to-one with `wasmtime::Store`s, the same way that
/// `VMContext`s are one-to-one with `wasmtime::Instance`s. And the same way
/// that multiple `wasmtime::Instance`s may be associated with the same
/// `wasmtime::Store`, multiple `VMContext`s hold a pointer to the same
/// `VMStoreContext` when they are associated with the same `wasmtime::Store`.
#[derive(Debug)]
#[repr(C)]
pub struct VMStoreContext {
    // NB: 64-bit integer fields are located first with pointer-sized fields
    // trailing afterwards. That makes the offsets in this structure easier to
    // calculate on 32-bit platforms as we don't have to worry about the
    // alignment of 64-bit integers.
    //
    /// Indicator of how much fuel has been consumed and is remaining to
    /// WebAssembly.
    ///
    /// This field is typically negative and increments towards positive. Upon
    /// turning positive a wasm trap will be generated. This field is only
    /// modified if wasm is configured to consume fuel.
    pub fuel_consumed: UnsafeCell<i64>,

    /// Deadline epoch for interruption: if epoch-based interruption
    /// is enabled and the global (per engine) epoch counter is
    /// observed to reach or exceed this value, the guest code will
    /// yield if running asynchronously.
    pub epoch_deadline: UnsafeCell<u64>,

    /// Current stack limit of the wasm module.
    ///
    /// For more information see `crates/cranelift/src/lib.rs`.
    pub stack_limit: UnsafeCell<usize>,

    /// The `VMMemoryDefinition` for this store's GC heap.
    pub gc_heap: VMMemoryDefinition,

    /// The value of the frame pointer register when we last called from Wasm to
    /// the host.
    ///
    /// Maintained by our Wasm-to-host trampoline, and cleared just before
    /// calling into Wasm in `catch_traps`.
    ///
    /// This member is `0` when Wasm is actively running and has not called out
    /// to the host.
    ///
    /// Used to find the start of a contiguous sequence of Wasm frames when
    /// walking the stack.
    pub last_wasm_exit_fp: UnsafeCell<usize>,

    /// The last Wasm program counter before we called from Wasm to the host.
    ///
    /// Maintained by our Wasm-to-host trampoline, and cleared just before
    /// calling into Wasm in `catch_traps`.
    ///
    /// This member is `0` when Wasm is actively running and has not called out
    /// to the host.
    ///
    /// Used when walking a contiguous sequence of Wasm frames.
    pub last_wasm_exit_pc: UnsafeCell<usize>,

    /// The last host stack pointer before we called into Wasm from the host.
    ///
    /// Maintained by our host-to-Wasm trampoline, and cleared just before
    /// calling into Wasm in `catch_traps`.
    ///
    /// This member is `0` when Wasm is actively running and has not called out
    /// to the host.
    ///
    /// When a host function is wrapped into a `wasmtime::Func`, and is then
    /// called from the host, then this member has the sentinel value of `-1 as
    /// usize`, meaning that this contiguous sequence of Wasm frames is the
    /// empty sequence, and it is not safe to dereference the
    /// `last_wasm_exit_fp`.
    ///
    /// Used to find the end of a contiguous sequence of Wasm frames when
    /// walking the stack.
    pub last_wasm_entry_fp: UnsafeCell<usize>,

    /// Stack information used by stack switching instructions. See documentation
    /// on `VMStackChain` for details.
    pub stack_chain: UnsafeCell<VMStackChain>,

    /// The range, in addresses, of the guard page that is currently in use.
    ///
    /// This field is used when signal handlers are run to determine whether a
    /// faulting address lies within the guard page of an async stack for
    /// example. If this happens then the signal handler aborts with a stack
    /// overflow message similar to what would happen had the stack overflow
    /// happened on the main thread. This field is, by default a null..null
    /// range indicating that no async guard is in use (aka no fiber). In such a
    /// situation while this field is read it'll never classify a fault as an
    /// guard page fault.
    pub async_guard_range: Range<*mut u8>,
}

// The `VMStoreContext` type is a pod-type with no destructor, and we don't
// access any fields from other threads, so add in these trait impls which are
// otherwise not available due to the `fuel_consumed` and `epoch_deadline`
// variables in `VMStoreContext`.
unsafe impl Send for VMStoreContext {}
unsafe impl Sync for VMStoreContext {}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMStoreContext {}

impl Default for VMStoreContext {
    fn default() -> VMStoreContext {
        VMStoreContext {
            fuel_consumed: UnsafeCell::new(0),
            epoch_deadline: UnsafeCell::new(0),
            stack_limit: UnsafeCell::new(usize::max_value()),
            gc_heap: VMMemoryDefinition {
                base: NonNull::dangling().into(),
                current_length: AtomicUsize::new(0),
            },
            last_wasm_exit_fp: UnsafeCell::new(0),
            last_wasm_exit_pc: UnsafeCell::new(0),
            last_wasm_entry_fp: UnsafeCell::new(0),
            stack_chain: UnsafeCell::new(VMStackChain::Absent),
            async_guard_range: ptr::null_mut()..ptr::null_mut(),
        }
    }
}

#[cfg(test)]
mod test_vmstore_context {
    use super::{VMMemoryDefinition, VMStoreContext};
    use core::mem::offset_of;
    use wasmtime_environ::{HostPtr, Module, PtrSize, VMOffsets};

    #[test]
    fn field_offsets() {
        let module = Module::new();
        let offsets = VMOffsets::new(HostPtr, &module);
        assert_eq!(
            offset_of!(VMStoreContext, stack_limit),
            usize::from(offsets.ptr.vmstore_context_stack_limit())
        );
        assert_eq!(
            offset_of!(VMStoreContext, fuel_consumed),
            usize::from(offsets.ptr.vmstore_context_fuel_consumed())
        );
        assert_eq!(
            offset_of!(VMStoreContext, epoch_deadline),
            usize::from(offsets.ptr.vmstore_context_epoch_deadline())
        );
        assert_eq!(
            offset_of!(VMStoreContext, gc_heap),
            usize::from(offsets.ptr.vmstore_context_gc_heap())
        );
        assert_eq!(
            offset_of!(VMStoreContext, gc_heap) + offset_of!(VMMemoryDefinition, base),
            usize::from(offsets.ptr.vmstore_context_gc_heap_base())
        );
        assert_eq!(
            offset_of!(VMStoreContext, gc_heap) + offset_of!(VMMemoryDefinition, current_length),
            usize::from(offsets.ptr.vmstore_context_gc_heap_current_length())
        );
        assert_eq!(
            offset_of!(VMStoreContext, last_wasm_exit_fp),
            usize::from(offsets.ptr.vmstore_context_last_wasm_exit_fp())
        );
        assert_eq!(
            offset_of!(VMStoreContext, last_wasm_exit_pc),
            usize::from(offsets.ptr.vmstore_context_last_wasm_exit_pc())
        );
        assert_eq!(
            offset_of!(VMStoreContext, last_wasm_entry_fp),
            usize::from(offsets.ptr.vmstore_context_last_wasm_entry_fp())
        );
        assert_eq!(
            offset_of!(VMStoreContext, stack_chain),
            usize::from(offsets.ptr.vmstore_context_stack_chain())
        )
    }
}

/// The VM "context", which is pointed to by the `vmctx` arg in Cranelift.
/// This has information about globals, memories, tables, and other runtime
/// state associated with the current instance.
///
/// The struct here is empty, as the sizes of these fields are dynamic, and
/// we can't describe them in Rust's type system. Sufficient memory is
/// allocated at runtime.
#[derive(Debug)]
#[repr(C, align(16))] // align 16 since globals are aligned to that and contained inside
pub struct VMContext {
    _magic: u32,
}

impl VMContext {
    /// Helper function to cast between context types using a debug assertion to
    /// protect against some mistakes.
    #[inline]
    pub unsafe fn from_opaque(opaque: NonNull<VMOpaqueContext>) -> NonNull<VMContext> {
        // Note that in general the offset of the "magic" field is stored in
        // `VMOffsets::vmctx_magic`. Given though that this is a sanity check
        // about converting this pointer to another type we ideally don't want
        // to read the offset from potentially corrupt memory. Instead it would
        // be better to catch errors here as soon as possible.
        //
        // To accomplish this the `VMContext` structure is laid out with the
        // magic field at a statically known offset (here it's 0 for now). This
        // static offset is asserted in `VMOffsets::from` and needs to be kept
        // in sync with this line for this debug assertion to work.
        //
        // Also note that this magic is only ever invalid in the presence of
        // bugs, meaning we don't actually read the magic and act differently
        // at runtime depending what it is, so this is a debug assertion as
        // opposed to a regular assertion.
        unsafe {
            debug_assert_eq!(opaque.as_ref().magic, VMCONTEXT_MAGIC);
        }
        opaque.cast()
    }
}

/// A "raw" and unsafe representation of a WebAssembly value.
///
/// This is provided for use with the `Func::new_unchecked` and
/// `Func::call_unchecked` APIs. In general it's unlikely you should be using
/// this from Rust, rather using APIs like `Func::wrap` and `TypedFunc::call`.
///
/// This is notably an "unsafe" way to work with `Val` and it's recommended to
/// instead use `Val` where possible. An important note about this union is that
/// fields are all stored in little-endian format, regardless of the endianness
/// of the host system.
#[repr(C)]
#[derive(Copy, Clone)]
pub union ValRaw {
    /// A WebAssembly `i32` value.
    ///
    /// Note that the payload here is a Rust `i32` but the WebAssembly `i32`
    /// type does not assign an interpretation of the upper bit as either signed
    /// or unsigned. The Rust type `i32` is simply chosen for convenience.
    ///
    /// This value is always stored in a little-endian format.
    i32: i32,

    /// A WebAssembly `i64` value.
    ///
    /// Note that the payload here is a Rust `i64` but the WebAssembly `i64`
    /// type does not assign an interpretation of the upper bit as either signed
    /// or unsigned. The Rust type `i64` is simply chosen for convenience.
    ///
    /// This value is always stored in a little-endian format.
    i64: i64,

    /// A WebAssembly `f32` value.
    ///
    /// Note that the payload here is a Rust `u32`. This is to allow passing any
    /// representation of NaN into WebAssembly without risk of changing NaN
    /// payload bits as its gets passed around the system. Otherwise though this
    /// `u32` value is the return value of `f32::to_bits` in Rust.
    ///
    /// This value is always stored in a little-endian format.
    f32: u32,

    /// A WebAssembly `f64` value.
    ///
    /// Note that the payload here is a Rust `u64`. This is to allow passing any
    /// representation of NaN into WebAssembly without risk of changing NaN
    /// payload bits as its gets passed around the system. Otherwise though this
    /// `u64` value is the return value of `f64::to_bits` in Rust.
    ///
    /// This value is always stored in a little-endian format.
    f64: u64,

    /// A WebAssembly `v128` value.
    ///
    /// The payload here is a Rust `[u8; 16]` which has the same number of bits
    /// but note that `v128` in WebAssembly is often considered a vector type
    /// such as `i32x4` or `f64x2`. This means that the actual interpretation
    /// of the underlying bits is left up to the instructions which consume
    /// this value.
    ///
    /// This value is always stored in a little-endian format.
    v128: [u8; 16],

    /// A WebAssembly `funcref` value (or one of its subtypes).
    ///
    /// The payload here is a pointer which is runtime-defined. This is one of
    /// the main points of unsafety about the `ValRaw` type as the validity of
    /// the pointer here is not easily verified and must be preserved by
    /// carefully calling the correct functions throughout the runtime.
    ///
    /// This value is always stored in a little-endian format.
    funcref: *mut c_void,

    /// A WebAssembly `externref` value (or one of its subtypes).
    ///
    /// The payload here is a compressed pointer value which is
    /// runtime-defined. This is one of the main points of unsafety about the
    /// `ValRaw` type as the validity of the pointer here is not easily verified
    /// and must be preserved by carefully calling the correct functions
    /// throughout the runtime.
    ///
    /// This value is always stored in a little-endian format.
    externref: u32,

    /// A WebAssembly `anyref` value (or one of its subtypes).
    ///
    /// The payload here is a compressed pointer value which is
    /// runtime-defined. This is one of the main points of unsafety about the
    /// `ValRaw` type as the validity of the pointer here is not easily verified
    /// and must be preserved by carefully calling the correct functions
    /// throughout the runtime.
    ///
    /// This value is always stored in a little-endian format.
    anyref: u32,

    /// A WebAssembly `exnref` value (or one of its subtypes).
    ///
    /// The payload here is a compressed pointer value which is
    /// runtime-defined. This is one of the main points of unsafety about the
    /// `ValRaw` type as the validity of the pointer here is not easily verified
    /// and must be preserved by carefully calling the correct functions
    /// throughout the runtime.
    ///
    /// This value is always stored in a little-endian format.
    exnref: u32,
}

// The `ValRaw` type is matched as `wasmtime_val_raw_t` in the C API so these
// are some simple assertions about the shape of the type which are additionally
// matched in C.
const _: () = {
    assert!(mem::size_of::<ValRaw>() == 16);
    assert!(mem::align_of::<ValRaw>() == mem::align_of::<u64>());
};

// This type is just a bag-of-bits so it's up to the caller to figure out how
// to safely deal with threading concerns and safely access interior bits.
unsafe impl Send for ValRaw {}
unsafe impl Sync for ValRaw {}

impl fmt::Debug for ValRaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Hex<T>(T);
        impl<T: fmt::LowerHex> fmt::Debug for Hex<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let bytes = mem::size_of::<T>();
                let hex_digits_per_byte = 2;
                let hex_digits = bytes * hex_digits_per_byte;
                write!(f, "0x{:0width$x}", self.0, width = hex_digits)
            }
        }

        unsafe {
            f.debug_struct("ValRaw")
                .field("i32", &Hex(self.i32))
                .field("i64", &Hex(self.i64))
                .field("f32", &Hex(self.f32))
                .field("f64", &Hex(self.f64))
                .field("v128", &Hex(u128::from_le_bytes(self.v128)))
                .field("funcref", &self.funcref)
                .field("externref", &Hex(self.externref))
                .field("anyref", &Hex(self.anyref))
                .field("exnref", &Hex(self.exnref))
                .finish()
        }
    }
}

impl ValRaw {
    /// Create a null reference that is compatible with any of
    /// `{any,extern,func,exn}ref`.
    pub fn null() -> ValRaw {
        unsafe {
            let raw = mem::MaybeUninit::<Self>::zeroed().assume_init();
            debug_assert_eq!(raw.get_anyref(), 0);
            debug_assert_eq!(raw.get_exnref(), 0);
            debug_assert_eq!(raw.get_externref(), 0);
            debug_assert_eq!(raw.get_funcref(), ptr::null_mut());
            raw
        }
    }

    /// Creates a WebAssembly `i32` value
    #[inline]
    pub fn i32(i: i32) -> ValRaw {
        // Note that this is intentionally not setting the `i32` field, instead
        // setting the `i64` field with a zero-extended version of `i`. For more
        // information on this see the comments on `Lower for Result` in the
        // `wasmtime` crate. Otherwise though all `ValRaw` constructors are
        // otherwise constrained to guarantee that the initial 64-bits are
        // always initialized.
        ValRaw::u64(i.unsigned().into())
    }

    /// Creates a WebAssembly `i64` value
    #[inline]
    pub fn i64(i: i64) -> ValRaw {
        ValRaw { i64: i.to_le() }
    }

    /// Creates a WebAssembly `i32` value
    #[inline]
    pub fn u32(i: u32) -> ValRaw {
        // See comments in `ValRaw::i32` for why this is setting the upper
        // 32-bits as well.
        ValRaw::u64(i.into())
    }

    /// Creates a WebAssembly `i64` value
    #[inline]
    pub fn u64(i: u64) -> ValRaw {
        ValRaw::i64(i as i64)
    }

    /// Creates a WebAssembly `f32` value
    #[inline]
    pub fn f32(i: u32) -> ValRaw {
        // See comments in `ValRaw::i32` for why this is setting the upper
        // 32-bits as well.
        ValRaw::u64(i.into())
    }

    /// Creates a WebAssembly `f64` value
    #[inline]
    pub fn f64(i: u64) -> ValRaw {
        ValRaw { f64: i.to_le() }
    }

    /// Creates a WebAssembly `v128` value
    #[inline]
    pub fn v128(i: u128) -> ValRaw {
        ValRaw {
            v128: i.to_le_bytes(),
        }
    }

    /// Creates a WebAssembly `funcref` value
    #[inline]
    pub fn funcref(i: *mut c_void) -> ValRaw {
        ValRaw {
            funcref: i.map_addr(|i| i.to_le()),
        }
    }

    /// Creates a WebAssembly `externref` value
    #[inline]
    pub fn externref(e: u32) -> ValRaw {
        assert!(cfg!(feature = "gc") || e == 0);
        ValRaw {
            externref: e.to_le(),
        }
    }

    /// Creates a WebAssembly `anyref` value
    #[inline]
    pub fn anyref(r: u32) -> ValRaw {
        assert!(cfg!(feature = "gc") || r == 0);
        ValRaw { anyref: r.to_le() }
    }

    /// Creates a WebAssembly `exnref` value
    #[inline]
    pub fn exnref(r: u32) -> ValRaw {
        assert!(cfg!(feature = "gc") || r == 0);
        ValRaw { exnref: r.to_le() }
    }

    /// Gets the WebAssembly `i32` value
    #[inline]
    pub fn get_i32(&self) -> i32 {
        unsafe { i32::from_le(self.i32) }
    }

    /// Gets the WebAssembly `i64` value
    #[inline]
    pub fn get_i64(&self) -> i64 {
        unsafe { i64::from_le(self.i64) }
    }

    /// Gets the WebAssembly `i32` value
    #[inline]
    pub fn get_u32(&self) -> u32 {
        self.get_i32().unsigned()
    }

    /// Gets the WebAssembly `i64` value
    #[inline]
    pub fn get_u64(&self) -> u64 {
        self.get_i64().unsigned()
    }

    /// Gets the WebAssembly `f32` value
    #[inline]
    pub fn get_f32(&self) -> u32 {
        unsafe { u32::from_le(self.f32) }
    }

    /// Gets the WebAssembly `f64` value
    #[inline]
    pub fn get_f64(&self) -> u64 {
        unsafe { u64::from_le(self.f64) }
    }

    /// Gets the WebAssembly `v128` value
    #[inline]
    pub fn get_v128(&self) -> u128 {
        unsafe { u128::from_le_bytes(self.v128) }
    }

    /// Gets the WebAssembly `funcref` value
    #[inline]
    pub fn get_funcref(&self) -> *mut c_void {
        unsafe { self.funcref.map_addr(|i| usize::from_le(i)) }
    }

    /// Gets the WebAssembly `externref` value
    #[inline]
    pub fn get_externref(&self) -> u32 {
        let externref = u32::from_le(unsafe { self.externref });
        assert!(cfg!(feature = "gc") || externref == 0);
        externref
    }

    /// Gets the WebAssembly `anyref` value
    #[inline]
    pub fn get_anyref(&self) -> u32 {
        let anyref = u32::from_le(unsafe { self.anyref });
        assert!(cfg!(feature = "gc") || anyref == 0);
        anyref
    }

    /// Gets the WebAssembly `exnref` value
    #[inline]
    pub fn get_exnref(&self) -> u32 {
        let exnref = u32::from_le(unsafe { self.exnref });
        assert!(cfg!(feature = "gc") || exnref == 0);
        exnref
    }
}

/// An "opaque" version of `VMContext` which must be explicitly casted to a
/// target context.
///
/// This context is used to represent that contexts specified in
/// `VMFuncRef` can have any type and don't have an implicit
/// structure. Neither wasmtime nor cranelift-generated code can rely on the
/// structure of an opaque context in general and only the code which configured
/// the context is able to rely on a particular structure. This is because the
/// context pointer configured for `VMFuncRef` is guaranteed to be
/// the first parameter passed.
///
/// Note that Wasmtime currently has a layout where all contexts that are casted
/// to an opaque context start with a 32-bit "magic" which can be used in debug
/// mode to debug-assert that the casts here are correct and have at least a
/// little protection against incorrect casts.
pub struct VMOpaqueContext {
    pub(crate) magic: u32,
    _marker: marker::PhantomPinned,
}

impl VMOpaqueContext {
    /// Helper function to clearly indicate that casts are desired.
    #[inline]
    pub fn from_vmcontext(ptr: NonNull<VMContext>) -> NonNull<VMOpaqueContext> {
        ptr.cast()
    }

    /// Helper function to clearly indicate that casts are desired.
    #[inline]
    pub fn from_vm_array_call_host_func_context(
        ptr: NonNull<VMArrayCallHostFuncContext>,
    ) -> NonNull<VMOpaqueContext> {
        ptr.cast()
    }
}
