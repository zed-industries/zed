//! Implementation of Wasm to CLIF memory access translation.
//!
//! Given
//!
//! * a dynamic Wasm memory index operand,
//! * a static offset immediate, and
//! * a static access size,
//!
//! bounds check the memory access and translate it into a native memory access.
//!
//! !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//! !!!                                                                      !!!
//! !!!    THIS CODE IS VERY SUBTLE, HAS MANY SPECIAL CASES, AND IS ALSO     !!!
//! !!!   ABSOLUTELY CRITICAL FOR MAINTAINING THE SAFETY OF THE WASM HEAP    !!!
//! !!!                             SANDBOX.                                 !!!
//! !!!                                                                      !!!
//! !!!    A good rule of thumb is to get two reviews on any substantive     !!!
//! !!!                         changes in here.                             !!!
//! !!!                                                                      !!!
//! !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::{
    Reachability,
    func_environ::FuncEnvironment,
    translate::{HeapData, TargetEnvironment},
};
use Reachability::*;
use cranelift_codegen::{
    cursor::{Cursor, FuncCursor},
    ir::{self, InstBuilder, RelSourceLoc, condcodes::IntCC},
    ir::{Expr, Fact},
};
use cranelift_frontend::FunctionBuilder;
use wasmtime_environ::Unsigned;

/// The kind of bounds check to perform when accessing a Wasm linear memory or
/// GC heap.
///
/// Prefer `BoundsCheck::*WholeObject` over `BoundsCheck::Field` when possible,
/// as that approach allows the mid-end to deduplicate bounds checks across
/// multiple accesses to the same GC object.
#[derive(Debug)]
pub enum BoundsCheck {
    /// Check that this one access in particular is in bounds:
    ///
    /// ```ignore
    /// index + offset + access_size <= bound
    /// ```
    StaticOffset { offset: u32, access_size: u8 },

    /// Assuming the precondition `offset + access_size <= object_size`, check
    /// that this whole object is in bounds:
    ///
    /// ```ignore
    /// index + object_size <= bound
    /// ```
    #[cfg(feature = "gc")]
    StaticObjectField {
        offset: u32,
        access_size: u8,
        object_size: u32,
    },

    /// Like `StaticWholeObject` but with dynamic offset and object size.
    ///
    /// It is *your* responsibility to ensure that the `offset + access_size <=
    /// object_size` precondition holds.
    #[cfg(feature = "gc")]
    DynamicObjectField {
        offset: ir::Value,
        object_size: ir::Value,
    },
}

/// Helper used to emit bounds checks (as necessary) and compute the native
/// address of a heap access.
///
/// Returns the `ir::Value` holding the native address of the heap access, or
/// `Reachability::Unreachable` if the heap access will unconditionally trap and
/// any subsequent code in this basic block is unreachable.
pub fn bounds_check_and_compute_addr(
    builder: &mut FunctionBuilder,
    env: &mut FuncEnvironment<'_>,
    heap: &HeapData,
    index: ir::Value,
    bounds_check: BoundsCheck,
    trap: ir::TrapCode,
) -> Reachability<ir::Value> {
    match bounds_check {
        BoundsCheck::StaticOffset {
            offset,
            access_size,
        } => bounds_check_field_access(builder, env, heap, index, offset, access_size, trap),

        #[cfg(feature = "gc")]
        BoundsCheck::StaticObjectField {
            offset,
            access_size,
            object_size,
        } => {
            // Assert that the precondition holds.
            let offset_and_access_size = offset.checked_add(access_size.into()).unwrap();
            assert!(offset_and_access_size <= object_size);

            // When we can, pretend that we are doing one big access of the
            // whole object all at once. This enables better GVN for repeated
            // accesses of the same object.
            if let Ok(object_size) = u8::try_from(object_size) {
                let obj_ptr = match bounds_check_field_access(
                    builder,
                    env,
                    heap,
                    index,
                    0,
                    object_size,
                    trap,
                ) {
                    Reachable(v) => v,
                    u @ Unreachable => return u,
                };
                let offset = builder.ins().iconst(env.pointer_type(), i64::from(offset));
                let field_ptr = builder.ins().iadd(obj_ptr, offset);
                return Reachable(field_ptr);
            }

            // Otherwise, bounds check just this one field's access.
            bounds_check_field_access(builder, env, heap, index, offset, access_size, trap)
        }

        // Compute the index of the end of the object, bounds check that and get
        // a pointer to just after the object, and then reverse offset from that
        // to get the pointer to the field being accessed.
        #[cfg(feature = "gc")]
        BoundsCheck::DynamicObjectField {
            offset,
            object_size,
        } => {
            assert_eq!(heap.index_type(), ir::types::I32);
            assert_eq!(builder.func.dfg.value_type(index), ir::types::I32);
            assert_eq!(builder.func.dfg.value_type(offset), ir::types::I32);
            assert_eq!(builder.func.dfg.value_type(object_size), ir::types::I32);

            let index_and_object_size = builder.ins().uadd_overflow_trap(index, object_size, trap);
            let ptr_just_after_obj = match bounds_check_field_access(
                builder,
                env,
                heap,
                index_and_object_size,
                0,
                0,
                trap,
            ) {
                Reachable(v) => v,
                u @ Unreachable => return u,
            };

            let backwards_offset = builder.ins().isub(object_size, offset);
            let backwards_offset = cast_index_to_pointer_ty(
                backwards_offset,
                ir::types::I32,
                env.pointer_type(),
                false,
                &mut builder.cursor(),
                trap,
            );

            let field_ptr = builder.ins().isub(ptr_just_after_obj, backwards_offset);
            Reachable(field_ptr)
        }
    }
}

fn bounds_check_field_access(
    builder: &mut FunctionBuilder,
    env: &mut FuncEnvironment<'_>,
    heap: &HeapData,
    index: ir::Value,
    offset: u32,
    access_size: u8,
    trap: ir::TrapCode,
) -> Reachability<ir::Value> {
    let pointer_bit_width = u16::try_from(env.pointer_type().bits()).unwrap();
    let bound_gv = heap.bound;
    let orig_index = index;
    let clif_memory_traps_enabled = env.clif_memory_traps_enabled();
    let spectre_mitigations_enabled =
        env.heap_access_spectre_mitigation() && clif_memory_traps_enabled;
    let pcc = env.proof_carrying_code();

    let host_page_size_log2 = env.target_config().page_size_align_log2;
    let can_use_virtual_memory = heap
        .memory
        .can_use_virtual_memory(env.tunables(), host_page_size_log2)
        && clif_memory_traps_enabled;
    let can_elide_bounds_check = heap
        .memory
        .can_elide_bounds_check(env.tunables(), host_page_size_log2)
        && clif_memory_traps_enabled;
    let memory_guard_size = env.tunables().memory_guard_size;
    let memory_reservation = env.tunables().memory_reservation;

    let offset_and_size = offset_plus_size(offset, access_size);
    let statically_in_bounds = statically_in_bounds(&builder.func, heap, index, offset_and_size);

    let index = cast_index_to_pointer_ty(
        index,
        heap.index_type(),
        env.pointer_type(),
        heap.pcc_memory_type.is_some(),
        &mut builder.cursor(),
        trap,
    );

    let oob_behavior = if spectre_mitigations_enabled {
        OobBehavior::ConditionallyLoadFromZero {
            select_spectre_guard: true,
        }
    } else if env.load_from_zero_allowed() {
        OobBehavior::ConditionallyLoadFromZero {
            select_spectre_guard: false,
        }
    } else {
        OobBehavior::ExplicitTrap
    };

    let make_compare = |builder: &mut FunctionBuilder,
                        compare_kind: IntCC,
                        lhs: ir::Value,
                        lhs_off: Option<i64>,
                        rhs: ir::Value,
                        rhs_off: Option<i64>| {
        let result = builder.ins().icmp(compare_kind, lhs, rhs);
        if pcc {
            // Name the original value as a def of the SSA value;
            // if the value was extended, name that as well with a
            // dynamic range, overwriting the basic full-range
            // fact that we previously put on the uextend.
            builder.func.dfg.facts[orig_index] = Some(Fact::Def { value: orig_index });
            if index != orig_index {
                builder.func.dfg.facts[index] = Some(Fact::value(pointer_bit_width, orig_index));
            }

            // Create a fact on the LHS that is a "trivial symbolic
            // fact": v1 has range v1+LHS_off..=v1+LHS_off
            builder.func.dfg.facts[lhs] = Some(Fact::value_offset(
                pointer_bit_width,
                orig_index,
                lhs_off.unwrap(),
            ));
            // If the RHS is a symbolic value (v1 or gv1), we can
            // emit a Compare fact.
            if let Some(rhs) = builder.func.dfg.facts[rhs]
                .as_ref()
                .and_then(|f| f.as_symbol())
            {
                builder.func.dfg.facts[result] = Some(Fact::Compare {
                    kind: compare_kind,
                    lhs: Expr::offset(&Expr::value(orig_index), lhs_off.unwrap()).unwrap(),
                    rhs: Expr::offset(rhs, rhs_off.unwrap()).unwrap(),
                });
            }
            // Likewise, if the RHS is a constant, we can emit a
            // Compare fact.
            if let Some(k) = builder.func.dfg.facts[rhs]
                .as_ref()
                .and_then(|f| f.as_const(pointer_bit_width))
            {
                builder.func.dfg.facts[result] = Some(Fact::Compare {
                    kind: compare_kind,
                    lhs: Expr::offset(&Expr::value(orig_index), lhs_off.unwrap()).unwrap(),
                    rhs: Expr::constant((k as i64).checked_add(rhs_off.unwrap()).unwrap()),
                });
            }
        }
        result
    };

    // We need to emit code that will trap (or compute an address that will trap
    // when accessed) if
    //
    //     index + offset + access_size > bound
    //
    // or if the `index + offset + access_size` addition overflows.
    //
    // Note that we ultimately want a 64-bit integer (we only target 64-bit
    // architectures at the moment) and that `offset` is a `u32` and
    // `access_size` is a `u8`. This means that we can add the latter together
    // as `u64`s without fear of overflow, and we only have to be concerned with
    // whether adding in `index` will overflow.
    //
    // Finally, the following if/else chains do have a little
    // bit of duplicated code across them, but I think writing it this way is
    // worth it for readability and seeing very clearly each of our cases for
    // different bounds checks and optimizations of those bounds checks. It is
    // intentionally written in a straightforward case-matching style that will
    // hopefully make it easy to port to ISLE one day.
    if offset_and_size > heap.memory.maximum_byte_size().unwrap_or(u64::MAX) {
        // Special case: trap immediately if `offset + access_size >
        // max_memory_size`, since we will end up being out-of-bounds regardless
        // of the given `index`.
        env.before_unconditionally_trapping_memory_access(builder);
        env.trap(builder, trap);
        return Unreachable;
    }

    // Special case: if this is a 32-bit platform and the `offset_and_size`
    // overflows the 32-bit address space then there's no hope of this ever
    // being in-bounds. We can't represent `offset_and_size` in CLIF as the
    // native pointer type anyway, so this is an unconditional trap.
    if pointer_bit_width < 64 && offset_and_size >= (1 << pointer_bit_width) {
        env.before_unconditionally_trapping_memory_access(builder);
        env.trap(builder, trap);
        return Unreachable;
    }

    // Special case for when we can completely omit explicit
    // bounds checks for 32-bit memories.
    //
    // First, let's rewrite our comparison to move all of the constants
    // to one side:
    //
    //         index + offset + access_size > bound
    //     ==> index > bound - (offset + access_size)
    //
    // We know the subtraction on the right-hand side won't wrap because
    // we didn't hit the unconditional trap case above.
    //
    // Additionally, we add our guard pages (if any) to the right-hand
    // side, since we can rely on the virtual memory subsystem at runtime
    // to catch out-of-bound accesses within the range `bound .. bound +
    // guard_size`. So now we are dealing with
    //
    //     index > bound + guard_size - (offset + access_size)
    //
    // Note that `bound + guard_size` cannot overflow for
    // correctly-configured heaps, as otherwise the heap wouldn't fit in
    // a 64-bit memory space.
    //
    // The complement of our should-this-trap comparison expression is
    // the should-this-not-trap comparison expression:
    //
    //     index <= bound + guard_size - (offset + access_size)
    //
    // If we know the right-hand side is greater than or equal to
    // `u32::MAX`, then
    //
    //     index <= u32::MAX <= bound + guard_size - (offset + access_size)
    //
    // This expression is always true when the heap is indexed with
    // 32-bit integers because `index` cannot be larger than
    // `u32::MAX`. This means that `index` is always either in bounds or
    // within the guard page region, neither of which require emitting an
    // explicit bounds check.
    if can_elide_bounds_check
        && u64::from(u32::MAX) <= memory_reservation + memory_guard_size - offset_and_size
    {
        assert!(heap.index_type() == ir::types::I32);
        assert!(
            can_use_virtual_memory,
            "static memories require the ability to use virtual memory"
        );
        return Reachable(compute_addr(
            &mut builder.cursor(),
            heap,
            env.pointer_type(),
            index,
            offset,
            AddrPcc::static32(heap.pcc_memory_type, memory_reservation + memory_guard_size),
        ));
    }

    // Special case when the `index` is a constant and statically known to be
    // in-bounds on this memory, no bounds checks necessary.
    if statically_in_bounds {
        return Reachable(compute_addr(
            &mut builder.cursor(),
            heap,
            env.pointer_type(),
            index,
            offset,
            AddrPcc::static32(heap.pcc_memory_type, memory_reservation + memory_guard_size),
        ));
    }

    // Special case for when we can rely on virtual memory, the minimum
    // byte size of this memory fits within the memory reservation, and
    // memory isn't allowed to move. In this situation we know that
    // memory will statically not grow beyond `memory_reservation` so we
    // and we know that memory from 0 to that limit is guaranteed to be
    // valid or trap. Here we effectively assume that the dynamic size
    // of linear memory is its maximal value, `memory_reservation`, and
    // we can avoid loading the actual length of memory.
    //
    // We have to explicitly test whether
    //
    //     index > bound - (offset + access_size)
    //
    // and trap if so.
    //
    // Since we have to emit explicit bounds checks, we might as well be
    // precise, not rely on the virtual memory subsystem at all, and not
    // factor in the guard pages here.
    if can_use_virtual_memory
        && heap.memory.minimum_byte_size().unwrap_or(u64::MAX) <= memory_reservation
        && !heap.memory.memory_may_move(env.tunables())
    {
        let adjusted_bound = memory_reservation.checked_sub(offset_and_size).unwrap();
        let adjusted_bound_value = builder
            .ins()
            .iconst(env.pointer_type(), adjusted_bound as i64);
        if pcc {
            builder.func.dfg.facts[adjusted_bound_value] =
                Some(Fact::constant(pointer_bit_width, adjusted_bound));
        }
        let oob = make_compare(
            builder,
            IntCC::UnsignedGreaterThan,
            index,
            Some(0),
            adjusted_bound_value,
            Some(0),
        );
        return Reachable(explicit_check_oob_condition_and_compute_addr(
            env,
            builder,
            heap,
            index,
            offset,
            access_size,
            oob_behavior,
            AddrPcc::static32(heap.pcc_memory_type, memory_reservation),
            oob,
            trap,
        ));
    }

    // Special case for when `offset + access_size == 1`:
    //
    //         index + 1 > bound
    //     ==> index >= bound
    //
    // Note that this special case is skipped for Pulley targets to assist with
    // pattern-matching bounds checks into single instructions. Otherwise more
    // patterns/instructions would have to be added to match this. In the end
    // the goal is to emit one instruction anyway, so this optimization is
    // largely only applicable for native platforms.
    if offset_and_size == 1 && !env.is_pulley() {
        let bound = get_dynamic_heap_bound(builder, env, heap);
        let oob = make_compare(
            builder,
            IntCC::UnsignedGreaterThanOrEqual,
            index,
            Some(0),
            bound,
            Some(0),
        );
        return Reachable(explicit_check_oob_condition_and_compute_addr(
            env,
            builder,
            heap,
            index,
            offset,
            access_size,
            oob_behavior,
            AddrPcc::dynamic(heap.pcc_memory_type, bound_gv),
            oob,
            trap,
        ));
    }

    // Special case for when we know that there are enough guard
    // pages to cover the offset and access size.
    //
    // The precise should-we-trap condition is
    //
    //     index + offset + access_size > bound
    //
    // However, if we instead check only the partial condition
    //
    //     index > bound
    //
    // then the most out of bounds that the access can be, while that
    // partial check still succeeds, is `offset + access_size`.
    //
    // However, when we have a guard region that is at least as large as
    // `offset + access_size`, we can rely on the virtual memory
    // subsystem handling these out-of-bounds errors at
    // runtime. Therefore, the partial `index > bound` check is
    // sufficient for this heap configuration.
    //
    // Additionally, this has the advantage that a series of Wasm loads
    // that use the same dynamic index operand but different static
    // offset immediates -- which is a common code pattern when accessing
    // multiple fields in the same struct that is in linear memory --
    // will all emit the same `index > bound` check, which we can GVN.
    if can_use_virtual_memory && offset_and_size <= memory_guard_size {
        let bound = get_dynamic_heap_bound(builder, env, heap);
        let oob = make_compare(
            builder,
            IntCC::UnsignedGreaterThan,
            index,
            Some(0),
            bound,
            Some(0),
        );
        return Reachable(explicit_check_oob_condition_and_compute_addr(
            env,
            builder,
            heap,
            index,
            offset,
            access_size,
            oob_behavior,
            AddrPcc::dynamic(heap.pcc_memory_type, bound_gv),
            oob,
            trap,
        ));
    }

    // Special case for when `offset + access_size <= min_size`.
    //
    // We know that `bound >= min_size`, so we can do the following
    // comparison, without fear of the right-hand side wrapping around:
    //
    //         index + offset + access_size > bound
    //     ==> index > bound - (offset + access_size)
    if offset_and_size <= heap.memory.minimum_byte_size().unwrap_or(u64::MAX) {
        let bound = get_dynamic_heap_bound(builder, env, heap);
        let adjustment = offset_and_size as i64;
        let adjustment_value = builder.ins().iconst(env.pointer_type(), adjustment);
        if pcc {
            builder.func.dfg.facts[adjustment_value] =
                Some(Fact::constant(pointer_bit_width, offset_and_size));
        }
        let adjusted_bound = builder.ins().isub(bound, adjustment_value);
        if pcc {
            builder.func.dfg.facts[adjusted_bound] = Some(Fact::global_value_offset(
                pointer_bit_width,
                bound_gv,
                -adjustment,
            ));
        }
        let oob = make_compare(
            builder,
            IntCC::UnsignedGreaterThan,
            index,
            Some(0),
            adjusted_bound,
            Some(adjustment),
        );
        return Reachable(explicit_check_oob_condition_and_compute_addr(
            env,
            builder,
            heap,
            index,
            offset,
            access_size,
            oob_behavior,
            AddrPcc::dynamic(heap.pcc_memory_type, bound_gv),
            oob,
            trap,
        ));
    }

    // General case for dynamic bounds checks:
    //
    //     index + offset + access_size > bound
    //
    // And we have to handle the overflow case in the left-hand side.
    let access_size_val = builder
        .ins()
        // Explicit cast from u64 to i64: we just want the raw
        // bits, and iconst takes an `Imm64`.
        .iconst(env.pointer_type(), offset_and_size as i64);
    if pcc {
        builder.func.dfg.facts[access_size_val] =
            Some(Fact::constant(pointer_bit_width, offset_and_size));
    }
    let adjusted_index = env.uadd_overflow_trap(builder, index, access_size_val, trap);
    if pcc {
        builder.func.dfg.facts[adjusted_index] = Some(Fact::value_offset(
            pointer_bit_width,
            index,
            i64::try_from(offset_and_size).unwrap(),
        ));
    }
    let bound = get_dynamic_heap_bound(builder, env, heap);
    let oob = make_compare(
        builder,
        IntCC::UnsignedGreaterThan,
        adjusted_index,
        i64::try_from(offset_and_size).ok(),
        bound,
        Some(0),
    );
    Reachable(explicit_check_oob_condition_and_compute_addr(
        env,
        builder,
        heap,
        index,
        offset,
        access_size,
        oob_behavior,
        AddrPcc::dynamic(heap.pcc_memory_type, bound_gv),
        oob,
        trap,
    ))
}

/// Get the bound of a dynamic heap as an `ir::Value`.
fn get_dynamic_heap_bound(
    builder: &mut FunctionBuilder,
    env: &mut FuncEnvironment<'_>,
    heap: &HeapData,
) -> ir::Value {
    let enable_pcc = heap.pcc_memory_type.is_some();

    let (value, gv) = match heap.memory.static_heap_size() {
        // The heap has a constant size, no need to actually load the
        // bound.  TODO: this is currently disabled for PCC because we
        // can't easily prove that the GV load indeed results in a
        // constant (that information is lost in the CLIF). We'll want
        // to create an `iconst` GV expression kind to reify this fact
        // in the GV, then re-enable this opt. (Or, alternately,
        // compile such memories with a static-bound memtype and
        // facts.)
        Some(max_size) if !enable_pcc => (
            builder.ins().iconst(env.pointer_type(), max_size as i64),
            heap.bound,
        ),

        // Load the heap bound from its global variable.
        _ => (
            builder.ins().global_value(env.pointer_type(), heap.bound),
            heap.bound,
        ),
    };

    // If proof-carrying code is enabled, apply a fact to the range to
    // tie it to the GV.
    if enable_pcc {
        builder.func.dfg.facts[value] = Some(Fact::global_value(
            u16::try_from(env.pointer_type().bits()).unwrap(),
            gv,
        ));
    }

    value
}

fn cast_index_to_pointer_ty(
    index: ir::Value,
    index_ty: ir::Type,
    pointer_ty: ir::Type,
    pcc: bool,
    pos: &mut FuncCursor,
    trap: ir::TrapCode,
) -> ir::Value {
    if index_ty == pointer_ty {
        return index;
    }

    // If the index size is larger than the pointer, that means that this is a
    // 32-bit host platform with a 64-bit wasm linear memory. If the index is
    // larger than 2**32 then that's guaranteed to be out-of-bounds, otherwise we
    // `ireduce` the index.
    //
    // Also note that at this time this branch doesn't support pcc nor the
    // value-label-ranges of the below path.
    //
    // Finally, note that the returned `low_bits` here are still subject to an
    // explicit bounds check in wasm so in terms of Spectre speculation on
    // either side of the `trapnz` should be ok.
    if index_ty.bits() > pointer_ty.bits() {
        assert_eq!(index_ty, ir::types::I64);
        assert_eq!(pointer_ty, ir::types::I32);
        let low_bits = pos.ins().ireduce(pointer_ty, index);
        let c32 = pos.ins().iconst(pointer_ty, 32);
        let high_bits = pos.ins().ushr(index, c32);
        let high_bits = pos.ins().ireduce(pointer_ty, high_bits);
        pos.ins().trapnz(high_bits, trap);
        return low_bits;
    }

    // Convert `index` to `addr_ty`.
    let extended_index = pos.ins().uextend(pointer_ty, index);

    // Add a range fact on the extended value.
    if pcc {
        pos.func.dfg.facts[extended_index] = Some(Fact::max_range_for_width_extended(
            u16::try_from(index_ty.bits()).unwrap(),
            u16::try_from(pointer_ty.bits()).unwrap(),
        ));
    }

    // Add debug value-label alias so that debuginfo can name the extended
    // value as the address
    let loc = pos.srcloc();
    let loc = RelSourceLoc::from_base_offset(pos.func.params.base_srcloc(), loc);
    pos.func
        .stencil
        .dfg
        .add_value_label_alias(extended_index, loc, index);

    extended_index
}

/// Which facts do we want to emit for proof-carrying code, if any, on
/// address computations?
#[derive(Clone, Copy, Debug)]
enum AddrPcc {
    /// A 32-bit static memory with the given size.
    Static32(ir::MemoryType, u64),
    /// Dynamic bounds-check, with actual memory size (the `GlobalValue`)
    /// expressed symbolically.
    Dynamic(ir::MemoryType, ir::GlobalValue),
}
impl AddrPcc {
    fn static32(memory_type: Option<ir::MemoryType>, size: u64) -> Option<Self> {
        memory_type.map(|ty| AddrPcc::Static32(ty, size))
    }
    fn dynamic(memory_type: Option<ir::MemoryType>, bound: ir::GlobalValue) -> Option<Self> {
        memory_type.map(|ty| AddrPcc::Dynamic(ty, bound))
    }
}

/// What to do on out-of-bounds for the
/// `explicit_check_oob_condition_and_compute_addr` function below.
enum OobBehavior {
    /// An explicit `trapnz` instruction should be used.
    ExplicitTrap,
    /// A load from NULL should be issued if the address is out-of-bounds.
    ConditionallyLoadFromZero {
        /// Whether or not to use `select_spectre_guard` to choose the address
        /// to load from. If `false` then a normal `select` is used.
        select_spectre_guard: bool,
    },
}

/// Emit explicit checks on the given out-of-bounds condition for the Wasm
/// address and return the native address.
///
/// This function deduplicates explicit bounds checks and Spectre mitigations
/// that inherently also implement bounds checking.
fn explicit_check_oob_condition_and_compute_addr(
    env: &mut FuncEnvironment<'_>,
    builder: &mut FunctionBuilder,
    heap: &HeapData,
    index: ir::Value,
    offset: u32,
    access_size: u8,
    oob_behavior: OobBehavior,
    // Whether we're emitting PCC facts.
    pcc: Option<AddrPcc>,
    // The `i8` boolean value that is non-zero when the heap access is out of
    // bounds (and therefore we should trap) and is zero when the heap access is
    // in bounds (and therefore we can proceed).
    oob_condition: ir::Value,
    trap: ir::TrapCode,
) -> ir::Value {
    if let OobBehavior::ExplicitTrap = oob_behavior {
        env.trapnz(builder, oob_condition, trap);
    }
    let addr_ty = env.pointer_type();

    let mut addr = compute_addr(&mut builder.cursor(), heap, addr_ty, index, offset, pcc);

    if let OobBehavior::ConditionallyLoadFromZero {
        select_spectre_guard,
    } = oob_behavior
    {
        // These mitigations rely on trapping when loading from NULL so
        // CLIF memory instruction traps must be allowed for this to be
        // generated.
        assert!(env.load_from_zero_allowed());
        let null = builder.ins().iconst(addr_ty, 0);
        addr = if select_spectre_guard {
            builder
                .ins()
                .select_spectre_guard(oob_condition, null, addr)
        } else {
            builder.ins().select(oob_condition, null, addr)
        };

        match pcc {
            None => {}
            Some(AddrPcc::Static32(ty, size)) => {
                builder.func.dfg.facts[null] =
                    Some(Fact::constant(u16::try_from(addr_ty.bits()).unwrap(), 0));
                builder.func.dfg.facts[addr] = Some(Fact::Mem {
                    ty,
                    min_offset: 0,
                    max_offset: size.checked_sub(u64::from(access_size)).unwrap(),
                    nullable: true,
                });
            }
            Some(AddrPcc::Dynamic(ty, gv)) => {
                builder.func.dfg.facts[null] =
                    Some(Fact::constant(u16::try_from(addr_ty.bits()).unwrap(), 0));
                builder.func.dfg.facts[addr] = Some(Fact::DynamicMem {
                    ty,
                    min: Expr::constant(0),
                    max: Expr::offset(
                        &Expr::global_value(gv),
                        i64::try_from(env.tunables().memory_guard_size)
                            .unwrap()
                            .checked_sub(i64::from(access_size))
                            .unwrap(),
                    )
                    .unwrap(),
                    nullable: true,
                });
            }
        }
    }

    addr
}

/// Emit code for the native address computation of a Wasm address,
/// without any bounds checks or overflow checks.
///
/// It is the caller's responsibility to ensure that any necessary bounds and
/// overflow checks are emitted, and that the resulting address is never used
/// unless they succeed.
fn compute_addr(
    pos: &mut FuncCursor,
    heap: &HeapData,
    addr_ty: ir::Type,
    index: ir::Value,
    offset: u32,
    pcc: Option<AddrPcc>,
) -> ir::Value {
    debug_assert_eq!(pos.func.dfg.value_type(index), addr_ty);

    let heap_base = pos.ins().global_value(addr_ty, heap.base);

    match pcc {
        None => {}
        Some(AddrPcc::Static32(ty, _size)) => {
            pos.func.dfg.facts[heap_base] = Some(Fact::Mem {
                ty,
                min_offset: 0,
                max_offset: 0,
                nullable: false,
            });
        }
        Some(AddrPcc::Dynamic(ty, _limit)) => {
            pos.func.dfg.facts[heap_base] = Some(Fact::dynamic_base_ptr(ty));
        }
    }

    let base_and_index = pos.ins().iadd(heap_base, index);

    match pcc {
        None => {}
        Some(AddrPcc::Static32(ty, _) | AddrPcc::Dynamic(ty, _)) => {
            if let Some(idx) = pos.func.dfg.facts[index]
                .as_ref()
                .and_then(|f| f.as_symbol())
                .cloned()
            {
                pos.func.dfg.facts[base_and_index] = Some(Fact::DynamicMem {
                    ty,
                    min: idx.clone(),
                    max: idx,
                    nullable: false,
                });
            } else {
                pos.func.dfg.facts[base_and_index] = Some(Fact::Mem {
                    ty,
                    min_offset: 0,
                    max_offset: u64::from(u32::MAX),
                    nullable: false,
                });
            }
        }
    }

    if offset == 0 {
        base_and_index
    } else {
        // NB: The addition of the offset immediate must happen *before* the
        // `select_spectre_guard`, if any. If it happens after, then we
        // potentially are letting speculative execution read the whole first
        // 4GiB of memory.
        let offset_val = pos.ins().iconst(addr_ty, i64::from(offset));

        if pcc.is_some() {
            pos.func.dfg.facts[offset_val] = Some(Fact::constant(
                u16::try_from(addr_ty.bits()).unwrap(),
                u64::from(offset),
            ));
        }

        let result = pos.ins().iadd(base_and_index, offset_val);

        match pcc {
            None => {}
            Some(AddrPcc::Static32(ty, _) | AddrPcc::Dynamic(ty, _)) => {
                if let Some(idx) = pos.func.dfg.facts[index]
                    .as_ref()
                    .and_then(|f| f.as_symbol())
                {
                    pos.func.dfg.facts[result] = Some(Fact::DynamicMem {
                        ty,
                        min: idx.clone(),
                        // Safety: adding an offset to an expression with
                        // zero offset -- add cannot wrap, so `unwrap()`
                        // cannot fail.
                        max: Expr::offset(idx, i64::from(offset)).unwrap(),
                        nullable: false,
                    });
                } else {
                    pos.func.dfg.facts[result] = Some(Fact::Mem {
                        ty,
                        min_offset: u64::from(offset),
                        // Safety: can't overflow -- two u32s summed in a
                        // 64-bit add. TODO: when memory64 is supported here,
                        // `u32::MAX` is no longer true, and we'll need to
                        // handle overflow here.
                        max_offset: u64::from(u32::MAX) + u64::from(offset),
                        nullable: false,
                    });
                }
            }
        }
        result
    }
}

#[inline]
fn offset_plus_size(offset: u32, size: u8) -> u64 {
    // Cannot overflow because we are widening to `u64`.
    offset as u64 + size as u64
}

/// Returns whether `index` is statically in-bounds with respect to this
/// `heap`'s configuration.
///
/// This is `true` when `index` is a constant and when the offset/size are added
/// in it's all still less than the minimum byte size of the heap.
///
/// The `offset_and_size` here are the static offset that was listed on the wasm
/// instruction plus the size of the access being made.
fn statically_in_bounds(
    func: &ir::Function,
    heap: &HeapData,
    index: ir::Value,
    offset_and_size: u64,
) -> bool {
    func.dfg
        .value_def(index)
        .inst()
        .and_then(|i| {
            let imm = match func.dfg.insts[i] {
                ir::InstructionData::UnaryImm {
                    opcode: ir::Opcode::Iconst,
                    imm,
                } => imm,
                _ => return None,
            };
            let ty = func.dfg.value_type(index);
            let index = imm.zero_extend_from_width(ty.bits()).bits().unsigned();
            let final_addr = index.checked_add(offset_and_size)?;
            Some(final_addr <= heap.memory.minimum_byte_size().unwrap_or(u64::MAX))
        })
        .unwrap_or(false)
}
