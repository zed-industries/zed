//! Offsets and sizes of various structs in `wasmtime::runtime::vm::*` that are
//! accessed directly by compiled Wasm code.

// Currently the `VMContext` allocation by field looks like this:
//
// struct VMContext {
//      // Fixed-width data comes first so the calculation of the offset of
//      // these fields is a compile-time constant when using `HostPtr`.
//      magic: u32,
//      _padding: u32, // (On 64-bit systems)
//      vm_store_context: *const VMStoreContext,
//      builtin_functions: *mut VMBuiltinFunctionsArray,
//      epoch_ptr: *mut AtomicU64,
//      gc_heap_data: *mut T, // Collector-specific pointer
//      type_ids: *const VMSharedTypeIndex,
//
//      // Variable-width fields come after the fixed-width fields above. Place
//      // memory-related items first as they're some of the most frequently
//      // accessed items and minimizing their offset in this structure can
//      // shrink the size of load/store instruction offset immediates on
//      // platforms like x64 and Pulley (e.g. fit in an 8-bit offset instead
//      // of needing a 32-bit offset)
//      imported_memories: [VMMemoryImport; module.num_imported_memories],
//      memories: [*mut VMMemoryDefinition; module.num_defined_memories],
//      owned_memories: [VMMemoryDefinition; module.num_owned_memories],
//      imported_functions: [VMFunctionImport; module.num_imported_functions],
//      imported_tables: [VMTableImport; module.num_imported_tables],
//      imported_globals: [VMGlobalImport; module.num_imported_globals],
//      imported_tags: [VMTagImport; module.num_imported_tags],
//      tables: [VMTableDefinition; module.num_defined_tables],
//      globals: [VMGlobalDefinition; module.num_defined_globals],
//      tags: [VMTagDefinition; module.num_defined_tags],
//      func_refs: [VMFuncRef; module.num_escaped_funcs],
// }

use crate::{
    DefinedGlobalIndex, DefinedMemoryIndex, DefinedTableIndex, DefinedTagIndex, FuncIndex,
    FuncRefIndex, GlobalIndex, MemoryIndex, Module, OwnedMemoryIndex, TableIndex, TagIndex,
};
use cranelift_entity::packed_option::ReservedValue;

#[cfg(target_pointer_width = "32")]
fn cast_to_u32(sz: usize) -> u32 {
    u32::try_from(sz).unwrap()
}
#[cfg(target_pointer_width = "64")]
fn cast_to_u32(sz: usize) -> u32 {
    u32::try_from(sz).expect("overflow in cast from usize to u32")
}

/// Align an offset used in this module to a specific byte-width by rounding up
#[inline]
fn align(offset: u32, width: u32) -> u32 {
    (offset + (width - 1)) / width * width
}

/// This class computes offsets to fields within `VMContext` and other
/// related structs that JIT code accesses directly.
#[derive(Debug, Clone, Copy)]
pub struct VMOffsets<P> {
    /// The size in bytes of a pointer on the target.
    pub ptr: P,
    /// The number of imported functions in the module.
    pub num_imported_functions: u32,
    /// The number of imported tables in the module.
    pub num_imported_tables: u32,
    /// The number of imported memories in the module.
    pub num_imported_memories: u32,
    /// The number of imported globals in the module.
    pub num_imported_globals: u32,
    /// The number of imported tags in the module.
    pub num_imported_tags: u32,
    /// The number of defined tables in the module.
    pub num_defined_tables: u32,
    /// The number of defined memories in the module.
    pub num_defined_memories: u32,
    /// The number of memories owned by the module instance.
    pub num_owned_memories: u32,
    /// The number of defined globals in the module.
    pub num_defined_globals: u32,
    /// The number of defined tags in the module.
    pub num_defined_tags: u32,
    /// The number of escaped functions in the module, the size of the func_refs
    /// array.
    pub num_escaped_funcs: u32,

    // precalculated offsets of various member fields
    imported_functions: u32,
    imported_tables: u32,
    imported_memories: u32,
    imported_globals: u32,
    imported_tags: u32,
    defined_tables: u32,
    defined_memories: u32,
    owned_memories: u32,
    defined_globals: u32,
    defined_tags: u32,
    defined_func_refs: u32,
    size: u32,
}

/// Trait used for the `ptr` representation of the field of `VMOffsets`
pub trait PtrSize {
    /// Returns the pointer size, in bytes, for the target.
    fn size(&self) -> u8;

    /// The offset of the `VMContext::store_context` field
    fn vmcontext_store_context(&self) -> u8 {
        u8::try_from(align(
            u32::try_from(core::mem::size_of::<u32>()).unwrap(),
            u32::from(self.size()),
        ))
        .unwrap()
    }

    /// The offset of the `VMContext::builtin_functions` field
    fn vmcontext_builtin_functions(&self) -> u8 {
        self.vmcontext_store_context() + self.size()
    }

    /// The offset of the `array_call` field.
    #[inline]
    fn vm_func_ref_array_call(&self) -> u8 {
        0 * self.size()
    }

    /// The offset of the `wasm_call` field.
    #[inline]
    fn vm_func_ref_wasm_call(&self) -> u8 {
        1 * self.size()
    }

    /// The offset of the `type_index` field.
    #[inline]
    fn vm_func_ref_type_index(&self) -> u8 {
        2 * self.size()
    }

    /// The offset of the `vmctx` field.
    #[inline]
    fn vm_func_ref_vmctx(&self) -> u8 {
        3 * self.size()
    }

    /// Return the size of `VMFuncRef`.
    #[inline]
    fn size_of_vm_func_ref(&self) -> u8 {
        4 * self.size()
    }

    /// Return the size of `VMGlobalDefinition`; this is the size of the largest value type (i.e. a
    /// V128).
    #[inline]
    fn size_of_vmglobal_definition(&self) -> u8 {
        16
    }

    /// Return the size of `VMTagDefinition`.
    #[inline]
    fn size_of_vmtag_definition(&self) -> u8 {
        4
    }

    // Offsets within `VMStoreContext`

    /// Return the offset of the `fuel_consumed` field of `VMStoreContext`
    #[inline]
    fn vmstore_context_fuel_consumed(&self) -> u8 {
        0
    }

    /// Return the offset of the `epoch_deadline` field of `VMStoreContext`
    #[inline]
    fn vmstore_context_epoch_deadline(&self) -> u8 {
        self.vmstore_context_fuel_consumed() + 8
    }

    /// Return the offset of the `stack_limit` field of `VMStoreContext`
    #[inline]
    fn vmstore_context_stack_limit(&self) -> u8 {
        self.vmstore_context_epoch_deadline() + 8
    }

    /// Return the offset of the `gc_heap` field of `VMStoreContext`.
    #[inline]
    fn vmstore_context_gc_heap(&self) -> u8 {
        self.vmstore_context_stack_limit() + self.size()
    }

    /// Return the offset of the `gc_heap.base` field within a `VMStoreContext`.
    fn vmstore_context_gc_heap_base(&self) -> u8 {
        let offset = self.vmstore_context_gc_heap() + self.vmmemory_definition_base();
        debug_assert!(offset < self.vmstore_context_last_wasm_exit_fp());
        offset
    }

    /// Return the offset of the `gc_heap.current_length` field within a `VMStoreContext`.
    fn vmstore_context_gc_heap_current_length(&self) -> u8 {
        let offset = self.vmstore_context_gc_heap() + self.vmmemory_definition_current_length();
        debug_assert!(offset < self.vmstore_context_last_wasm_exit_fp());
        offset
    }

    /// Return the offset of the `last_wasm_exit_fp` field of `VMStoreContext`.
    fn vmstore_context_last_wasm_exit_fp(&self) -> u8 {
        self.vmstore_context_gc_heap() + self.size_of_vmmemory_definition()
    }

    /// Return the offset of the `last_wasm_exit_pc` field of `VMStoreContext`.
    fn vmstore_context_last_wasm_exit_pc(&self) -> u8 {
        self.vmstore_context_last_wasm_exit_fp() + self.size()
    }

    /// Return the offset of the `last_wasm_entry_fp` field of `VMStoreContext`.
    fn vmstore_context_last_wasm_entry_fp(&self) -> u8 {
        self.vmstore_context_last_wasm_exit_pc() + self.size()
    }

    /// Return the offset of the `stack_chain` field of `VMStoreContext`.
    fn vmstore_context_stack_chain(&self) -> u8 {
        self.vmstore_context_last_wasm_entry_fp() + self.size()
    }

    // Offsets within `VMMemoryDefinition`

    /// The offset of the `base` field.
    #[inline]
    fn vmmemory_definition_base(&self) -> u8 {
        0 * self.size()
    }

    /// The offset of the `current_length` field.
    #[inline]
    fn vmmemory_definition_current_length(&self) -> u8 {
        1 * self.size()
    }

    /// Return the size of `VMMemoryDefinition`.
    #[inline]
    fn size_of_vmmemory_definition(&self) -> u8 {
        2 * self.size()
    }

    /// Return the size of `*mut VMMemoryDefinition`.
    #[inline]
    fn size_of_vmmemory_pointer(&self) -> u8 {
        self.size()
    }

    // Offsets within `VMArrayCallHostFuncContext`.

    /// Return the offset of `VMArrayCallHostFuncContext::func_ref`.
    fn vmarray_call_host_func_context_func_ref(&self) -> u8 {
        u8::try_from(align(
            u32::try_from(core::mem::size_of::<u32>()).unwrap(),
            u32::from(self.size()),
        ))
        .unwrap()
    }

    /// Return the size of `VMStackChain`.
    fn size_of_vmstack_chain(&self) -> u8 {
        2 * self.size()
    }

    // Offsets within `VMStackLimits`

    /// Return the offset of `VMStackLimits::stack_limit`.
    fn vmstack_limits_stack_limit(&self) -> u8 {
        0
    }

    /// Return the offset of `VMStackLimits::last_wasm_entry_fp`.
    fn vmstack_limits_last_wasm_entry_fp(&self) -> u8 {
        self.size()
    }

    // Offsets within `VMHostArray`

    /// Return the offset of `VMHostArray::length`.
    fn vmhostarray_length(&self) -> u8 {
        0
    }

    /// Return the offset of `VMHostArray::capacity`.
    fn vmhostarray_capacity(&self) -> u8 {
        4
    }

    /// Return the offset of `VMHostArray::data`.
    fn vmhostarray_data(&self) -> u8 {
        8
    }

    /// Return the size of `VMHostArray`.
    fn size_of_vmhostarray(&self) -> u8 {
        8 + self.size()
    }

    // Offsets within `VMCommonStackInformation`

    /// Return the offset of `VMCommonStackInformation::limits`.
    fn vmcommon_stack_information_limits(&self) -> u8 {
        0 * self.size()
    }

    /// Return the offset of `VMCommonStackInformation::state`.
    fn vmcommon_stack_information_state(&self) -> u8 {
        2 * self.size()
    }

    /// Return the offset of `VMCommonStackInformation::handlers`.
    fn vmcommon_stack_information_handlers(&self) -> u8 {
        u8::try_from(align(
            self.vmcommon_stack_information_state() as u32 + 4,
            u32::from(self.size()),
        ))
        .unwrap()
    }

    /// Return the offset of `VMCommonStackInformation::first_switch_handler_index`.
    fn vmcommon_stack_information_first_switch_handler_index(&self) -> u8 {
        self.vmcommon_stack_information_handlers() + self.size_of_vmhostarray()
    }

    /// Return the size of `VMCommonStackInformation`.
    fn size_of_vmcommon_stack_information(&self) -> u8 {
        u8::try_from(align(
            self.vmcommon_stack_information_first_switch_handler_index() as u32 + 4,
            u32::from(self.size()),
        ))
        .unwrap()
    }

    // Offsets within `VMContRef`

    /// Return the offset of `VMContRef::common_stack_information`.
    fn vmcontref_common_stack_information(&self) -> u8 {
        0 * self.size()
    }

    /// Return the offset of `VMContRef::parent_chain`.
    fn vmcontref_parent_chain(&self) -> u8 {
        u8::try_from(align(
            (self.vmcontref_common_stack_information() + self.size_of_vmcommon_stack_information())
                as u32,
            u32::from(self.size()),
        ))
        .unwrap()
    }

    /// Return the offset of `VMContRef::last_ancestor`.
    fn vmcontref_last_ancestor(&self) -> u8 {
        self.vmcontref_parent_chain() + 2 * self.size()
    }

    /// Return the offset of `VMContRef::revision`.
    fn vmcontref_revision(&self) -> u8 {
        self.vmcontref_last_ancestor() + self.size()
    }

    /// Return the offset of `VMContRef::stack`.
    fn vmcontref_stack(&self) -> u8 {
        self.vmcontref_revision() + 8
    }

    /// Return the offset of `VMContRef::args`.
    fn vmcontref_args(&self) -> u8 {
        self.vmcontref_stack() + 3 * self.size()
    }

    /// Return the offset of `VMContRef::values`.
    fn vmcontref_values(&self) -> u8 {
        self.vmcontref_args() + self.size_of_vmhostarray()
    }

    /// Return the offset to the `magic` value in this `VMContext`.
    #[inline]
    fn vmctx_magic(&self) -> u8 {
        // This is required by the implementation of `VMContext::instance` and
        // `VMContext::instance_mut`. If this value changes then those locations
        // need to be updated.
        0
    }

    /// Return the offset to the `VMStoreContext` structure
    #[inline]
    fn vmctx_store_context(&self) -> u8 {
        self.vmctx_magic() + self.size()
    }

    /// Return the offset to the `VMBuiltinFunctionsArray` structure
    #[inline]
    fn vmctx_builtin_functions(&self) -> u8 {
        self.vmctx_store_context() + self.size()
    }

    /// Return the offset to the `*const AtomicU64` epoch-counter
    /// pointer.
    #[inline]
    fn vmctx_epoch_ptr(&self) -> u8 {
        self.vmctx_builtin_functions() + self.size()
    }

    /// Return the offset to the `*mut T` collector-specific data.
    ///
    /// This is a pointer that different collectors can use however they see
    /// fit.
    #[inline]
    fn vmctx_gc_heap_data(&self) -> u8 {
        self.vmctx_epoch_ptr() + self.size()
    }

    /// The offset of the `type_ids` array pointer.
    #[inline]
    fn vmctx_type_ids_array(&self) -> u8 {
        self.vmctx_gc_heap_data() + self.size()
    }

    /// The end of statically known offsets in `VMContext`.
    ///
    /// Data after this is dynamically sized.
    #[inline]
    fn vmctx_dynamic_data_start(&self) -> u8 {
        self.vmctx_type_ids_array() + self.size()
    }
}

/// Type representing the size of a pointer for the current compilation host
#[derive(Clone, Copy)]
pub struct HostPtr;

impl PtrSize for HostPtr {
    #[inline]
    fn size(&self) -> u8 {
        core::mem::size_of::<usize>() as u8
    }
}

impl PtrSize for u8 {
    #[inline]
    fn size(&self) -> u8 {
        *self
    }
}

/// Used to construct a `VMOffsets`
#[derive(Debug, Clone, Copy)]
pub struct VMOffsetsFields<P> {
    /// The size in bytes of a pointer on the target.
    pub ptr: P,
    /// The number of imported functions in the module.
    pub num_imported_functions: u32,
    /// The number of imported tables in the module.
    pub num_imported_tables: u32,
    /// The number of imported memories in the module.
    pub num_imported_memories: u32,
    /// The number of imported globals in the module.
    pub num_imported_globals: u32,
    /// The number of imported tags in the module.
    pub num_imported_tags: u32,
    /// The number of defined tables in the module.
    pub num_defined_tables: u32,
    /// The number of defined memories in the module.
    pub num_defined_memories: u32,
    /// The number of memories owned by the module instance.
    pub num_owned_memories: u32,
    /// The number of defined globals in the module.
    pub num_defined_globals: u32,
    /// The number of defined tags in the module.
    pub num_defined_tags: u32,
    /// The number of escaped functions in the module, the size of the function
    /// references array.
    pub num_escaped_funcs: u32,
}

impl<P: PtrSize> VMOffsets<P> {
    /// Return a new `VMOffsets` instance, for a given pointer size.
    pub fn new(ptr: P, module: &Module) -> Self {
        let num_owned_memories = module
            .memories
            .iter()
            .skip(module.num_imported_memories)
            .filter(|p| !p.1.shared)
            .count()
            .try_into()
            .unwrap();
        VMOffsets::from(VMOffsetsFields {
            ptr,
            num_imported_functions: cast_to_u32(module.num_imported_funcs),
            num_imported_tables: cast_to_u32(module.num_imported_tables),
            num_imported_memories: cast_to_u32(module.num_imported_memories),
            num_imported_globals: cast_to_u32(module.num_imported_globals),
            num_imported_tags: cast_to_u32(module.num_imported_tags),
            num_defined_tables: cast_to_u32(module.num_defined_tables()),
            num_defined_memories: cast_to_u32(module.num_defined_memories()),
            num_owned_memories,
            num_defined_globals: cast_to_u32(module.globals.len() - module.num_imported_globals),
            num_defined_tags: cast_to_u32(module.tags.len() - module.num_imported_tags),
            num_escaped_funcs: cast_to_u32(module.num_escaped_funcs),
        })
    }

    /// Returns the size, in bytes, of the target
    #[inline]
    pub fn pointer_size(&self) -> u8 {
        self.ptr.size()
    }

    /// Returns an iterator which provides a human readable description and a
    /// byte size. The iterator returned will iterate over the bytes allocated
    /// to the entire `VMOffsets` structure to explain where each byte size is
    /// coming from.
    pub fn region_sizes(&self) -> impl Iterator<Item = (&str, u32)> {
        macro_rules! calculate_sizes {
            ($($name:ident: $desc:tt,)*) => {{
                let VMOffsets {
                    // These fields are metadata not talking about specific
                    // offsets of specific fields.
                    ptr: _,
                    num_imported_functions: _,
                    num_imported_tables: _,
                    num_imported_memories: _,
                    num_imported_globals: _,
                    num_imported_tags: _,
                    num_defined_tables: _,
                    num_defined_globals: _,
                    num_defined_memories: _,
                    num_defined_tags: _,
                    num_owned_memories: _,
                    num_escaped_funcs: _,

                    // used as the initial size below
                    size,

                    // exhaustively match the rest of the fields with input from
                    // the macro
                    $($name,)*
                } = *self;

                // calculate the size of each field by relying on the inputs to
                // the macro being in reverse order and determining the size of
                // the field as the offset from the field to the last field.
                let mut last = size;
                $(
                    assert!($name <= last);
                    let tmp = $name;
                    let $name = last - $name;
                    last = tmp;
                )*
                assert_ne!(last, 0);
                IntoIterator::into_iter([
                    $(($desc, $name),)*
                    ("static vmctx data", last),
                ])
            }};
        }

        calculate_sizes! {
            defined_func_refs: "module functions",
            defined_tags: "defined tags",
            defined_globals: "defined globals",
            defined_tables: "defined tables",
            imported_tags: "imported tags",
            imported_globals: "imported globals",
            imported_tables: "imported tables",
            imported_functions: "imported functions",
            owned_memories: "owned memories",
            defined_memories: "defined memories",
            imported_memories: "imported memories",
        }
    }
}

impl<P: PtrSize> From<VMOffsetsFields<P>> for VMOffsets<P> {
    fn from(fields: VMOffsetsFields<P>) -> VMOffsets<P> {
        let mut ret = Self {
            ptr: fields.ptr,
            num_imported_functions: fields.num_imported_functions,
            num_imported_tables: fields.num_imported_tables,
            num_imported_memories: fields.num_imported_memories,
            num_imported_globals: fields.num_imported_globals,
            num_imported_tags: fields.num_imported_tags,
            num_defined_tables: fields.num_defined_tables,
            num_defined_memories: fields.num_defined_memories,
            num_owned_memories: fields.num_owned_memories,
            num_defined_globals: fields.num_defined_globals,
            num_defined_tags: fields.num_defined_tags,
            num_escaped_funcs: fields.num_escaped_funcs,
            imported_functions: 0,
            imported_tables: 0,
            imported_memories: 0,
            imported_globals: 0,
            imported_tags: 0,
            defined_tables: 0,
            defined_memories: 0,
            owned_memories: 0,
            defined_globals: 0,
            defined_tags: 0,
            defined_func_refs: 0,
            size: 0,
        };

        // Convenience functions for checked addition and multiplication.
        // As side effect this reduces binary size by using only a single
        // `#[track_caller]` location for each function instead of one for
        // each individual invocation.
        #[inline]
        fn cadd(count: u32, size: u32) -> u32 {
            count.checked_add(size).unwrap()
        }

        #[inline]
        fn cmul(count: u32, size: u8) -> u32 {
            count.checked_mul(u32::from(size)).unwrap()
        }

        let mut next_field_offset = u32::from(ret.ptr.vmctx_dynamic_data_start());

        macro_rules! fields {
            (size($field:ident) = $size:expr, $($rest:tt)*) => {
                ret.$field = next_field_offset;
                next_field_offset = cadd(next_field_offset, u32::from($size));
                fields!($($rest)*);
            };
            (align($align:expr), $($rest:tt)*) => {
                next_field_offset = align(next_field_offset, $align);
                fields!($($rest)*);
            };
            () => {};
        }

        fields! {
            size(imported_memories)
                = cmul(ret.num_imported_memories, ret.size_of_vmmemory_import()),
            size(defined_memories)
                = cmul(ret.num_defined_memories, ret.ptr.size_of_vmmemory_pointer()),
            size(owned_memories)
                = cmul(ret.num_owned_memories, ret.ptr.size_of_vmmemory_definition()),
            size(imported_functions)
                = cmul(ret.num_imported_functions, ret.size_of_vmfunction_import()),
            size(imported_tables)
                = cmul(ret.num_imported_tables, ret.size_of_vmtable_import()),
            size(imported_globals)
                = cmul(ret.num_imported_globals, ret.size_of_vmglobal_import()),
            size(imported_tags)
                = cmul(ret.num_imported_tags, ret.size_of_vmtag_import()),
            size(defined_tables)
                = cmul(ret.num_defined_tables, ret.size_of_vmtable_definition()),
            align(16),
            size(defined_globals)
                = cmul(ret.num_defined_globals, ret.ptr.size_of_vmglobal_definition()),
            size(defined_tags)
                = cmul(ret.num_defined_tags, ret.ptr.size_of_vmtag_definition()),
            size(defined_func_refs) = cmul(
                ret.num_escaped_funcs,
                ret.ptr.size_of_vm_func_ref(),
            ),
        }

        ret.size = next_field_offset;

        return ret;
    }
}

impl<P: PtrSize> VMOffsets<P> {
    /// The offset of the `wasm_call` field.
    #[inline]
    pub fn vmfunction_import_wasm_call(&self) -> u8 {
        0 * self.pointer_size()
    }

    /// The offset of the `array_call` field.
    #[inline]
    pub fn vmfunction_import_array_call(&self) -> u8 {
        1 * self.pointer_size()
    }

    /// The offset of the `vmctx` field.
    #[inline]
    pub fn vmfunction_import_vmctx(&self) -> u8 {
        2 * self.pointer_size()
    }

    /// Return the size of `VMFunctionImport`.
    #[inline]
    pub fn size_of_vmfunction_import(&self) -> u8 {
        3 * self.pointer_size()
    }
}

/// Offsets for `*const VMFunctionBody`.
impl<P: PtrSize> VMOffsets<P> {
    /// The size of the `current_elements` field.
    pub fn size_of_vmfunction_body_ptr(&self) -> u8 {
        1 * self.pointer_size()
    }
}

/// Offsets for `VMTableImport`.
impl<P: PtrSize> VMOffsets<P> {
    /// The offset of the `from` field.
    #[inline]
    pub fn vmtable_import_from(&self) -> u8 {
        0 * self.pointer_size()
    }

    /// The offset of the `vmctx` field.
    #[inline]
    pub fn vmtable_import_vmctx(&self) -> u8 {
        1 * self.pointer_size()
    }

    /// The offset of the `index` field.
    #[inline]
    pub fn vmtable_import_index(&self) -> u8 {
        2 * self.pointer_size()
    }

    /// Return the size of `VMTableImport`.
    #[inline]
    pub fn size_of_vmtable_import(&self) -> u8 {
        3 * self.pointer_size()
    }
}

/// Offsets for `VMTableDefinition`.
impl<P: PtrSize> VMOffsets<P> {
    /// The offset of the `base` field.
    #[inline]
    pub fn vmtable_definition_base(&self) -> u8 {
        0 * self.pointer_size()
    }

    /// The offset of the `current_elements` field.
    pub fn vmtable_definition_current_elements(&self) -> u8 {
        1 * self.pointer_size()
    }

    /// The size of the `current_elements` field.
    #[inline]
    pub fn size_of_vmtable_definition_current_elements(&self) -> u8 {
        self.pointer_size()
    }

    /// Return the size of `VMTableDefinition`.
    #[inline]
    pub fn size_of_vmtable_definition(&self) -> u8 {
        2 * self.pointer_size()
    }
}

/// Offsets for `VMMemoryImport`.
impl<P: PtrSize> VMOffsets<P> {
    /// The offset of the `from` field.
    #[inline]
    pub fn vmmemory_import_from(&self) -> u8 {
        0 * self.pointer_size()
    }

    /// The offset of the `vmctx` field.
    #[inline]
    pub fn vmmemory_import_vmctx(&self) -> u8 {
        1 * self.pointer_size()
    }

    /// The offset of the `index` field.
    #[inline]
    pub fn vmmemory_import_index(&self) -> u8 {
        2 * self.pointer_size()
    }

    /// Return the size of `VMMemoryImport`.
    #[inline]
    pub fn size_of_vmmemory_import(&self) -> u8 {
        3 * self.pointer_size()
    }
}

/// Offsets for `VMGlobalImport`.
impl<P: PtrSize> VMOffsets<P> {
    /// The offset of the `from` field.
    #[inline]
    pub fn vmglobal_import_from(&self) -> u8 {
        0 * self.pointer_size()
    }

    /// Return the size of `VMGlobalImport`.
    #[inline]
    pub fn size_of_vmglobal_import(&self) -> u8 {
        // `VMGlobalImport` has two pointers plus 8 bytes for `VMGlobalKind`
        2 * self.pointer_size() + 8
    }
}

/// Offsets for `VMSharedTypeIndex`.
impl<P: PtrSize> VMOffsets<P> {
    /// Return the size of `VMSharedTypeIndex`.
    #[inline]
    pub fn size_of_vmshared_type_index(&self) -> u8 {
        4
    }
}

/// Offsets for `VMTagImport`.
impl<P: PtrSize> VMOffsets<P> {
    /// The offset of the `from` field.
    #[inline]
    pub fn vmtag_import_from(&self) -> u8 {
        0 * self.pointer_size()
    }

    /// Return the size of `VMTagImport`.
    #[inline]
    pub fn size_of_vmtag_import(&self) -> u8 {
        3 * self.pointer_size()
    }
}

/// Offsets for `VMContext`.
impl<P: PtrSize> VMOffsets<P> {
    /// The offset of the `tables` array.
    #[inline]
    pub fn vmctx_imported_functions_begin(&self) -> u32 {
        self.imported_functions
    }

    /// The offset of the `tables` array.
    #[inline]
    pub fn vmctx_imported_tables_begin(&self) -> u32 {
        self.imported_tables
    }

    /// The offset of the `memories` array.
    #[inline]
    pub fn vmctx_imported_memories_begin(&self) -> u32 {
        self.imported_memories
    }

    /// The offset of the `globals` array.
    #[inline]
    pub fn vmctx_imported_globals_begin(&self) -> u32 {
        self.imported_globals
    }

    /// The offset of the `tags` array.
    #[inline]
    pub fn vmctx_imported_tags_begin(&self) -> u32 {
        self.imported_tags
    }

    /// The offset of the `tables` array.
    #[inline]
    pub fn vmctx_tables_begin(&self) -> u32 {
        self.defined_tables
    }

    /// The offset of the `memories` array.
    #[inline]
    pub fn vmctx_memories_begin(&self) -> u32 {
        self.defined_memories
    }

    /// The offset of the `owned_memories` array.
    #[inline]
    pub fn vmctx_owned_memories_begin(&self) -> u32 {
        self.owned_memories
    }

    /// The offset of the `globals` array.
    #[inline]
    pub fn vmctx_globals_begin(&self) -> u32 {
        self.defined_globals
    }

    /// The offset of the `tags` array.
    #[inline]
    pub fn vmctx_tags_begin(&self) -> u32 {
        self.defined_tags
    }

    /// The offset of the `func_refs` array.
    #[inline]
    pub fn vmctx_func_refs_begin(&self) -> u32 {
        self.defined_func_refs
    }

    /// Return the size of the `VMContext` allocation.
    #[inline]
    pub fn size_of_vmctx(&self) -> u32 {
        self.size
    }

    /// Return the offset to `VMFunctionImport` index `index`.
    #[inline]
    pub fn vmctx_vmfunction_import(&self, index: FuncIndex) -> u32 {
        assert!(index.as_u32() < self.num_imported_functions);
        self.vmctx_imported_functions_begin()
            + index.as_u32() * u32::from(self.size_of_vmfunction_import())
    }

    /// Return the offset to `VMTable` index `index`.
    #[inline]
    pub fn vmctx_vmtable_import(&self, index: TableIndex) -> u32 {
        assert!(index.as_u32() < self.num_imported_tables);
        self.vmctx_imported_tables_begin()
            + index.as_u32() * u32::from(self.size_of_vmtable_import())
    }

    /// Return the offset to `VMMemoryImport` index `index`.
    #[inline]
    pub fn vmctx_vmmemory_import(&self, index: MemoryIndex) -> u32 {
        assert!(index.as_u32() < self.num_imported_memories);
        self.vmctx_imported_memories_begin()
            + index.as_u32() * u32::from(self.size_of_vmmemory_import())
    }

    /// Return the offset to `VMGlobalImport` index `index`.
    #[inline]
    pub fn vmctx_vmglobal_import(&self, index: GlobalIndex) -> u32 {
        assert!(index.as_u32() < self.num_imported_globals);
        self.vmctx_imported_globals_begin()
            + index.as_u32() * u32::from(self.size_of_vmglobal_import())
    }

    /// Return the offset to `VMTagImport` index `index`.
    #[inline]
    pub fn vmctx_vmtag_import(&self, index: TagIndex) -> u32 {
        assert!(index.as_u32() < self.num_imported_tags);
        self.vmctx_imported_tags_begin() + index.as_u32() * u32::from(self.size_of_vmtag_import())
    }

    /// Return the offset to `VMTableDefinition` index `index`.
    #[inline]
    pub fn vmctx_vmtable_definition(&self, index: DefinedTableIndex) -> u32 {
        assert!(index.as_u32() < self.num_defined_tables);
        self.vmctx_tables_begin() + index.as_u32() * u32::from(self.size_of_vmtable_definition())
    }

    /// Return the offset to the `*mut VMMemoryDefinition` at index `index`.
    #[inline]
    pub fn vmctx_vmmemory_pointer(&self, index: DefinedMemoryIndex) -> u32 {
        assert!(index.as_u32() < self.num_defined_memories);
        self.vmctx_memories_begin()
            + index.as_u32() * u32::from(self.ptr.size_of_vmmemory_pointer())
    }

    /// Return the offset to the owned `VMMemoryDefinition` at index `index`.
    #[inline]
    pub fn vmctx_vmmemory_definition(&self, index: OwnedMemoryIndex) -> u32 {
        assert!(index.as_u32() < self.num_owned_memories);
        self.vmctx_owned_memories_begin()
            + index.as_u32() * u32::from(self.ptr.size_of_vmmemory_definition())
    }

    /// Return the offset to the `VMGlobalDefinition` index `index`.
    #[inline]
    pub fn vmctx_vmglobal_definition(&self, index: DefinedGlobalIndex) -> u32 {
        assert!(index.as_u32() < self.num_defined_globals);
        self.vmctx_globals_begin()
            + index.as_u32() * u32::from(self.ptr.size_of_vmglobal_definition())
    }

    /// Return the offset to the `VMTagDefinition` index `index`.
    #[inline]
    pub fn vmctx_vmtag_definition(&self, index: DefinedTagIndex) -> u32 {
        assert!(index.as_u32() < self.num_defined_tags);
        self.vmctx_tags_begin() + index.as_u32() * u32::from(self.ptr.size_of_vmtag_definition())
    }

    /// Return the offset to the `VMFuncRef` for the given function
    /// index (either imported or defined).
    #[inline]
    pub fn vmctx_func_ref(&self, index: FuncRefIndex) -> u32 {
        assert!(!index.is_reserved_value());
        assert!(index.as_u32() < self.num_escaped_funcs);
        self.vmctx_func_refs_begin() + index.as_u32() * u32::from(self.ptr.size_of_vm_func_ref())
    }

    /// Return the offset to the `wasm_call` field in `*const VMFunctionBody` index `index`.
    #[inline]
    pub fn vmctx_vmfunction_import_wasm_call(&self, index: FuncIndex) -> u32 {
        self.vmctx_vmfunction_import(index) + u32::from(self.vmfunction_import_wasm_call())
    }

    /// Return the offset to the `array_call` field in `*const VMFunctionBody` index `index`.
    #[inline]
    pub fn vmctx_vmfunction_import_array_call(&self, index: FuncIndex) -> u32 {
        self.vmctx_vmfunction_import(index) + u32::from(self.vmfunction_import_array_call())
    }

    /// Return the offset to the `vmctx` field in `*const VMFunctionBody` index `index`.
    #[inline]
    pub fn vmctx_vmfunction_import_vmctx(&self, index: FuncIndex) -> u32 {
        self.vmctx_vmfunction_import(index) + u32::from(self.vmfunction_import_vmctx())
    }

    /// Return the offset to the `from` field in the imported `VMTable` at index
    /// `index`.
    #[inline]
    pub fn vmctx_vmtable_from(&self, index: TableIndex) -> u32 {
        self.vmctx_vmtable_import(index) + u32::from(self.vmtable_import_from())
    }

    /// Return the offset to the `base` field in `VMTableDefinition` index `index`.
    #[inline]
    pub fn vmctx_vmtable_definition_base(&self, index: DefinedTableIndex) -> u32 {
        self.vmctx_vmtable_definition(index) + u32::from(self.vmtable_definition_base())
    }

    /// Return the offset to the `current_elements` field in `VMTableDefinition` index `index`.
    #[inline]
    pub fn vmctx_vmtable_definition_current_elements(&self, index: DefinedTableIndex) -> u32 {
        self.vmctx_vmtable_definition(index) + u32::from(self.vmtable_definition_current_elements())
    }

    /// Return the offset to the `from` field in `VMMemoryImport` index `index`.
    #[inline]
    pub fn vmctx_vmmemory_import_from(&self, index: MemoryIndex) -> u32 {
        self.vmctx_vmmemory_import(index) + u32::from(self.vmmemory_import_from())
    }

    /// Return the offset to the `base` field in `VMMemoryDefinition` index `index`.
    #[inline]
    pub fn vmctx_vmmemory_definition_base(&self, index: OwnedMemoryIndex) -> u32 {
        self.vmctx_vmmemory_definition(index) + u32::from(self.ptr.vmmemory_definition_base())
    }

    /// Return the offset to the `current_length` field in `VMMemoryDefinition` index `index`.
    #[inline]
    pub fn vmctx_vmmemory_definition_current_length(&self, index: OwnedMemoryIndex) -> u32 {
        self.vmctx_vmmemory_definition(index)
            + u32::from(self.ptr.vmmemory_definition_current_length())
    }

    /// Return the offset to the `from` field in `VMGlobalImport` index `index`.
    #[inline]
    pub fn vmctx_vmglobal_import_from(&self, index: GlobalIndex) -> u32 {
        self.vmctx_vmglobal_import(index) + u32::from(self.vmglobal_import_from())
    }

    /// Return the offset to the `from` field in `VMTagImport` index `index`.
    #[inline]
    pub fn vmctx_vmtag_import_from(&self, index: TagIndex) -> u32 {
        self.vmctx_vmtag_import(index) + u32::from(self.vmtag_import_from())
    }
}

/// Offsets for `VMGcHeader`.
impl<P: PtrSize> VMOffsets<P> {
    /// Return the offset for the `VMGcHeader::kind` field.
    #[inline]
    pub fn vm_gc_header_kind(&self) -> u32 {
        0
    }

    /// Return the offset for the `VMGcHeader`'s reserved bits.
    #[inline]
    pub fn vm_gc_header_reserved_bits(&self) -> u32 {
        // NB: The reserved bits are the unused `VMGcKind` bits.
        self.vm_gc_header_kind()
    }

    /// Return the offset for the `VMGcHeader::ty` field.
    #[inline]
    pub fn vm_gc_header_ty(&self) -> u32 {
        self.vm_gc_header_kind() + 4
    }
}

/// Offsets for `VMDrcHeader`.
///
/// Should only be used when the DRC collector is enabled.
impl<P: PtrSize> VMOffsets<P> {
    /// Return the offset for `VMDrcHeader::ref_count`.
    #[inline]
    pub fn vm_drc_header_ref_count(&self) -> u32 {
        8
    }

    /// Return the offset for `VMDrcHeader::next_over_approximated_stack_root`.
    #[inline]
    pub fn vm_drc_header_next_over_approximated_stack_root(&self) -> u32 {
        self.vm_drc_header_ref_count() + 8
    }
}

/// Magic value for core Wasm VM contexts.
///
/// This is stored at the start of all `VMContext` structures.
pub const VMCONTEXT_MAGIC: u32 = u32::from_le_bytes(*b"core");

/// Equivalent of `VMCONTEXT_MAGIC` except for array-call host functions.
///
/// This is stored at the start of all `VMArrayCallHostFuncContext` structures
/// and double-checked on `VMArrayCallHostFuncContext::from_opaque`.
pub const VM_ARRAY_CALL_HOST_FUNC_MAGIC: u32 = u32::from_le_bytes(*b"ACHF");

#[cfg(test)]
mod tests {
    use crate::vmoffsets::align;

    #[test]
    fn alignment() {
        fn is_aligned(x: u32) -> bool {
            x % 16 == 0
        }
        assert!(is_aligned(align(0, 16)));
        assert!(is_aligned(align(32, 16)));
        assert!(is_aligned(align(33, 16)));
        assert!(is_aligned(align(31, 16)));
    }
}
