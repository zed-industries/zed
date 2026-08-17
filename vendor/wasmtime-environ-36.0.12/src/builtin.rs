/// Helper macro to iterate over all builtin functions and their signatures.
#[macro_export]
macro_rules! foreach_builtin_function {
    ($mac:ident) => {
        $mac! {
            // Returns an index for wasm's `memory.grow` builtin function.
            memory_grow(vmctx: vmctx, delta: u64, index: u32) -> pointer;
            // Returns an index for wasm's `table.copy` when both tables are locally
            // defined.
            table_copy(vmctx: vmctx, dst_index: u32, src_index: u32, dst: u64, src: u64, len: u64) -> bool;
            // Returns an index for wasm's `table.init`.
            table_init(vmctx: vmctx, table: u32, elem: u32, dst: u64, src: u64, len: u64) -> bool;
            // Returns an index for wasm's `elem.drop`.
            elem_drop(vmctx: vmctx, elem: u32);
            // Returns an index for wasm's `memory.copy`
            memory_copy(vmctx: vmctx, dst_index: u32, dst: u64, src_index: u32, src: u64, len: u64) -> bool;
            // Returns an index for wasm's `memory.fill` instruction.
            memory_fill(vmctx: vmctx, memory: u32, dst: u64, val: u32, len: u64) -> bool;
            // Returns an index for wasm's `memory.init` instruction.
            memory_init(vmctx: vmctx, memory: u32, data: u32, dst: u64, src: u32, len: u32) -> bool;
            // Returns a value for wasm's `ref.func` instruction.
            ref_func(vmctx: vmctx, func: u32) -> pointer;
            // Returns an index for wasm's `data.drop` instruction.
            data_drop(vmctx: vmctx, data: u32);
            // Returns a table entry after lazily initializing it.
            table_get_lazy_init_func_ref(vmctx: vmctx, table: u32, index: u64) -> pointer;
            // Returns an index for Wasm's `table.grow` instruction for `funcref`s.
            table_grow_func_ref(vmctx: vmctx, table: u32, delta: u64, init: pointer) -> pointer;
            // Returns an index for Wasm's `table.fill` instruction for `funcref`s.
            table_fill_func_ref(vmctx: vmctx, table: u32, dst: u64, val: pointer, len: u64) -> bool;
            // Returns an index for wasm's `memory.atomic.notify` instruction.
            #[cfg(feature = "threads")]
            memory_atomic_notify(vmctx: vmctx, memory: u32, addr: u64, count: u32) -> u64;
            // Returns an index for wasm's `memory.atomic.wait32` instruction.
            #[cfg(feature = "threads")]
            memory_atomic_wait32(vmctx: vmctx, memory: u32, addr: u64, expected: u32, timeout: u64) -> u64;
            // Returns an index for wasm's `memory.atomic.wait64` instruction.
            #[cfg(feature = "threads")]
            memory_atomic_wait64(vmctx: vmctx, memory: u32, addr: u64, expected: u64, timeout: u64) -> u64;
            // Invoked when fuel has run out while executing a function.
            out_of_gas(vmctx: vmctx) -> bool;
            // Invoked when we reach a new epoch.
            #[cfg(target_has_atomic = "64")]
            new_epoch(vmctx: vmctx) -> u64;
            // Invoked before malloc returns.
            #[cfg(feature = "wmemcheck")]
            check_malloc(vmctx: vmctx, addr: u32, len: u32) -> bool;
            // Invoked before the free returns.
            #[cfg(feature = "wmemcheck")]
            check_free(vmctx: vmctx, addr: u32) -> bool;
            // Invoked before a load is executed.
            #[cfg(feature = "wmemcheck")]
            check_load(vmctx: vmctx, num_bytes: u32, addr: u32, offset: u32) -> bool;
            // Invoked before a store is executed.
            #[cfg(feature = "wmemcheck")]
            check_store(vmctx: vmctx, num_bytes: u32, addr: u32, offset: u32) -> bool;
            // Invoked after malloc is called.
            #[cfg(feature = "wmemcheck")]
            malloc_start(vmctx: vmctx);
            // Invoked after free is called.
            #[cfg(feature = "wmemcheck")]
            free_start(vmctx: vmctx);
            // Invoked when wasm stack pointer is updated.
            #[cfg(feature = "wmemcheck")]
            update_stack_pointer(vmctx: vmctx, value: u32);
            // Invoked before memory.grow is called.
            #[cfg(feature = "wmemcheck")]
            update_mem_size(vmctx: vmctx, num_bytes: u32);

            // Drop a non-stack GC reference (eg an overwritten table entry)
            // once it will no longer be used again. (Note: `val` is not of type
            // `reference` because it needn't appear in any stack maps, as it
            // must not be live after this call.)
            #[cfg(feature = "gc-drc")]
            drop_gc_ref(vmctx: vmctx, val: u32);

            // Grow the GC heap by `bytes_needed` bytes.
            //
            // Traps if growing the GC heap fails.
            #[cfg(feature = "gc-null")]
            grow_gc_heap(vmctx: vmctx, bytes_needed: u64) -> bool;

            // Allocate a new, uninitialized GC object and return a reference to
            // it.
            #[cfg(feature = "gc-drc")]
            gc_alloc_raw(
                vmctx: vmctx,
                kind: u32,
                module_interned_type_index: u32,
                size: u32,
                align: u32
            ) -> u32;

            // Intern a `funcref` into the GC heap, returning its
            // `FuncRefTableId`.
            //
            // This libcall may not GC.
            #[cfg(feature = "gc")]
            intern_func_ref_for_gc_heap(
                vmctx: vmctx,
                func_ref: pointer
            ) -> u64;

            // Get the raw `VMFuncRef` pointer associated with a
            // `FuncRefTableId` from an earlier `intern_func_ref_for_gc_heap`
            // call.
            //
            // This libcall may not GC.
            //
            // Passes in the `ModuleInternedTypeIndex` of the funcref's expected
            // type, or `ModuleInternedTypeIndex::reserved_value()` if we are
            // getting the function reference as an untyped `funcref` rather
            // than a typed `(ref $ty)`.
            //
            // TODO: We will want to eventually expose the table directly to
            // Wasm code, so that it doesn't need to make a libcall to go from
            // id to `VMFuncRef`. That will be a little tricky: it will also
            // require updating the pointer to the slab in the `VMContext` (or
            // `VMStoreContext` or wherever we put it) when the slab is
            // resized.
            #[cfg(feature = "gc")]
            get_interned_func_ref(
                vmctx: vmctx,
                func_ref_id: u32,
                module_interned_type_index: u32
            ) -> pointer;

            // Builtin implementation of the `array.new_data` instruction.
            #[cfg(feature = "gc")]
            array_new_data(
                vmctx: vmctx,
                array_interned_type_index: u32,
                data_index: u32,
                data_offset: u32,
                len: u32
            ) -> u32;

            // Builtin implementation of the `array.new_elem` instruction.
            #[cfg(feature = "gc")]
            array_new_elem(
                vmctx: vmctx,
                array_interned_type_index: u32,
                elem_index: u32,
                elem_offset: u32,
                len: u32
            ) -> u32;

            // Builtin implementation of the `array.copy` instruction.
            #[cfg(feature = "gc")]
            array_copy(
                vmctx: vmctx,
                dst_array: u32,
                dst_index: u32,
                src_array: u32,
                src_index: u32,
                len: u32
            ) -> bool;

            // Builtin implementation of the `array.init_data` instruction.
            #[cfg(feature = "gc")]
            array_init_data(
                vmctx: vmctx,
                array_interned_type_index: u32,
                array: u32,
                dst_index: u32,
                data_index: u32,
                data_offset: u32,
                len: u32
            ) -> bool;

            // Builtin implementation of the `array.init_elem` instruction.
            #[cfg(feature = "gc")]
            array_init_elem(
                vmctx: vmctx,
                array_interned_type_index: u32,
                array: u32,
                dst: u32,
                elem_index: u32,
                src: u32,
                len: u32
            ) -> bool;

            // Returns whether `actual_engine_type` is a subtype of
            // `expected_engine_type`.
            #[cfg(feature = "gc")]
            is_subtype(
                vmctx: vmctx,
                actual_engine_type: u32,
                expected_engine_type: u32
            ) -> u32;

            // Returns an index for Wasm's `table.grow` instruction for GC references.
            #[cfg(feature = "gc")]
            table_grow_gc_ref(vmctx: vmctx, table: u32, delta: u64, init: u32) -> pointer;

            // Returns an index for Wasm's `table.fill` instruction for GC references.
            #[cfg(feature = "gc")]
            table_fill_gc_ref(vmctx: vmctx, table: u32, dst: u64, val: u32, len: u64) -> bool;

            // Wasm floating-point routines for when the CPU instructions aren't available.
            ceil_f32(vmctx: vmctx, x: f32) -> f32;
            ceil_f64(vmctx: vmctx, x: f64) -> f64;
            floor_f32(vmctx: vmctx, x: f32) -> f32;
            floor_f64(vmctx: vmctx, x: f64) -> f64;
            trunc_f32(vmctx: vmctx, x: f32) -> f32;
            trunc_f64(vmctx: vmctx, x: f64) -> f64;
            nearest_f32(vmctx: vmctx, x: f32) -> f32;
            nearest_f64(vmctx: vmctx, x: f64) -> f64;
            i8x16_swizzle(vmctx: vmctx, a: i8x16, b: i8x16) -> i8x16;
            i8x16_shuffle(vmctx: vmctx, a: i8x16, b: i8x16, c: i8x16) -> i8x16;
            fma_f32x4(vmctx: vmctx, x: f32x4, y: f32x4, z: f32x4) -> f32x4;
            fma_f64x2(vmctx: vmctx, x: f64x2, y: f64x2, z: f64x2) -> f64x2;

            // Raises an unconditional trap with the specified code.
            //
            // This is used when signals-based-traps are disabled for backends
            // when an illegal instruction can't be executed for example.
            trap(vmctx: vmctx, code: u8);

            // Raises an unconditional trap where the trap information must have
            // been previously filled in.
            raise(vmctx: vmctx);

            // Creates a new continuation from a funcref.
            #[cfg(feature = "stack-switching")]
            cont_new(vmctx: vmctx, r: pointer, param_count: u32, result_count: u32) -> pointer;

            // Returns an index for Wasm's `table.grow` instruction
            // for `contobj`s.  Note that the initial
            // Option<VMContObj> (i.e., the value to fill the new
            // slots with) is split into two arguments: The underlying
            // continuation reference and the revision count.  To
            // denote the continuation being `None`, `init_contref`
            // may be 0.
            #[cfg(feature = "stack-switching")]
            table_grow_cont_obj(vmctx: vmctx, table: u32, delta: u64, init_contref: pointer, init_revision: u64) -> pointer;

            // `value_contref` and `value_revision` together encode
            // the Option<VMContObj>, as in previous libcall.
            #[cfg(feature = "stack-switching")]
            table_fill_cont_obj(vmctx: vmctx, table: u32, dst: u64, value_contref: pointer, value_revision: u64, len: u64) -> bool;
        }
    };
}

/// Helper macro to define a builtin type such as `BuiltinFunctionIndex` and
/// `ComponentBuiltinFunctionIndex` using the iterator macro, e.g.
/// `foreach_builtin_function`, as the way to generate accessor methods.
macro_rules! declare_builtin_index {
    ($index_name:ident, $iter:ident) => {
        /// An index type for builtin functions.
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $index_name(u32);

        impl $index_name {
            /// Create a new builtin from its raw index
            pub const fn from_u32(i: u32) -> Self {
                assert!(i < Self::len());
                Self(i)
            }

            /// Return the index as an u32 number.
            pub const fn index(&self) -> u32 {
                self.0
            }

            $iter!(declare_builtin_index_constructors);
        }
    };
}

/// Helper macro used by the above macro.
macro_rules! declare_builtin_index_constructors {
    (
        $(
            $( #[$attr:meta] )*
            $name:ident( $( $pname:ident: $param:ident ),* ) $( -> $result:ident )?;
        )*
    ) => {
        declare_builtin_index_constructors!(
            @indices;
            0;
            $( $( #[$attr] )* $name; )*
        );

        /// Returns a symbol name for this builtin.
        pub fn name(&self) -> &'static str {
            $(
                if *self == Self::$name() {
                    return stringify!($name);
                }
            )*
            unreachable!()
        }
    };

    // Base case: no more indices to declare, so define the total number of
    // function indices.
    (
        @indices;
        $len:expr;
    ) => {
        /// Returns the total number of builtin functions.
        pub const fn len() -> u32 {
            $len
        }
    };

    // Recursive case: declare the next index, and then keep declaring the rest of
    // the indices.
    (
         @indices;
         $index:expr;
         $( #[$this_attr:meta] )*
         $this_name:ident;
         $(
             $( #[$rest_attr:meta] )*
             $rest_name:ident;
         )*
    ) => {
        #[expect(missing_docs, reason = "macro-generated")]
        pub const fn $this_name() -> Self {
            Self($index)
        }

        declare_builtin_index_constructors!(
            @indices;
            ($index + 1);
            $( $( #[$rest_attr] )* $rest_name; )*
        );
    }
}

// Define `struct BuiltinFunctionIndex`
declare_builtin_index!(BuiltinFunctionIndex, foreach_builtin_function);

/// Return value of [`BuiltinFunctionIndex::trap_sentinel`].
pub enum TrapSentinel {
    /// A falsy or zero value indicates a trap.
    Falsy,
    /// The value `-2` indicates a trap (used for growth-related builtins).
    NegativeTwo,
    /// The value `-1` indicates a trap .
    NegativeOne,
    /// Any negative value indicates a trap.
    Negative,
}

impl BuiltinFunctionIndex {
    /// Describes the return value of this builtin and what represents a trap.
    ///
    /// Libcalls don't raise traps themselves and instead delegate to compilers
    /// to do so. This means that some return values of libcalls indicate a trap
    /// is happening and this is represented with sentinel values. This function
    /// returns the description of the sentinel value which indicates a trap, if
    /// any. If `None` is returned from this function then this builtin cannot
    /// generate a trap.
    #[allow(unreachable_code, unused_macro_rules, reason = "macro-generated code")]
    pub fn trap_sentinel(&self) -> Option<TrapSentinel> {
        macro_rules! trap_sentinel {
            (
                $(
                    $( #[$attr:meta] )*
                    $name:ident( $( $pname:ident: $param:ident ),* ) $( -> $result:ident )?;
                )*
            ) => {{
                $(
                    $(#[$attr])*
                    if *self == BuiltinFunctionIndex::$name() {
                        let mut _ret = None;
                        $(_ret = Some(trap_sentinel!(@get $name $result));)?
                        return _ret;
                    }
                )*

                None
            }};

            // Growth-related functions return -2 as a sentinel.
            (@get memory_grow pointer) => (TrapSentinel::NegativeTwo);
            (@get table_grow_func_ref pointer) => (TrapSentinel::NegativeTwo);
            (@get table_grow_gc_ref pointer) => (TrapSentinel::NegativeTwo);
            (@get table_grow_cont_obj pointer) => (TrapSentinel::NegativeTwo);

            // Atomics-related functions return a negative value indicating trap
            // indicate a trap.
            (@get memory_atomic_notify u64) => (TrapSentinel::Negative);
            (@get memory_atomic_wait32 u64) => (TrapSentinel::Negative);
            (@get memory_atomic_wait64 u64) => (TrapSentinel::Negative);

            // GC returns an optional GC ref, encoded as a `u64` with a negative
            // value indicating a trap.
            (@get gc u64) => (TrapSentinel::Negative);

            // GC allocation functions return a u32 which is zero to indicate a
            // trap.
            (@get gc_alloc_raw u32) => (TrapSentinel::Falsy);
            (@get array_new_data u32) => (TrapSentinel::Falsy);
            (@get array_new_elem u32) => (TrapSentinel::Falsy);

            // The final epoch represents a trap
            (@get new_epoch u64) => (TrapSentinel::NegativeOne);

            // These libcalls can't trap
            (@get ref_func pointer) => (return None);
            (@get table_get_lazy_init_func_ref pointer) => (return None);
            (@get get_interned_func_ref pointer) => (return None);
            (@get intern_func_ref_for_gc_heap u64) => (return None);
            (@get is_subtype u32) => (return None);
            (@get ceil_f32 f32) => (return None);
            (@get ceil_f64 f64) => (return None);
            (@get floor_f32 f32) => (return None);
            (@get floor_f64 f64) => (return None);
            (@get trunc_f32 f32) => (return None);
            (@get trunc_f64 f64) => (return None);
            (@get nearest_f32 f32) => (return None);
            (@get nearest_f64 f64) => (return None);
            (@get i8x16_swizzle i8x16) => (return None);
            (@get i8x16_shuffle i8x16) => (return None);
            (@get fma_f32x4 f32x4) => (return None);
            (@get fma_f64x2 f64x2) => (return None);

            (@get cont_new pointer) => (TrapSentinel::Negative);

            // Bool-returning functions use `false` as an indicator of a trap.
            (@get $name:ident bool) => (TrapSentinel::Falsy);

            (@get $name:ident $ret:ident) => (
                compile_error!(concat!("no trap sentinel registered for ", stringify!($name)))
            )
        }

        foreach_builtin_function!(trap_sentinel)
    }
}
