//! Runtime library calls.
//!
//! Note that Wasm compilers may sometimes perform these inline rather than
//! calling them, particularly when CPUs have special instructions which compute
//! them directly.
//!
//! These functions are called by compiled Wasm code, and therefore must take
//! certain care about some things:
//!
//! * They must only contain basic, raw i32/i64/f32/f64/pointer parameters that
//!   are safe to pass across the system ABI.
//!
//! * If any nested function propagates an `Err(trap)` out to the library
//!   function frame, we need to raise it. This involves some nasty and quite
//!   unsafe code under the covers! Notably, after raising the trap, drops
//!   **will not** be run for local variables! This can lead to things like
//!   leaking `InstanceHandle`s which leads to never deallocating JIT code,
//!   instances, and modules if we are not careful!
//!
//! * The libcall must be entered via a Wasm-to-libcall trampoline that saves
//!   the last Wasm FP and PC for stack walking purposes. (For more details, see
//!   `crates/wasmtime/src/runtime/vm/backtrace.rs`.)
//!
//! To make it easier to correctly handle all these things, **all** libcalls
//! must be defined via the `libcall!` helper macro! See its doc comments below
//! for an example, or just look at the rest of the file.
//!
//! ## Dealing with `externref`s
//!
//! When receiving a raw `*mut u8` that is actually a `VMExternRef` reference,
//! convert it into a proper `VMExternRef` with `VMExternRef::clone_from_raw` as
//! soon as apossible. Any GC before raw pointer is converted into a reference
//! can potentially collect the referenced object, which could lead to use after
//! free.
//!
//! Avoid this by eagerly converting into a proper `VMExternRef`! (Unfortunately
//! there is no macro to help us automatically get this correct, so stay
//! vigilant!)
//!
//! ```ignore
//! pub unsafe extern "C" my_libcall_takes_ref(raw_extern_ref: *mut u8) {
//!     // Before `clone_from_raw`, `raw_extern_ref` is potentially unrooted,
//!     // and doing GC here could lead to use after free!
//!
//!     let my_extern_ref = if raw_extern_ref.is_null() {
//!         None
//!     } else {
//!         Some(VMExternRef::clone_from_raw(raw_extern_ref))
//!     };
//!
//!     // Now that we did `clone_from_raw`, it is safe to do a GC (or do
//!     // anything else that might transitively GC, like call back into
//!     // Wasm!)
//! }
//! ```

#[cfg(feature = "stack-switching")]
use super::stack_switching::VMContObj;
use crate::prelude::*;
use crate::runtime::store::StoreInstanceId;
#[cfg(feature = "gc")]
use crate::runtime::vm::VMGcRef;
use crate::runtime::vm::table::TableElementType;
use crate::runtime::vm::vmcontext::VMFuncRef;
use crate::runtime::vm::{
    HostResultHasUnwindSentinel, Instance, TrapReason, VMStore, f32x4, f64x2, i8x16,
};
use core::convert::Infallible;
use core::pin::Pin;
use core::ptr::NonNull;
#[cfg(feature = "threads")]
use core::time::Duration;
use wasmtime_environ::{
    DataIndex, DefinedMemoryIndex, DefinedTableIndex, ElemIndex, FuncIndex, MemoryIndex,
    TableIndex, Trap,
};
#[cfg(feature = "wmemcheck")]
use wasmtime_wmemcheck::AccessError::{
    DoubleMalloc, InvalidFree, InvalidRead, InvalidWrite, OutOfBounds,
};

/// Raw functions which are actually called from compiled code.
///
/// Invocation of a builtin currently looks like:
///
/// * A wasm function calls a cranelift-compiled trampoline that's generated
///   once-per-builtin.
/// * The cranelift-compiled trampoline performs any necessary actions to exit
///   wasm, such as dealing with fp/pc/etc.
/// * The cranelift-compiled trampoline loads a function pointer from an array
///   stored in `VMContext` That function pointer is defined in this module.
/// * This module runs, handling things like `catch_unwind` and `Result` and
///   such.
/// * This module delegates to the outer module (this file) which has the actual
///   implementation.
///
/// For more information on converting from host-defined values to Cranelift ABI
/// values see the `catch_unwind_and_record_trap` function.
pub mod raw {
    use crate::runtime::vm::{InstanceAndStore, VMContext, f32x4, f64x2, i8x16};
    use core::ptr::NonNull;

    macro_rules! libcall {
        (
            $(
                $( #[cfg($attr:meta)] )?
                $name:ident( vmctx: vmctx $(, $pname:ident: $param:ident )* ) $(-> $result:ident)?;
            )*
        ) => {
            $(
                // This is the direct entrypoint from the compiled module which
                // still has the raw signature.
                //
                // This will delegate to the outer module to the actual
                // implementation and automatically perform `catch_unwind` along
                // with conversion of the return value in the face of traps.
                #[allow(improper_ctypes_definitions, reason = "__m128i known not FFI-safe")]
                pub unsafe extern "C" fn $name(
                    vmctx: NonNull<VMContext>,
                    $( $pname : libcall!(@ty $param), )*
                ) $(-> libcall!(@ty $result))? {
                    $(#[cfg($attr)])?
                    {
                        crate::runtime::vm::traphandlers::catch_unwind_and_record_trap(|| unsafe {
                            InstanceAndStore::from_vmctx(vmctx, |pair| {
                                let (instance, store) = pair.unpack_mut();
                                super::$name(store, instance, $($pname),*)
                            })
                        })
                    }
                    $(
                        #[cfg(not($attr))]
                        {
                            let _ = vmctx;
                            unreachable!();
                        }
                    )?
                }

                // This works around a `rustc` bug where compiling with LTO
                // will sometimes strip out some of these symbols resulting
                // in a linking failure.
                #[allow(improper_ctypes_definitions, reason = "__m128i known not FFI-safe")]
                const _: () = {
                    #[used]
                    static I_AM_USED: unsafe extern "C" fn(
                        NonNull<VMContext>,
                        $( $pname : libcall!(@ty $param), )*
                    ) $( -> libcall!(@ty $result))? = $name;
                };
            )*
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
    }

    wasmtime_environ::foreach_builtin_function!(libcall);
}

fn memory_grow(
    store: &mut dyn VMStore,
    mut instance: Pin<&mut Instance>,
    delta: u64,
    memory_index: u32,
) -> Result<Option<AllocationSize>, TrapReason> {
    let memory_index = DefinedMemoryIndex::from_u32(memory_index);
    let module = instance.env_module();
    let page_size_log2 = module.memories[module.memory_index(memory_index)].page_size_log2;

    let result = instance
        .as_mut()
        .memory_grow(store, memory_index, delta)?
        .map(|size_in_bytes| AllocationSize(size_in_bytes >> page_size_log2));

    Ok(result)
}

/// A helper structure to represent the return value of a memory or table growth
/// call.
///
/// This represents a byte or element-based count of the size of an item on the
/// host. For example a memory is how many bytes large the memory is, or a table
/// is how many elements large it is. It's assumed that the value here is never
/// -1 or -2 as that would mean the entire host address space is allocated which
/// is not possible.
struct AllocationSize(usize);

/// Special implementation for growth-related libcalls.
///
/// Here the optional return value means:
///
/// * `Some(val)` - the growth succeeded and the previous size of the item was
///   `val`.
/// * `None` - the growth failed.
///
/// The failure case returns -1 (or `usize::MAX` as an unsigned integer) and the
/// successful case returns the `val` itself. Note that -2 (`usize::MAX - 1`
/// when unsigned) is unwind as a sentinel to indicate an unwind as no valid
/// allocation can be that large.
unsafe impl HostResultHasUnwindSentinel for Option<AllocationSize> {
    type Abi = *mut u8;
    const SENTINEL: *mut u8 = (usize::MAX - 1) as *mut u8;

    fn into_abi(self) -> *mut u8 {
        match self {
            Some(size) => {
                debug_assert!(size.0 < (usize::MAX - 1));
                size.0 as *mut u8
            }
            None => usize::MAX as *mut u8,
        }
    }
}

/// Implementation of `table.grow` for `funcref` tables.
unsafe fn table_grow_func_ref(
    store: &mut dyn VMStore,
    mut instance: Pin<&mut Instance>,
    defined_table_index: u32,
    delta: u64,
    init_value: *mut u8,
) -> Result<Option<AllocationSize>> {
    let defined_table_index = DefinedTableIndex::from_u32(defined_table_index);
    let table_index = instance.env_module().table_index(defined_table_index);
    debug_assert!(matches!(
        instance.as_mut().table_element_type(table_index),
        TableElementType::Func,
    ));
    let element = NonNull::new(init_value.cast::<VMFuncRef>()).into();
    let result = instance
        .defined_table_grow(store, defined_table_index, delta, element)?
        .map(AllocationSize);
    Ok(result)
}

/// Implementation of `table.grow` for GC-reference tables.
#[cfg(feature = "gc")]
unsafe fn table_grow_gc_ref(
    store: &mut dyn VMStore,
    mut instance: Pin<&mut Instance>,
    defined_table_index: u32,
    delta: u64,
    init_value: u32,
) -> Result<Option<AllocationSize>> {
    let defined_table_index = DefinedTableIndex::from_u32(defined_table_index);
    let table_index = instance.env_module().table_index(defined_table_index);
    debug_assert!(matches!(
        instance.as_mut().table_element_type(table_index),
        TableElementType::GcRef,
    ));

    let element = VMGcRef::from_raw_u32(init_value)
        .map(|r| {
            store
                .store_opaque_mut()
                .unwrap_gc_store_mut()
                .clone_gc_ref(&r)
        })
        .into();

    let result = instance
        .defined_table_grow(store, defined_table_index, delta, element)?
        .map(AllocationSize);
    Ok(result)
}

#[cfg(feature = "stack-switching")]
unsafe fn table_grow_cont_obj(
    store: &mut dyn VMStore,
    mut instance: Pin<&mut Instance>,
    defined_table_index: u32,
    delta: u64,
    // The following two values together form the initial Option<VMContObj>.
    // A None value is indicated by the pointer being null.
    init_value_contref: *mut u8,
    init_value_revision: u64,
) -> Result<Option<AllocationSize>> {
    let defined_table_index = DefinedTableIndex::from_u32(defined_table_index);
    let table_index = instance.env_module().table_index(defined_table_index);
    debug_assert!(matches!(
        instance.as_mut().table_element_type(table_index),
        TableElementType::Cont,
    ));
    let element =
        unsafe { VMContObj::from_raw_parts(init_value_contref, init_value_revision).into() };
    let result = instance
        .defined_table_grow(store, defined_table_index, delta, element)?
        .map(AllocationSize);
    Ok(result)
}

/// Implementation of `table.fill` for `funcref`s.
unsafe fn table_fill_func_ref(
    store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    table_index: u32,
    dst: u64,
    val: *mut u8,
    len: u64,
) -> Result<()> {
    let table_index = DefinedTableIndex::from_u32(table_index);
    let table = instance.get_defined_table(table_index);
    match table.element_type() {
        TableElementType::Func => {
            let val = NonNull::new(val.cast::<VMFuncRef>());
            table.fill(store.optional_gc_store_mut(), dst, val.into(), len)?;
            Ok(())
        }
        TableElementType::GcRef => unreachable!(),
        TableElementType::Cont => unreachable!(),
    }
}

#[cfg(feature = "gc")]
unsafe fn table_fill_gc_ref(
    store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    table_index: u32,
    dst: u64,
    val: u32,
    len: u64,
) -> Result<()> {
    let table_index = DefinedTableIndex::from_u32(table_index);
    let table = instance.get_defined_table(table_index);
    match table.element_type() {
        TableElementType::Func => unreachable!(),
        TableElementType::GcRef => {
            let gc_store = store.store_opaque_mut().unwrap_gc_store_mut();
            let gc_ref = VMGcRef::from_raw_u32(val);
            let gc_ref = gc_ref.map(|r| gc_store.clone_gc_ref(&r));
            table.fill(Some(gc_store), dst, gc_ref.into(), len)?;
            Ok(())
        }

        TableElementType::Cont => unreachable!(),
    }
}

#[cfg(feature = "stack-switching")]
unsafe fn table_fill_cont_obj(
    store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    table_index: u32,
    dst: u64,
    value_contref: *mut u8,
    value_revision: u64,
    len: u64,
) -> Result<()> {
    let table_index = DefinedTableIndex::from_u32(table_index);
    let table = instance.get_defined_table(table_index);
    match table.element_type() {
        TableElementType::Cont => {
            let contobj = unsafe { VMContObj::from_raw_parts(value_contref, value_revision) };
            table.fill(store.optional_gc_store_mut(), dst, contobj.into(), len)?;
            Ok(())
        }
        _ => panic!("Wrong table filling function"),
    }
}

// Implementation of `table.copy`.
fn table_copy(
    store: &mut dyn VMStore,
    mut instance: Pin<&mut Instance>,
    dst_table_index: u32,
    src_table_index: u32,
    dst: u64,
    src: u64,
    len: u64,
) -> Result<(), Trap> {
    let dst_table_index = TableIndex::from_u32(dst_table_index);
    let src_table_index = TableIndex::from_u32(src_table_index);
    let store = store.store_opaque_mut();

    // Convert the two table indices relative to `instance` into two
    // defining instances and the defined table index within that instance.
    let (dst_def_index, dst_instance) = instance
        .as_mut()
        .defined_table_index_and_instance(dst_table_index);
    let dst_instance_id = dst_instance.id();
    let (src_def_index, src_instance) = instance
        .as_mut()
        .defined_table_index_and_instance(src_table_index);
    let src_instance_id = src_instance.id();

    let src_table = crate::Table::from_raw(
        StoreInstanceId::new(store.id(), src_instance_id),
        src_def_index,
    );
    let dst_table = crate::Table::from_raw(
        StoreInstanceId::new(store.id(), dst_instance_id),
        dst_def_index,
    );

    // SAFETY: this is only safe if the two tables have the same type, and that
    // was validated during wasm-validation time.
    unsafe { crate::Table::copy_raw(store, &dst_table, dst, &src_table, src, len) }
}

// Implementation of `table.init`.
fn table_init(
    store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    table_index: u32,
    elem_index: u32,
    dst: u64,
    src: u64,
    len: u64,
) -> Result<(), Trap> {
    let table_index = TableIndex::from_u32(table_index);
    let elem_index = ElemIndex::from_u32(elem_index);
    instance.table_init(
        store.store_opaque_mut(),
        table_index,
        elem_index,
        dst,
        src,
        len,
    )
}

// Implementation of `elem.drop`.
fn elem_drop(_store: &mut dyn VMStore, instance: Pin<&mut Instance>, elem_index: u32) {
    let elem_index = ElemIndex::from_u32(elem_index);
    instance.elem_drop(elem_index)
}

// Implementation of `memory.copy`.
fn memory_copy(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    dst_index: u32,
    dst: u64,
    src_index: u32,
    src: u64,
    len: u64,
) -> Result<(), Trap> {
    let src_index = MemoryIndex::from_u32(src_index);
    let dst_index = MemoryIndex::from_u32(dst_index);
    instance.memory_copy(dst_index, dst, src_index, src, len)
}

// Implementation of `memory.fill` for locally defined memories.
fn memory_fill(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    memory_index: u32,
    dst: u64,
    val: u32,
    len: u64,
) -> Result<(), Trap> {
    let memory_index = DefinedMemoryIndex::from_u32(memory_index);
    #[expect(clippy::cast_possible_truncation, reason = "known to truncate here")]
    instance.memory_fill(memory_index, dst, val as u8, len)
}

// Implementation of `memory.init`.
fn memory_init(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    memory_index: u32,
    data_index: u32,
    dst: u64,
    src: u32,
    len: u32,
) -> Result<(), Trap> {
    let memory_index = MemoryIndex::from_u32(memory_index);
    let data_index = DataIndex::from_u32(data_index);
    instance.memory_init(memory_index, data_index, dst, src, len)
}

// Implementation of `ref.func`.
fn ref_func(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    func_index: u32,
) -> NonNull<u8> {
    instance
        .get_func_ref(FuncIndex::from_u32(func_index))
        .expect("ref_func: funcref should always be available for given func index")
        .cast()
}

// Implementation of `data.drop`.
fn data_drop(_store: &mut dyn VMStore, instance: Pin<&mut Instance>, data_index: u32) {
    let data_index = DataIndex::from_u32(data_index);
    instance.data_drop(data_index)
}

// Returns a table entry after lazily initializing it.
unsafe fn table_get_lazy_init_func_ref(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    table_index: u32,
    index: u64,
) -> *mut u8 {
    let table_index = TableIndex::from_u32(table_index);
    let table = instance.get_table_with_lazy_init(table_index, core::iter::once(index));
    let elem = (*table)
        .get(None, index)
        .expect("table access already bounds-checked");

    match unsafe { elem.into_func_ref_asserting_initialized() } {
        Some(ptr) => ptr.as_ptr().cast(),
        None => core::ptr::null_mut(),
    }
}

/// Drop a GC reference.
#[cfg(feature = "gc-drc")]
unsafe fn drop_gc_ref(store: &mut dyn VMStore, _instance: Pin<&mut Instance>, gc_ref: u32) {
    log::trace!("libcalls::drop_gc_ref({gc_ref:#x})");
    let gc_ref = VMGcRef::from_raw_u32(gc_ref).expect("non-null VMGcRef");
    store
        .store_opaque_mut()
        .unwrap_gc_store_mut()
        .drop_gc_ref(gc_ref);
}

/// Grow the GC heap.
#[cfg(feature = "gc-null")]
unsafe fn grow_gc_heap(
    store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    bytes_needed: u64,
) -> Result<()> {
    let orig_len = u64::try_from(store.gc_store()?.gc_heap.vmmemory().current_length()).unwrap();

    unsafe {
        store
            .maybe_async_gc(None, Some(bytes_needed))
            .context("failed to grow the GC heap")
            .context(crate::Trap::AllocationTooLarge)?;
    }

    // JIT code relies on the memory having grown by `bytes_needed` bytes if
    // this libcall returns successfully, so trap if we didn't grow that much.
    let new_len = u64::try_from(store.gc_store()?.gc_heap.vmmemory().current_length()).unwrap();
    if orig_len
        .checked_add(bytes_needed)
        .is_none_or(|expected_len| new_len < expected_len)
    {
        return Err(crate::Trap::AllocationTooLarge.into());
    }

    Ok(())
}

/// Allocate a raw, unininitialized GC object for Wasm code.
///
/// The Wasm code is responsible for initializing the object.
#[cfg(feature = "gc-drc")]
unsafe fn gc_alloc_raw(
    store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    kind_and_reserved: u32,
    module_interned_type_index: u32,
    size: u32,
    align: u32,
) -> Result<core::num::NonZeroU32> {
    use crate::vm::VMGcHeader;
    use core::alloc::Layout;
    use wasmtime_environ::{ModuleInternedTypeIndex, VMGcKind};

    let kind = VMGcKind::from_high_bits_of_u32(kind_and_reserved);
    log::trace!("gc_alloc_raw(kind={kind:?}, size={size}, align={align})");

    let module = instance
        .runtime_module()
        .expect("should never allocate GC types defined in a dummy module");

    let module_interned_type_index = ModuleInternedTypeIndex::from_u32(module_interned_type_index);
    let shared_type_index = module
        .signatures()
        .shared_type(module_interned_type_index)
        .expect("should have engine type index for module type index");

    let mut header = VMGcHeader::from_kind_and_index(kind, shared_type_index);
    header.set_reserved_u26(kind_and_reserved & VMGcKind::UNUSED_MASK);

    let size = usize::try_from(size).unwrap();
    let align = usize::try_from(align).unwrap();
    assert!(align.is_power_of_two());
    let layout = Layout::from_size_align(size, align).map_err(|e| {
        let err = Error::from(crate::Trap::AllocationTooLarge);
        err.context(e)
    })?;

    let store = store.store_opaque_mut();
    let gc_ref = unsafe {
        store.retry_after_gc_maybe_async((), |store, ()| {
            store
                .unwrap_gc_store_mut()
                .alloc_raw(header, layout)?
                .map_err(|bytes_needed| crate::GcHeapOutOfMemory::new((), bytes_needed).into())
        })?
    };

    let raw = store.unwrap_gc_store_mut().expose_gc_ref_to_wasm(gc_ref);
    Ok(raw)
}

// Intern a `funcref` into the GC heap, returning its `FuncRefTableId`.
//
// This libcall may not GC.
#[cfg(feature = "gc")]
unsafe fn intern_func_ref_for_gc_heap(
    store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    func_ref: *mut u8,
) -> Result<u32> {
    use crate::{store::AutoAssertNoGc, vm::SendSyncPtr};
    use core::ptr::NonNull;

    let mut store = AutoAssertNoGc::new(store.store_opaque_mut());

    let func_ref = func_ref.cast::<VMFuncRef>();
    let func_ref = NonNull::new(func_ref).map(SendSyncPtr::new);

    let func_ref_id = unsafe { store.gc_store_mut()?.func_ref_table.intern(func_ref) };
    Ok(func_ref_id.into_raw())
}

// Get the raw `VMFuncRef` pointer associated with a `FuncRefTableId` from an
// earlier `intern_func_ref_for_gc_heap` call.
//
// This libcall may not GC.
#[cfg(feature = "gc")]
unsafe fn get_interned_func_ref(
    store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    func_ref_id: u32,
    module_interned_type_index: u32,
) -> *mut u8 {
    use super::FuncRefTableId;
    use crate::store::AutoAssertNoGc;
    use wasmtime_environ::{ModuleInternedTypeIndex, packed_option::ReservedValue};

    let store = AutoAssertNoGc::new(store.store_opaque_mut());

    let func_ref_id = FuncRefTableId::from_raw(func_ref_id);
    let module_interned_type_index = ModuleInternedTypeIndex::from_bits(module_interned_type_index);

    let func_ref = if module_interned_type_index.is_reserved_value() {
        store
            .unwrap_gc_store()
            .func_ref_table
            .get_untyped(func_ref_id)
    } else {
        let types = store.engine().signatures();
        let engine_ty = instance.engine_type_index(module_interned_type_index);
        store
            .unwrap_gc_store()
            .func_ref_table
            .get_typed(types, func_ref_id, engine_ty)
    };

    func_ref.map_or(core::ptr::null_mut(), |f| f.as_ptr().cast())
}

/// Implementation of the `array.new_data` instruction.
#[cfg(feature = "gc")]
unsafe fn array_new_data(
    store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    array_type_index: u32,
    data_index: u32,
    src: u32,
    len: u32,
) -> Result<core::num::NonZeroU32> {
    use crate::ArrayType;
    use wasmtime_environ::ModuleInternedTypeIndex;

    let store = store.store_opaque_mut();
    let array_type_index = ModuleInternedTypeIndex::from_u32(array_type_index);
    let data_index = DataIndex::from_u32(data_index);

    // Calculate the byte-length of the data (as opposed to the element-length
    // of the array).
    let data_range = instance.wasm_data_range(data_index);
    let shared_ty = instance.engine_type_index(array_type_index);
    let array_ty = ArrayType::from_shared_type_index(store.engine(), shared_ty);
    let one_elem_size = array_ty
        .element_type()
        .data_byte_size()
        .expect("Wasm validation ensures that this type have a defined byte size");
    let byte_len = len
        .checked_mul(one_elem_size)
        .and_then(|x| usize::try_from(x).ok())
        .ok_or_else(|| Trap::MemoryOutOfBounds)?;

    // Get the data from the segment, checking bounds.
    let src = usize::try_from(src).map_err(|_| Trap::MemoryOutOfBounds)?;
    let data = instance
        .wasm_data(data_range)
        .get(src..)
        .and_then(|d| d.get(..byte_len))
        .ok_or_else(|| Trap::MemoryOutOfBounds)?;

    // Allocate the (uninitialized) array.
    let gc_layout = store
        .engine()
        .signatures()
        .layout(shared_ty)
        .expect("array types have GC layouts");
    let array_layout = gc_layout.unwrap_array();
    let array_ref = unsafe {
        store.retry_after_gc_maybe_async((), |store, ()| {
            store
                .unwrap_gc_store_mut()
                .alloc_uninit_array(shared_ty, len, &array_layout)?
                .map_err(|bytes_needed| crate::GcHeapOutOfMemory::new((), bytes_needed).into())
        })?
    };

    // Copy the data into the array, initializing it.
    store
        .unwrap_gc_store_mut()
        .gc_object_data(array_ref.as_gc_ref())
        .copy_from_slice(array_layout.base_size, data);

    // Return the array to Wasm!
    let raw = store
        .unwrap_gc_store_mut()
        .expose_gc_ref_to_wasm(array_ref.into());
    Ok(raw)
}

/// Implementation of the `array.init_data` instruction.
#[cfg(feature = "gc")]
unsafe fn array_init_data(
    store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    array_type_index: u32,
    array: u32,
    dst: u32,
    data_index: u32,
    src: u32,
    len: u32,
) -> Result<()> {
    use crate::ArrayType;
    use wasmtime_environ::ModuleInternedTypeIndex;

    let array_type_index = ModuleInternedTypeIndex::from_u32(array_type_index);
    let data_index = DataIndex::from_u32(data_index);

    log::trace!(
        "array.init_data(array={array:#x}, dst={dst}, data_index={data_index:?}, src={src}, len={len})",
    );

    // Null check the array.
    let gc_ref = VMGcRef::from_raw_u32(array).ok_or_else(|| Trap::NullReference)?;
    let array = gc_ref
        .into_arrayref(&*store.unwrap_gc_store().gc_heap)
        .expect("gc ref should be an array");

    let dst = usize::try_from(dst).map_err(|_| Trap::MemoryOutOfBounds)?;
    let src = usize::try_from(src).map_err(|_| Trap::MemoryOutOfBounds)?;
    let len = usize::try_from(len).map_err(|_| Trap::MemoryOutOfBounds)?;

    // Bounds check the array.
    let array_len = array.len(store.store_opaque());
    let array_len = usize::try_from(array_len).map_err(|_| Trap::ArrayOutOfBounds)?;
    if dst.checked_add(len).ok_or_else(|| Trap::ArrayOutOfBounds)? > array_len {
        return Err(Trap::ArrayOutOfBounds.into());
    }

    // Calculate the byte length from the array length.
    let shared_ty = instance.engine_type_index(array_type_index);
    let array_ty = ArrayType::from_shared_type_index(store.engine(), shared_ty);
    let one_elem_size = array_ty
        .element_type()
        .data_byte_size()
        .expect("Wasm validation ensures that this type have a defined byte size");
    let data_len = len
        .checked_mul(usize::try_from(one_elem_size).unwrap())
        .ok_or_else(|| Trap::MemoryOutOfBounds)?;

    // Get the data from the segment, checking its bounds.
    let data_range = instance.wasm_data_range(data_index);
    let data = instance
        .wasm_data(data_range)
        .get(src..)
        .and_then(|d| d.get(..data_len))
        .ok_or_else(|| Trap::MemoryOutOfBounds)?;

    // Copy the data into the array.

    let dst_offset = u32::try_from(dst)
        .unwrap()
        .checked_mul(one_elem_size)
        .unwrap();

    let array_layout = store
        .engine()
        .signatures()
        .layout(shared_ty)
        .expect("array types have GC layouts");
    let array_layout = array_layout.unwrap_array();

    let obj_offset = array_layout.base_size.checked_add(dst_offset).unwrap();

    store
        .unwrap_gc_store_mut()
        .gc_object_data(array.as_gc_ref())
        .copy_from_slice(obj_offset, data);

    Ok(())
}

#[cfg(feature = "gc")]
unsafe fn array_new_elem(
    store: &mut dyn VMStore,
    mut instance: Pin<&mut Instance>,
    array_type_index: u32,
    elem_index: u32,
    src: u32,
    len: u32,
) -> Result<core::num::NonZeroU32> {
    use crate::{
        ArrayRef, ArrayRefPre, ArrayType, Func, RootSet, RootedGcRefImpl, Val,
        store::AutoAssertNoGc,
        vm::const_expr::{ConstEvalContext, ConstExprEvaluator},
    };
    use wasmtime_environ::{ModuleInternedTypeIndex, TableSegmentElements};

    // Convert indices to their typed forms.
    let array_type_index = ModuleInternedTypeIndex::from_u32(array_type_index);
    let elem_index = ElemIndex::from_u32(elem_index);

    let mut storage = None;
    let elements = instance.passive_element_segment(&mut storage, elem_index);

    let src = usize::try_from(src).map_err(|_| Trap::TableOutOfBounds)?;
    let len = usize::try_from(len).map_err(|_| Trap::TableOutOfBounds)?;

    let shared_ty = instance.engine_type_index(array_type_index);
    let array_ty = ArrayType::from_shared_type_index(store.engine(), shared_ty);
    let elem_ty = array_ty.element_type();
    let pre = ArrayRefPre::_new(store, array_ty);

    RootSet::with_lifo_scope(store, |store| {
        // Turn the elements into `Val`s.
        let mut vals = Vec::with_capacity(usize::try_from(elements.len()).unwrap());
        match elements {
            TableSegmentElements::Functions(fs) => {
                vals.extend(
                    fs.get(src..)
                        .and_then(|s| s.get(..len))
                        .ok_or_else(|| Trap::TableOutOfBounds)?
                        .iter()
                        .map(|f| {
                            let raw_func_ref = instance.as_mut().get_func_ref(*f);
                            let func = unsafe {
                                raw_func_ref.map(|p| Func::from_vm_func_ref(store.id(), p))
                            };
                            Val::FuncRef(func)
                        }),
                );
            }
            TableSegmentElements::Expressions(xs) => {
                let xs = xs
                    .get(src..)
                    .and_then(|s| s.get(..len))
                    .ok_or_else(|| Trap::TableOutOfBounds)?;

                let mut const_context = ConstEvalContext::new(instance.id());
                let mut const_evaluator = ConstExprEvaluator::default();

                vals.extend(xs.iter().map(|x| unsafe {
                    let raw = const_evaluator
                        .eval(store, &mut const_context, x)
                        .expect("const expr should be valid");
                    let mut store = AutoAssertNoGc::new(store);
                    Val::_from_raw(&mut store, raw, elem_ty.unwrap_val_type())
                }));
            }
        }

        let array = unsafe { ArrayRef::new_fixed_maybe_async(store, &pre, &vals)? };

        let mut store = AutoAssertNoGc::new(store);
        let gc_ref = array.try_clone_gc_ref(&mut store)?;
        let raw = store.unwrap_gc_store_mut().expose_gc_ref_to_wasm(gc_ref);
        Ok(raw)
    })
}

#[cfg(feature = "gc")]
unsafe fn array_init_elem(
    store: &mut dyn VMStore,
    mut instance: Pin<&mut Instance>,
    array_type_index: u32,
    array: u32,
    dst: u32,
    elem_index: u32,
    src: u32,
    len: u32,
) -> Result<()> {
    use crate::{
        ArrayRef, Func, OpaqueRootScope, Val,
        store::AutoAssertNoGc,
        vm::const_expr::{ConstEvalContext, ConstExprEvaluator},
    };
    use wasmtime_environ::{ModuleInternedTypeIndex, TableSegmentElements};

    let mut store = OpaqueRootScope::new(store.store_opaque_mut());

    // Convert the indices into their typed forms.
    let _array_type_index = ModuleInternedTypeIndex::from_u32(array_type_index);
    let elem_index = ElemIndex::from_u32(elem_index);

    log::trace!(
        "array.init_elem(array={array:#x}, dst={dst}, elem_index={elem_index:?}, src={src}, len={len})",
    );

    // Convert the raw GC ref into a `Rooted<ArrayRef>`.
    let array = VMGcRef::from_raw_u32(array).ok_or_else(|| Trap::NullReference)?;
    let array = store.unwrap_gc_store_mut().clone_gc_ref(&array);
    let array = {
        let mut no_gc = AutoAssertNoGc::new(&mut store);
        ArrayRef::from_cloned_gc_ref(&mut no_gc, array)
    };

    // Bounds check the destination within the array.
    let array_len = array._len(&store)?;
    log::trace!("array_len = {array_len}");
    if dst.checked_add(len).ok_or_else(|| Trap::ArrayOutOfBounds)? > array_len {
        return Err(Trap::ArrayOutOfBounds.into());
    }

    // Get the passive element segment.
    let mut storage = None;
    let elements = instance.passive_element_segment(&mut storage, elem_index);

    // Convert array offsets into `usize`s.
    let src = usize::try_from(src).map_err(|_| Trap::TableOutOfBounds)?;
    let len = usize::try_from(len).map_err(|_| Trap::TableOutOfBounds)?;

    // Turn the elements into `Val`s.
    let vals = match elements {
        TableSegmentElements::Functions(fs) => fs
            .get(src..)
            .and_then(|s| s.get(..len))
            .ok_or_else(|| Trap::TableOutOfBounds)?
            .iter()
            .map(|f| {
                let raw_func_ref = instance.as_mut().get_func_ref(*f);
                let func = unsafe { raw_func_ref.map(|p| Func::from_vm_func_ref(store.id(), p)) };
                Val::FuncRef(func)
            })
            .collect::<Vec<_>>(),
        TableSegmentElements::Expressions(xs) => {
            let elem_ty = array._ty(&store)?.element_type();
            let elem_ty = elem_ty.unwrap_val_type();

            let mut const_context = ConstEvalContext::new(instance.id());
            let mut const_evaluator = ConstExprEvaluator::default();

            xs.get(src..)
                .and_then(|s| s.get(..len))
                .ok_or_else(|| Trap::TableOutOfBounds)?
                .iter()
                .map(|x| unsafe {
                    let raw = const_evaluator
                        .eval(&mut store, &mut const_context, x)
                        .expect("const expr should be valid");
                    let mut store = AutoAssertNoGc::new(&mut store);
                    Val::_from_raw(&mut store, raw, elem_ty)
                })
                .collect::<Vec<_>>()
        }
    };

    // Copy the values into the array.
    for (i, val) in vals.into_iter().enumerate() {
        let i = u32::try_from(i).unwrap();
        let j = dst.checked_add(i).unwrap();
        array._set(&mut store, j, val)?;
    }

    Ok(())
}

// TODO: Specialize this libcall for only non-GC array elements, so we never
// have to do GC barriers and their associated indirect calls through the `dyn
// GcHeap`. Instead, implement those copies inline in Wasm code. Then, use bulk
// `memcpy`-style APIs to do the actual copies here.
#[cfg(feature = "gc")]
unsafe fn array_copy(
    store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    dst_array: u32,
    dst: u32,
    src_array: u32,
    src: u32,
    len: u32,
) -> Result<()> {
    use crate::{ArrayRef, OpaqueRootScope, store::AutoAssertNoGc};

    log::trace!(
        "array.copy(dst_array={dst_array:#x}, dst_index={dst}, src_array={src_array:#x}, src_index={src}, len={len})",
    );

    let mut store = OpaqueRootScope::new(store.store_opaque_mut());
    let mut store = AutoAssertNoGc::new(&mut store);

    // Convert the raw GC refs into `Rooted<ArrayRef>`s.
    let dst_array = VMGcRef::from_raw_u32(dst_array).ok_or_else(|| Trap::NullReference)?;
    let dst_array = store.unwrap_gc_store_mut().clone_gc_ref(&dst_array);
    let dst_array = ArrayRef::from_cloned_gc_ref(&mut store, dst_array);
    let src_array = VMGcRef::from_raw_u32(src_array).ok_or_else(|| Trap::NullReference)?;
    let src_array = store.unwrap_gc_store_mut().clone_gc_ref(&src_array);
    let src_array = ArrayRef::from_cloned_gc_ref(&mut store, src_array);

    // Bounds check the destination array's elements.
    let dst_array_len = dst_array._len(&store)?;
    if dst.checked_add(len).ok_or_else(|| Trap::ArrayOutOfBounds)? > dst_array_len {
        return Err(Trap::ArrayOutOfBounds.into());
    }

    // Bounds check the source array's elements.
    let src_array_len = src_array._len(&store)?;
    if src.checked_add(len).ok_or_else(|| Trap::ArrayOutOfBounds)? > src_array_len {
        return Err(Trap::ArrayOutOfBounds.into());
    }

    let mut store = AutoAssertNoGc::new(&mut store);
    // If `src_array` and `dst_array` are the same array, then we are
    // potentially doing an overlapping copy, so make sure to copy elements in
    // the order that doesn't clobber the source elements before they are
    // copied. If they are different arrays, the order doesn't matter, but we
    // simply don't bother checking.
    if src > dst {
        for i in 0..len {
            let src_elem = src_array._get(&mut store, src + i)?;
            let dst_i = dst + i;
            dst_array._set(&mut store, dst_i, src_elem)?;
        }
    } else {
        for i in (0..len).rev() {
            let src_elem = src_array._get(&mut store, src + i)?;
            let dst_i = dst + i;
            dst_array._set(&mut store, dst_i, src_elem)?;
        }
    }
    Ok(())
}

#[cfg(feature = "gc")]
unsafe fn is_subtype(
    store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    actual_engine_type: u32,
    expected_engine_type: u32,
) -> u32 {
    use wasmtime_environ::VMSharedTypeIndex;

    let actual = VMSharedTypeIndex::from_u32(actual_engine_type);
    let expected = VMSharedTypeIndex::from_u32(expected_engine_type);

    let is_subtype: bool = store.engine().signatures().is_subtype(actual, expected);

    log::trace!("is_subtype(actual={actual:?}, expected={expected:?}) -> {is_subtype}",);
    is_subtype as u32
}

// Implementation of `memory.atomic.notify` for locally defined memories.
#[cfg(feature = "threads")]
fn memory_atomic_notify(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    memory_index: u32,
    addr_index: u64,
    count: u32,
) -> Result<u32, Trap> {
    let memory = DefinedMemoryIndex::from_u32(memory_index);
    instance
        .get_defined_memory_mut(memory)
        .atomic_notify(addr_index, count)
}

// Implementation of `memory.atomic.wait32` for locally defined memories.
#[cfg(feature = "threads")]
fn memory_atomic_wait32(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    memory_index: u32,
    addr_index: u64,
    expected: u32,
    timeout: u64,
) -> Result<u32, Trap> {
    let timeout = (timeout as i64 >= 0).then(|| Duration::from_nanos(timeout));
    let memory = DefinedMemoryIndex::from_u32(memory_index);
    Ok(instance
        .get_defined_memory_mut(memory)
        .atomic_wait32(addr_index, expected, timeout)? as u32)
}

// Implementation of `memory.atomic.wait64` for locally defined memories.
#[cfg(feature = "threads")]
fn memory_atomic_wait64(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    memory_index: u32,
    addr_index: u64,
    expected: u64,
    timeout: u64,
) -> Result<u32, Trap> {
    let timeout = (timeout as i64 >= 0).then(|| Duration::from_nanos(timeout));
    let memory = DefinedMemoryIndex::from_u32(memory_index);
    Ok(instance
        .get_defined_memory_mut(memory)
        .atomic_wait64(addr_index, expected, timeout)? as u32)
}

// Hook for when an instance runs out of fuel.
fn out_of_gas(store: &mut dyn VMStore, _instance: Pin<&mut Instance>) -> Result<()> {
    store.out_of_gas()
}

// Hook for when an instance observes that the epoch has changed.
#[cfg(target_has_atomic = "64")]
fn new_epoch(store: &mut dyn VMStore, _instance: Pin<&mut Instance>) -> Result<NextEpoch> {
    store.new_epoch().map(NextEpoch)
}

struct NextEpoch(u64);

unsafe impl HostResultHasUnwindSentinel for NextEpoch {
    type Abi = u64;
    const SENTINEL: u64 = u64::MAX;
    fn into_abi(self) -> u64 {
        self.0
    }
}

// Hook for validating malloc using wmemcheck_state.
#[cfg(feature = "wmemcheck")]
unsafe fn check_malloc(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    addr: u32,
    len: u32,
) -> Result<()> {
    if let Some(wmemcheck_state) = instance.wmemcheck_state_mut() {
        let result = wmemcheck_state.malloc(addr as usize, len as usize);
        wmemcheck_state.memcheck_on();
        match result {
            Ok(()) => {}
            Err(DoubleMalloc { addr, len }) => {
                bail!("Double malloc at addr {:#x} of size {}", addr, len)
            }
            Err(OutOfBounds { addr, len }) => {
                bail!("Malloc out of bounds at addr {:#x} of size {}", addr, len);
            }
            _ => {
                panic!("unreachable")
            }
        }
    }
    Ok(())
}

// Hook for validating free using wmemcheck_state.
#[cfg(feature = "wmemcheck")]
unsafe fn check_free(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    addr: u32,
) -> Result<()> {
    if let Some(wmemcheck_state) = instance.wmemcheck_state_mut() {
        let result = wmemcheck_state.free(addr as usize);
        wmemcheck_state.memcheck_on();
        match result {
            Ok(()) => {}
            Err(InvalidFree { addr }) => {
                bail!("Invalid free at addr {:#x}", addr)
            }
            _ => {
                panic!("unreachable")
            }
        }
    }
    Ok(())
}

// Hook for validating load using wmemcheck_state.
#[cfg(feature = "wmemcheck")]
fn check_load(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    num_bytes: u32,
    addr: u32,
    offset: u32,
) -> Result<()> {
    if let Some(wmemcheck_state) = instance.wmemcheck_state_mut() {
        let result = wmemcheck_state.read(addr as usize + offset as usize, num_bytes as usize);
        match result {
            Ok(()) => {}
            Err(InvalidRead { addr, len }) => {
                bail!("Invalid load at addr {:#x} of size {}", addr, len);
            }
            Err(OutOfBounds { addr, len }) => {
                bail!("Load out of bounds at addr {:#x} of size {}", addr, len);
            }
            _ => {
                panic!("unreachable")
            }
        }
    }
    Ok(())
}

// Hook for validating store using wmemcheck_state.
#[cfg(feature = "wmemcheck")]
fn check_store(
    _store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    num_bytes: u32,
    addr: u32,
    offset: u32,
) -> Result<()> {
    if let Some(wmemcheck_state) = instance.wmemcheck_state_mut() {
        let result = wmemcheck_state.write(addr as usize + offset as usize, num_bytes as usize);
        match result {
            Ok(()) => {}
            Err(InvalidWrite { addr, len }) => {
                bail!("Invalid store at addr {:#x} of size {}", addr, len)
            }
            Err(OutOfBounds { addr, len }) => {
                bail!("Store out of bounds at addr {:#x} of size {}", addr, len)
            }
            _ => {
                panic!("unreachable")
            }
        }
    }
    Ok(())
}

// Hook for turning wmemcheck load/store validation off when entering a malloc function.
#[cfg(feature = "wmemcheck")]
fn malloc_start(_store: &mut dyn VMStore, instance: Pin<&mut Instance>) {
    if let Some(wmemcheck_state) = instance.wmemcheck_state_mut() {
        wmemcheck_state.memcheck_off();
    }
}

// Hook for turning wmemcheck load/store validation off when entering a free function.
#[cfg(feature = "wmemcheck")]
fn free_start(_store: &mut dyn VMStore, instance: Pin<&mut Instance>) {
    if let Some(wmemcheck_state) = instance.wmemcheck_state_mut() {
        wmemcheck_state.memcheck_off();
    }
}

// Hook for tracking wasm stack updates using wmemcheck_state.
#[cfg(feature = "wmemcheck")]
fn update_stack_pointer(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>, _value: u32) {
    // TODO: stack-tracing has yet to be finalized. All memory below
    // the address of the top of the stack is marked as valid for
    // loads and stores.
    // if let Some(wmemcheck_state) = &mut instance.wmemcheck_state {
    //     instance.wmemcheck_state.update_stack_pointer(value as usize);
    // }
}

// Hook updating wmemcheck_state memory state vector every time memory.grow is called.
#[cfg(feature = "wmemcheck")]
fn update_mem_size(_store: &mut dyn VMStore, instance: Pin<&mut Instance>, num_pages: u32) {
    if let Some(wmemcheck_state) = instance.wmemcheck_state_mut() {
        const KIB: usize = 1024;
        let num_bytes = num_pages as usize * 64 * KIB;
        wmemcheck_state.update_mem_size(num_bytes);
    }
}

fn floor_f32(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>, val: f32) -> f32 {
    wasmtime_math::WasmFloat::wasm_floor(val)
}

fn floor_f64(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>, val: f64) -> f64 {
    wasmtime_math::WasmFloat::wasm_floor(val)
}

fn ceil_f32(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>, val: f32) -> f32 {
    wasmtime_math::WasmFloat::wasm_ceil(val)
}

fn ceil_f64(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>, val: f64) -> f64 {
    wasmtime_math::WasmFloat::wasm_ceil(val)
}

fn trunc_f32(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>, val: f32) -> f32 {
    wasmtime_math::WasmFloat::wasm_trunc(val)
}

fn trunc_f64(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>, val: f64) -> f64 {
    wasmtime_math::WasmFloat::wasm_trunc(val)
}

fn nearest_f32(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>, val: f32) -> f32 {
    wasmtime_math::WasmFloat::wasm_nearest(val)
}

fn nearest_f64(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>, val: f64) -> f64 {
    wasmtime_math::WasmFloat::wasm_nearest(val)
}

// This intrinsic is only used on x86_64 platforms as an implementation of
// the `i8x16.swizzle` instruction when `pshufb` in SSSE3 is not available.
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
fn i8x16_swizzle(
    _store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    a: i8x16,
    b: i8x16,
) -> i8x16 {
    union U {
        reg: i8x16,
        mem: [u8; 16],
    }

    unsafe {
        let a = U { reg: a }.mem;
        let b = U { reg: b }.mem;

        // Use the `swizzle` semantics of returning 0 on any out-of-bounds
        // index, rather than the x86 pshufb semantics, since Wasmtime uses
        // this to implement `i8x16.swizzle`.
        let select = |arr: &[u8; 16], byte: u8| {
            if byte >= 16 { 0x00 } else { arr[byte as usize] }
        };

        U {
            mem: [
                select(&a, b[0]),
                select(&a, b[1]),
                select(&a, b[2]),
                select(&a, b[3]),
                select(&a, b[4]),
                select(&a, b[5]),
                select(&a, b[6]),
                select(&a, b[7]),
                select(&a, b[8]),
                select(&a, b[9]),
                select(&a, b[10]),
                select(&a, b[11]),
                select(&a, b[12]),
                select(&a, b[13]),
                select(&a, b[14]),
                select(&a, b[15]),
            ],
        }
        .reg
    }
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "sse")))]
fn i8x16_swizzle(
    _store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    _a: i8x16,
    _b: i8x16,
) -> i8x16 {
    unreachable!()
}

// This intrinsic is only used on x86_64 platforms as an implementation of
// the `i8x16.shuffle` instruction when `pshufb` in SSSE3 is not available.
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
fn i8x16_shuffle(
    _store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    a: i8x16,
    b: i8x16,
    c: i8x16,
) -> i8x16 {
    union U {
        reg: i8x16,
        mem: [u8; 16],
    }

    unsafe {
        let ab = [U { reg: a }.mem, U { reg: b }.mem];
        let c = U { reg: c }.mem;

        // Use the `shuffle` semantics of returning 0 on any out-of-bounds
        // index, rather than the x86 pshufb semantics, since Wasmtime uses
        // this to implement `i8x16.shuffle`.
        let select = |arr: &[[u8; 16]; 2], byte: u8| {
            if byte >= 32 {
                0x00
            } else if byte >= 16 {
                arr[1][byte as usize - 16]
            } else {
                arr[0][byte as usize]
            }
        };

        U {
            mem: [
                select(&ab, c[0]),
                select(&ab, c[1]),
                select(&ab, c[2]),
                select(&ab, c[3]),
                select(&ab, c[4]),
                select(&ab, c[5]),
                select(&ab, c[6]),
                select(&ab, c[7]),
                select(&ab, c[8]),
                select(&ab, c[9]),
                select(&ab, c[10]),
                select(&ab, c[11]),
                select(&ab, c[12]),
                select(&ab, c[13]),
                select(&ab, c[14]),
                select(&ab, c[15]),
            ],
        }
        .reg
    }
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "sse")))]
fn i8x16_shuffle(
    _store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    _a: i8x16,
    _b: i8x16,
    _c: i8x16,
) -> i8x16 {
    unreachable!()
}

fn fma_f32x4(
    _store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    x: f32x4,
    y: f32x4,
    z: f32x4,
) -> f32x4 {
    union U {
        reg: f32x4,
        mem: [f32; 4],
    }

    unsafe {
        let x = U { reg: x }.mem;
        let y = U { reg: y }.mem;
        let z = U { reg: z }.mem;

        U {
            mem: [
                wasmtime_math::WasmFloat::wasm_mul_add(x[0], y[0], z[0]),
                wasmtime_math::WasmFloat::wasm_mul_add(x[1], y[1], z[1]),
                wasmtime_math::WasmFloat::wasm_mul_add(x[2], y[2], z[2]),
                wasmtime_math::WasmFloat::wasm_mul_add(x[3], y[3], z[3]),
            ],
        }
        .reg
    }
}

fn fma_f64x2(
    _store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    x: f64x2,
    y: f64x2,
    z: f64x2,
) -> f64x2 {
    union U {
        reg: f64x2,
        mem: [f64; 2],
    }

    unsafe {
        let x = U { reg: x }.mem;
        let y = U { reg: y }.mem;
        let z = U { reg: z }.mem;

        U {
            mem: [
                wasmtime_math::WasmFloat::wasm_mul_add(x[0], y[0], z[0]),
                wasmtime_math::WasmFloat::wasm_mul_add(x[1], y[1], z[1]),
            ],
        }
        .reg
    }
}

/// This intrinsic is just used to record trap information.
///
/// The `Infallible` "ok" type here means that this never returns success, it
/// only ever returns an error, and this hooks into the machinery to handle
/// `Result` values to record such trap information.
fn trap(
    _store: &mut dyn VMStore,
    _instance: Pin<&mut Instance>,
    code: u8,
) -> Result<Infallible, TrapReason> {
    Err(TrapReason::Wasm(
        wasmtime_environ::Trap::from_u8(code).unwrap(),
    ))
}

fn raise(_store: &mut dyn VMStore, _instance: Pin<&mut Instance>) {
    // SAFETY: this is only called from compiled wasm so we know that wasm has
    // already been entered. It's a dynamic safety precondition that the trap
    // information has already been arranged to be present.
    #[cfg(has_host_compiler_backend)]
    unsafe {
        crate::runtime::vm::traphandlers::raise_preexisting_trap()
    }

    // When Cranelift isn't in use then this is an unused libcall for Pulley, so
    // just insert a stub to catch bugs if it's accidentally called.
    #[cfg(not(has_host_compiler_backend))]
    unreachable!()
}

// Builtins for continuations. These are thin wrappers around the
// respective definitions in stack_switching.rs.
#[cfg(feature = "stack-switching")]
fn cont_new(
    store: &mut dyn VMStore,
    instance: Pin<&mut Instance>,
    func: *mut u8,
    param_count: u32,
    result_count: u32,
) -> Result<Option<AllocationSize>, TrapReason> {
    let ans =
        crate::vm::stack_switching::cont_new(store, instance, func, param_count, result_count)?;
    Ok(Some(AllocationSize(ans.cast::<u8>() as usize)))
}
