#![expect(non_snake_case, reason = "DSL style here")]

use crate::cdsl::instructions::{
    AllInstructions, InstructionBuilder as Inst, InstructionGroupBuilder,
};
use crate::cdsl::operands::Operand;
use crate::cdsl::types::{LaneType, ValueType};
use crate::cdsl::typevar::{Interval, TypeSetBuilder, TypeVar};
use crate::shared::formats::Formats;
use crate::shared::types;
use crate::shared::{entities::EntityRefs, immediates::Immediates};

#[inline(never)]
fn define_control_flow(
    ig: &mut InstructionGroupBuilder,
    formats: &Formats,
    imm: &Immediates,
    entities: &EntityRefs,
) {
    ig.push(
        Inst::new(
            "jump",
            r#"
        Jump.

        Unconditionally jump to a basic block, passing the specified
        block arguments. The number and types of arguments must match the
        destination block.
        "#,
            &formats.jump,
        )
        .operands_in(vec![
            Operand::new("block_call", &entities.block_call)
                .with_doc("Destination basic block, with its arguments provided"),
        ])
        .branches(),
    );

    let ScalarTruthy = &TypeVar::new(
        "ScalarTruthy",
        "A scalar truthy type",
        TypeSetBuilder::new().ints(Interval::All).build(),
    );

    ig.push(
        Inst::new(
            "brif",
            r#"
        Conditional branch when cond is non-zero.

        Take the ``then`` branch when ``c != 0``, and the ``else`` branch otherwise.
        "#,
            &formats.brif,
        )
        .operands_in(vec![
            Operand::new("c", ScalarTruthy).with_doc("Controlling value to test"),
            Operand::new("block_then", &entities.block_then).with_doc("Then block"),
            Operand::new("block_else", &entities.block_else).with_doc("Else block"),
        ])
        .branches(),
    );

    {
        let _i32 = &TypeVar::new(
            "i32",
            "A 32 bit scalar integer type",
            TypeSetBuilder::new().ints(32..32).build(),
        );

        ig.push(
            Inst::new(
                "br_table",
                r#"
        Indirect branch via jump table.

        Use ``x`` as an unsigned index into the jump table ``JT``. If a jump
        table entry is found, branch to the corresponding block. If no entry was
        found or the index is out-of-bounds, branch to the default block of the
        table.

        Note that this branch instruction can't pass arguments to the targeted
        blocks. Split critical edges as needed to work around this.

        Do not confuse this with "tables" in WebAssembly. ``br_table`` is for
        jump tables with destinations within the current function only -- think
        of a ``match`` in Rust or a ``switch`` in C.  If you want to call a
        function in a dynamic library, that will typically use
        ``call_indirect``.
        "#,
                &formats.branch_table,
            )
            .operands_in(vec![
                Operand::new("x", _i32).with_doc("i32 index into jump table"),
                Operand::new("JT", &entities.jump_table),
            ])
            .branches(),
        );
    }

    let iAddr = &TypeVar::new(
        "iAddr",
        "An integer address type",
        TypeSetBuilder::new().ints(32..64).build(),
    );

    ig.push(
        Inst::new(
            "debugtrap",
            r#"
        Encodes an assembly debug trap.
        "#,
            &formats.nullary,
        )
        .other_side_effects()
        .can_load()
        .can_store(),
    );

    ig.push(
        Inst::new(
            "trap",
            r#"
        Terminate execution unconditionally.
        "#,
            &formats.trap,
        )
        .operands_in(vec![Operand::new("code", &imm.trapcode)])
        .can_trap()
        .terminates_block(),
    );

    ig.push(
        Inst::new(
            "trapz",
            r#"
        Trap when zero.

        if ``c`` is non-zero, execution continues at the following instruction.
        "#,
            &formats.cond_trap,
        )
        .operands_in(vec![
            Operand::new("c", ScalarTruthy).with_doc("Controlling value to test"),
            Operand::new("code", &imm.trapcode),
        ])
        .can_trap()
        // When one `trapz` dominates another `trapz` and they have identical
        // conditions and trap codes, it is safe to deduplicate them (like GVN,
        // although there is not actually any value being numbered). Either the
        // first `trapz` raised a trap and execution halted, or it didn't and
        // therefore the dominated `trapz` will not raise a trap either.
        .side_effects_idempotent(),
    );

    ig.push(
        Inst::new(
            "trapnz",
            r#"
        Trap when non-zero.

        If ``c`` is zero, execution continues at the following instruction.
        "#,
            &formats.cond_trap,
        )
        .operands_in(vec![
            Operand::new("c", ScalarTruthy).with_doc("Controlling value to test"),
            Operand::new("code", &imm.trapcode),
        ])
        .can_trap()
        // See the above comment for `trapz` and idempotent side effects.
        .side_effects_idempotent(),
    );

    ig.push(
        Inst::new(
            "return",
            r#"
        Return from the function.

        Unconditionally transfer control to the calling function, passing the
        provided return values. The list of return values must match the
        function signature's return types.
        "#,
            &formats.multiary,
        )
        .operands_in(vec![
            Operand::new("rvals", &entities.varargs).with_doc("return values"),
        ])
        .returns(),
    );

    ig.push(
        Inst::new(
            "call",
            r#"
        Direct function call.

        Call a function which has been declared in the preamble. The argument
        types must match the function's signature.
        "#,
            &formats.call,
        )
        .operands_in(vec![
            Operand::new("FN", &entities.func_ref)
                .with_doc("function to call, declared by `function`"),
            Operand::new("args", &entities.varargs).with_doc("call arguments"),
        ])
        .operands_out(vec![
            Operand::new("rvals", &entities.varargs).with_doc("return values"),
        ])
        .call(),
    );

    ig.push(
        Inst::new(
            "call_indirect",
            r#"
        Indirect function call.

        Call the function pointed to by `callee` with the given arguments. The
        called function must match the specified signature.

        Note that this is different from WebAssembly's ``call_indirect``; the
        callee is a native address, rather than a table index. For WebAssembly,
        `table_addr` and `load` are used to obtain a native address
        from a table.
        "#,
            &formats.call_indirect,
        )
        .operands_in(vec![
            Operand::new("SIG", &entities.sig_ref).with_doc("function signature"),
            Operand::new("callee", iAddr).with_doc("address of function to call"),
            Operand::new("args", &entities.varargs).with_doc("call arguments"),
        ])
        .operands_out(vec![
            Operand::new("rvals", &entities.varargs).with_doc("return values"),
        ])
        .call(),
    );

    ig.push(
        Inst::new(
            "return_call",
            r#"
        Direct tail call.

        Tail call a function which has been declared in the preamble. The
        argument types must match the function's signature, the caller and
        callee calling conventions must be the same, and must be a calling
        convention that supports tail calls.

        This instruction is a block terminator.
        "#,
            &formats.call,
        )
        .operands_in(vec![
            Operand::new("FN", &entities.func_ref)
                .with_doc("function to call, declared by `function`"),
            Operand::new("args", &entities.varargs).with_doc("call arguments"),
        ])
        .returns()
        .call(),
    );

    ig.push(
        Inst::new(
            "return_call_indirect",
            r#"
        Indirect tail call.

        Call the function pointed to by `callee` with the given arguments. The
        argument types must match the function's signature, the caller and
        callee calling conventions must be the same, and must be a calling
        convention that supports tail calls.

        This instruction is a block terminator.

        Note that this is different from WebAssembly's ``tail_call_indirect``;
        the callee is a native address, rather than a table index. For
        WebAssembly, `table_addr` and `load` are used to obtain a native address
        from a table.
        "#,
            &formats.call_indirect,
        )
        .operands_in(vec![
            Operand::new("SIG", &entities.sig_ref).with_doc("function signature"),
            Operand::new("callee", iAddr).with_doc("address of function to call"),
            Operand::new("args", &entities.varargs).with_doc("call arguments"),
        ])
        .returns()
        .call(),
    );

    ig.push(
        Inst::new(
            "func_addr",
            r#"
        Get the address of a function.

        Compute the absolute address of a function declared in the preamble.
        The returned address can be used as a ``callee`` argument to
        `call_indirect`. This is also a method for calling functions that
        are too far away to be addressable by a direct `call`
        instruction.
        "#,
            &formats.func_addr,
        )
        .operands_in(vec![
            Operand::new("FN", &entities.func_ref)
                .with_doc("function to call, declared by `function`"),
        ])
        .operands_out(vec![Operand::new("addr", iAddr)]),
    );

    ig.push(
        Inst::new(
            "try_call",
            r#"
        Call a function, catching the specified exceptions.

        Call the function pointed to by `callee` with the given arguments. On
        normal return, branch to the first target, with function returns
        available as `retN` block arguments. On exceptional return,
        look up the thrown exception tag in the provided exception table;
        if the tag matches one of the targets, branch to the matching
        target with the exception payloads available as `exnN` block arguments.
        If no tag matches, then propagate the exception up the stack.

        It is the Cranelift embedder's responsibility to define the meaning
        of tags: they are accepted by this instruction and passed through
        to unwind metadata tables in Cranelift's output. Actual unwinding is
        outside the purview of the core Cranelift compiler.

        Payload values on exception are passed in fixed register(s) that are
        defined by the platform and ABI. See the documentation on `CallConv`
        for details.
        "#,
            &formats.try_call,
        )
        .operands_in(vec![
            Operand::new("callee", &entities.func_ref)
                .with_doc("function to call, declared by `function`"),
            Operand::new("args", &entities.varargs).with_doc("call arguments"),
            Operand::new("ET", &entities.exception_table).with_doc("exception table"),
        ])
        .call()
        .branches(),
    );

    ig.push(
        Inst::new(
            "try_call_indirect",
            r#"
        Call a function, catching the specified exceptions.

        Call the function pointed to by `callee` with the given arguments. On
        normal return, branch to the first target, with function returns
        available as `retN` block arguments. On exceptional return,
        look up the thrown exception tag in the provided exception table;
        if the tag matches one of the targets, branch to the matching
        target with the exception payloads available as `exnN` block arguments.
        If no tag matches, then propagate the exception up the stack.

        It is the Cranelift embedder's responsibility to define the meaning
        of tags: they are accepted by this instruction and passed through
        to unwind metadata tables in Cranelift's output. Actual unwinding is
        outside the purview of the core Cranelift compiler.

        Payload values on exception are passed in fixed register(s) that are
        defined by the platform and ABI. See the documentation on `CallConv`
        for details.
        "#,
            &formats.try_call_indirect,
        )
        .operands_in(vec![
            Operand::new("callee", iAddr).with_doc("address of function to call"),
            Operand::new("args", &entities.varargs).with_doc("call arguments"),
            Operand::new("ET", &entities.exception_table).with_doc("exception table"),
        ])
        .call()
        .branches(),
    );
}

#[inline(never)]
fn define_simd_lane_access(
    ig: &mut InstructionGroupBuilder,
    formats: &Formats,
    imm: &Immediates,
    _: &EntityRefs,
) {
    let TxN = &TypeVar::new(
        "TxN",
        "A SIMD vector type",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .floats(Interval::All)
            .simd_lanes(Interval::All)
            .dynamic_simd_lanes(Interval::All)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "splat",
            r#"
        Vector splat.

        Return a vector whose lanes are all ``x``.
        "#,
            &formats.unary,
        )
        .operands_in(vec![
            Operand::new("x", &TxN.lane_of()).with_doc("Value to splat to all lanes"),
        ])
        .operands_out(vec![Operand::new("a", TxN)]),
    );

    let I8x16 = &TypeVar::new(
        "I8x16",
        "A SIMD vector type consisting of 16 lanes of 8-bit integers",
        TypeSetBuilder::new()
            .ints(8..8)
            .simd_lanes(16..16)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "swizzle",
            r#"
        Vector swizzle.

        Returns a new vector with byte-width lanes selected from the lanes of the first input
        vector ``x`` specified in the second input vector ``s``. The indices ``i`` in range
        ``[0, 15]`` select the ``i``-th element of ``x``. For indices outside of the range the
        resulting lane is 0. Note that this operates on byte-width lanes.
        "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", I8x16).with_doc("Vector to modify by re-arranging lanes"),
            Operand::new("y", I8x16).with_doc("Mask for re-arranging lanes"),
        ])
        .operands_out(vec![Operand::new("a", I8x16)]),
    );

    ig.push(
        Inst::new(
            "x86_pshufb",
            r#"
        A vector swizzle lookalike which has the semantics of `pshufb` on x64.

        This instruction will permute the 8-bit lanes of `x` with the indices
        specified in `y`. Each lane in the mask, `y`, uses the bottom four
        bits for selecting the lane from `x` unless the most significant bit
        is set, in which case the lane is zeroed. The output vector will have
        the following contents when the element of `y` is in these ranges:

        * `[0, 127]` -> `x[y[i] % 16]`
        * `[128, 255]` -> 0
        "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", I8x16).with_doc("Vector to modify by re-arranging lanes"),
            Operand::new("y", I8x16).with_doc("Mask for re-arranging lanes"),
        ])
        .operands_out(vec![Operand::new("a", I8x16)]),
    );

    ig.push(
        Inst::new(
            "insertlane",
            r#"
        Insert ``y`` as lane ``Idx`` in x.

        The lane index, ``Idx``, is an immediate value, not an SSA value. It
        must indicate a valid lane index for the type of ``x``.
        "#,
            &formats.ternary_imm8,
        )
        .operands_in(vec![
            Operand::new("x", TxN).with_doc("The vector to modify"),
            Operand::new("y", &TxN.lane_of()).with_doc("New lane value"),
            Operand::new("Idx", &imm.uimm8).with_doc("Lane index"),
        ])
        .operands_out(vec![Operand::new("a", TxN)]),
    );

    ig.push(
        Inst::new(
            "extractlane",
            r#"
        Extract lane ``Idx`` from ``x``.

        The lane index, ``Idx``, is an immediate value, not an SSA value. It
        must indicate a valid lane index for the type of ``x``. Note that the upper bits of ``a``
        may or may not be zeroed depending on the ISA but the type system should prevent using
        ``a`` as anything other than the extracted value.
        "#,
            &formats.binary_imm8,
        )
        .operands_in(vec![
            Operand::new("x", TxN),
            Operand::new("Idx", &imm.uimm8).with_doc("Lane index"),
        ])
        .operands_out(vec![Operand::new("a", &TxN.lane_of())]),
    );
}

#[inline(never)]
fn define_simd_arithmetic(
    ig: &mut InstructionGroupBuilder,
    formats: &Formats,
    _: &Immediates,
    _: &EntityRefs,
) {
    let Int = &TypeVar::new(
        "Int",
        "A scalar or vector integer type",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .simd_lanes(Interval::All)
            .build(),
    );

    ig.push(
        Inst::new(
            "smin",
            r#"
        Signed integer minimum.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Int), Operand::new("y", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "umin",
            r#"
        Unsigned integer minimum.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Int), Operand::new("y", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "smax",
            r#"
        Signed integer maximum.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Int), Operand::new("y", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "umax",
            r#"
        Unsigned integer maximum.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Int), Operand::new("y", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    let IxN = &TypeVar::new(
        "IxN",
        "A SIMD vector type containing integers",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .simd_lanes(Interval::All)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "avg_round",
            r#"
        Unsigned average with rounding: `a := (x + y + 1) // 2`

        The addition does not lose any information (such as from overflow).
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", IxN), Operand::new("y", IxN)])
        .operands_out(vec![Operand::new("a", IxN)]),
    );

    ig.push(
        Inst::new(
            "uadd_sat",
            r#"
        Add with unsigned saturation.

        This is similar to `iadd` but the operands are interpreted as unsigned integers and their
        summed result, instead of wrapping, will be saturated to the highest unsigned integer for
        the controlling type (e.g. `0xFF` for i8).
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", IxN), Operand::new("y", IxN)])
        .operands_out(vec![Operand::new("a", IxN)]),
    );

    ig.push(
        Inst::new(
            "sadd_sat",
            r#"
        Add with signed saturation.

        This is similar to `iadd` but the operands are interpreted as signed integers and their
        summed result, instead of wrapping, will be saturated to the lowest or highest
        signed integer for the controlling type (e.g. `0x80` or `0x7F` for i8). For example,
        since an `sadd_sat.i8` of `0x70` and `0x70` is greater than `0x7F`, the result will be
        clamped to `0x7F`.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", IxN), Operand::new("y", IxN)])
        .operands_out(vec![Operand::new("a", IxN)]),
    );

    ig.push(
        Inst::new(
            "usub_sat",
            r#"
        Subtract with unsigned saturation.

        This is similar to `isub` but the operands are interpreted as unsigned integers and their
        difference, instead of wrapping, will be saturated to the lowest unsigned integer for
        the controlling type (e.g. `0x00` for i8).
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", IxN), Operand::new("y", IxN)])
        .operands_out(vec![Operand::new("a", IxN)]),
    );

    ig.push(
        Inst::new(
            "ssub_sat",
            r#"
        Subtract with signed saturation.

        This is similar to `isub` but the operands are interpreted as signed integers and their
        difference, instead of wrapping, will be saturated to the lowest or highest
        signed integer for the controlling type (e.g. `0x80` or `0x7F` for i8).
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", IxN), Operand::new("y", IxN)])
        .operands_out(vec![Operand::new("a", IxN)]),
    );
}

pub(crate) fn define(
    all_instructions: &mut AllInstructions,
    formats: &Formats,
    imm: &Immediates,
    entities: &EntityRefs,
) {
    let mut ig = InstructionGroupBuilder::new(all_instructions);

    define_control_flow(&mut ig, formats, imm, entities);
    define_simd_lane_access(&mut ig, formats, imm, entities);
    define_simd_arithmetic(&mut ig, formats, imm, entities);

    // Operand kind shorthands.
    let i8: &TypeVar = &ValueType::from(LaneType::from(types::Int::I8)).into();
    let f16_: &TypeVar = &ValueType::from(LaneType::from(types::Float::F16)).into();
    let f32_: &TypeVar = &ValueType::from(LaneType::from(types::Float::F32)).into();
    let f64_: &TypeVar = &ValueType::from(LaneType::from(types::Float::F64)).into();
    let f128_: &TypeVar = &ValueType::from(LaneType::from(types::Float::F128)).into();

    // Starting definitions.
    let Int = &TypeVar::new(
        "Int",
        "A scalar or vector integer type",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .simd_lanes(Interval::All)
            .dynamic_simd_lanes(Interval::All)
            .build(),
    );

    let NarrowInt = &TypeVar::new(
        "NarrowInt",
        "An integer type of width up to `i64`",
        TypeSetBuilder::new().ints(8..64).build(),
    );

    let ScalarTruthy = &TypeVar::new(
        "ScalarTruthy",
        "A scalar truthy type",
        TypeSetBuilder::new().ints(Interval::All).build(),
    );

    let iB = &TypeVar::new(
        "iB",
        "A scalar integer type",
        TypeSetBuilder::new().ints(Interval::All).build(),
    );

    let iSwappable = &TypeVar::new(
        "iSwappable",
        "A multi byte scalar integer type",
        TypeSetBuilder::new().ints(16..128).build(),
    );

    let iAddr = &TypeVar::new(
        "iAddr",
        "An integer address type",
        TypeSetBuilder::new().ints(32..64).build(),
    );

    let TxN = &TypeVar::new(
        "TxN",
        "A SIMD vector type",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .floats(Interval::All)
            .simd_lanes(Interval::All)
            .includes_scalars(false)
            .build(),
    );
    let Any = &TypeVar::new(
        "Any",
        "Any integer, float, or reference scalar or vector type",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .floats(Interval::All)
            .simd_lanes(Interval::All)
            .includes_scalars(true)
            .build(),
    );

    let Mem = &TypeVar::new(
        "Mem",
        "Any type that can be stored in memory",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .floats(Interval::All)
            .simd_lanes(Interval::All)
            .dynamic_simd_lanes(Interval::All)
            .build(),
    );

    let MemTo = &TypeVar::copy_from(Mem, "MemTo".to_string());

    ig.push(
        Inst::new(
            "load",
            r#"
        Load from memory at ``p + Offset``.

        This is a polymorphic instruction that can load any value type which
        has a memory representation.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", Mem).with_doc("Value loaded")])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "store",
            r#"
        Store ``x`` to memory at ``p + Offset``.

        This is a polymorphic instruction that can store any value type with a
        memory representation.
        "#,
            &formats.store,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("x", Mem).with_doc("Value to be stored"),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .can_store(),
    );

    let iExt8 = &TypeVar::new(
        "iExt8",
        "An integer type with more than 8 bits",
        TypeSetBuilder::new().ints(16..64).build(),
    );

    ig.push(
        Inst::new(
            "uload8",
            r#"
        Load 8 bits from memory at ``p + Offset`` and zero-extend.

        This is equivalent to ``load.i8`` followed by ``uextend``.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", iExt8)])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "sload8",
            r#"
        Load 8 bits from memory at ``p + Offset`` and sign-extend.

        This is equivalent to ``load.i8`` followed by ``sextend``.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", iExt8)])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "istore8",
            r#"
        Store the low 8 bits of ``x`` to memory at ``p + Offset``.

        This is equivalent to ``ireduce.i8`` followed by ``store.i8``.
        "#,
            &formats.store,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("x", iExt8),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .can_store(),
    );

    let iExt16 = &TypeVar::new(
        "iExt16",
        "An integer type with more than 16 bits",
        TypeSetBuilder::new().ints(32..64).build(),
    );

    ig.push(
        Inst::new(
            "uload16",
            r#"
        Load 16 bits from memory at ``p + Offset`` and zero-extend.

        This is equivalent to ``load.i16`` followed by ``uextend``.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", iExt16)])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "sload16",
            r#"
        Load 16 bits from memory at ``p + Offset`` and sign-extend.

        This is equivalent to ``load.i16`` followed by ``sextend``.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", iExt16)])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "istore16",
            r#"
        Store the low 16 bits of ``x`` to memory at ``p + Offset``.

        This is equivalent to ``ireduce.i16`` followed by ``store.i16``.
        "#,
            &formats.store,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("x", iExt16),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .can_store(),
    );

    let iExt32 = &TypeVar::new(
        "iExt32",
        "An integer type with more than 32 bits",
        TypeSetBuilder::new().ints(64..64).build(),
    );

    ig.push(
        Inst::new(
            "uload32",
            r#"
        Load 32 bits from memory at ``p + Offset`` and zero-extend.

        This is equivalent to ``load.i32`` followed by ``uextend``.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", iExt32)])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "sload32",
            r#"
        Load 32 bits from memory at ``p + Offset`` and sign-extend.

        This is equivalent to ``load.i32`` followed by ``sextend``.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", iExt32)])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "istore32",
            r#"
        Store the low 32 bits of ``x`` to memory at ``p + Offset``.

        This is equivalent to ``ireduce.i32`` followed by ``store.i32``.
        "#,
            &formats.store,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("x", iExt32),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .can_store(),
    );
    ig.push(
        Inst::new(
            "stack_switch",
            r#"
        Suspends execution of the current stack and resumes execution of another
        one.

        The target stack to switch to is identified by the data stored at
        ``load_context_ptr``. Before switching, this instruction stores
        analogous information about the
        current (i.e., original) stack at ``store_context_ptr``, to
        enabled switching back to the original stack at a later point.

        The size, alignment and layout of the information stored at
        ``load_context_ptr`` and ``store_context_ptr`` is platform-dependent.
        The instruction assumes that ``load_context_ptr`` and
        ``store_context_ptr`` are valid pointers to memory with said layout and
        alignment, and does not perform any checks on these pointers or the data
        stored there.

        The instruction is experimental and only supported on x64 Linux at the
        moment.

        When switching from a stack A to a stack B, one of the following cases
        must apply:
        1. Stack B was previously suspended using a ``stack_switch`` instruction.
        2. Stack B is a newly initialized stack. The necessary initialization is
        platform-dependent and will generally involve running some kind of
        trampoline to start execution of a function on the new stack.

        In both cases, the ``in_payload`` argument of the ``stack_switch``
        instruction executed on A is passed to stack B. In the first case above,
        it will be the result value of the earlier ``stack_switch`` instruction
        executed on stack B. In the second case, the value will be accessible to
        the trampoline in a platform-dependent register.

        The pointers ``load_context_ptr`` and ``store_context_ptr`` are allowed
        to be equal; the instruction ensures that all data is loaded from the
        former before writing to the latter.

        Stack switching is one-shot in the sense that each ``stack_switch``
        operation effectively consumes the context identified by
        ``load_context_ptr``. In other words, performing two ``stack_switches``
        using the same ``load_context_ptr`` causes undefined behavior, unless
        the context at ``load_context_ptr`` is overwritten by another
        `stack_switch` in between.
        "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("store_context_ptr", iAddr),
            Operand::new("load_context_ptr", iAddr),
            Operand::new("in_payload0", iAddr),
        ])
        .operands_out(vec![Operand::new("out_payload0", iAddr)])
        .other_side_effects()
        .can_load()
        .can_store()
        .call(),
    );

    let I16x8 = &TypeVar::new(
        "I16x8",
        "A SIMD vector with exactly 8 lanes of 16-bit values",
        TypeSetBuilder::new()
            .ints(16..16)
            .simd_lanes(8..8)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "uload8x8",
            r#"
        Load an 8x8 vector (64 bits) from memory at ``p + Offset`` and zero-extend into an i16x8
        vector.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", I16x8).with_doc("Value loaded")])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "sload8x8",
            r#"
        Load an 8x8 vector (64 bits) from memory at ``p + Offset`` and sign-extend into an i16x8
        vector.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", I16x8).with_doc("Value loaded")])
        .can_load(),
    );

    let I32x4 = &TypeVar::new(
        "I32x4",
        "A SIMD vector with exactly 4 lanes of 32-bit values",
        TypeSetBuilder::new()
            .ints(32..32)
            .simd_lanes(4..4)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "uload16x4",
            r#"
        Load a 16x4 vector (64 bits) from memory at ``p + Offset`` and zero-extend into an i32x4
        vector.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", I32x4).with_doc("Value loaded")])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "sload16x4",
            r#"
        Load a 16x4 vector (64 bits) from memory at ``p + Offset`` and sign-extend into an i32x4
        vector.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", I32x4).with_doc("Value loaded")])
        .can_load(),
    );

    let I64x2 = &TypeVar::new(
        "I64x2",
        "A SIMD vector with exactly 2 lanes of 64-bit values",
        TypeSetBuilder::new()
            .ints(64..64)
            .simd_lanes(2..2)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "uload32x2",
            r#"
        Load an 32x2 vector (64 bits) from memory at ``p + Offset`` and zero-extend into an i64x2
        vector.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", I64x2).with_doc("Value loaded")])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "sload32x2",
            r#"
        Load a 32x2 vector (64 bits) from memory at ``p + Offset`` and sign-extend into an i64x2
        vector.
        "#,
            &formats.load,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("Offset", &imm.offset32).with_doc("Byte offset from base address"),
        ])
        .operands_out(vec![Operand::new("a", I64x2).with_doc("Value loaded")])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "stack_load",
            r#"
        Load a value from a stack slot at the constant offset.

        This is a polymorphic instruction that can load any value type which
        has a memory representation.

        The offset is an immediate constant, not an SSA value. The memory
        access cannot go out of bounds, i.e.
        `sizeof(a) + Offset <= sizeof(SS)`.
        "#,
            &formats.stack_load,
        )
        .operands_in(vec![
            Operand::new("SS", &entities.stack_slot),
            Operand::new("Offset", &imm.offset32).with_doc("In-bounds offset into stack slot"),
        ])
        .operands_out(vec![Operand::new("a", Mem).with_doc("Value loaded")])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "stack_store",
            r#"
        Store a value to a stack slot at a constant offset.

        This is a polymorphic instruction that can store any value type with a
        memory representation.

        The offset is an immediate constant, not an SSA value. The memory
        access cannot go out of bounds, i.e.
        `sizeof(a) + Offset <= sizeof(SS)`.
        "#,
            &formats.stack_store,
        )
        .operands_in(vec![
            Operand::new("x", Mem).with_doc("Value to be stored"),
            Operand::new("SS", &entities.stack_slot),
            Operand::new("Offset", &imm.offset32).with_doc("In-bounds offset into stack slot"),
        ])
        .can_store(),
    );

    ig.push(
        Inst::new(
            "stack_addr",
            r#"
        Get the address of a stack slot.

        Compute the absolute address of a byte in a stack slot. The offset must
        refer to a byte inside the stack slot:
        `0 <= Offset < sizeof(SS)`.
        "#,
            &formats.stack_load,
        )
        .operands_in(vec![
            Operand::new("SS", &entities.stack_slot),
            Operand::new("Offset", &imm.offset32).with_doc("In-bounds offset into stack slot"),
        ])
        .operands_out(vec![Operand::new("addr", iAddr)]),
    );

    ig.push(
        Inst::new(
            "dynamic_stack_load",
            r#"
        Load a value from a dynamic stack slot.

        This is a polymorphic instruction that can load any value type which
        has a memory representation.
        "#,
            &formats.dynamic_stack_load,
        )
        .operands_in(vec![Operand::new("DSS", &entities.dynamic_stack_slot)])
        .operands_out(vec![Operand::new("a", Mem).with_doc("Value loaded")])
        .can_load(),
    );

    ig.push(
        Inst::new(
            "dynamic_stack_store",
            r#"
        Store a value to a dynamic stack slot.

        This is a polymorphic instruction that can store any dynamic value type with a
        memory representation.
        "#,
            &formats.dynamic_stack_store,
        )
        .operands_in(vec![
            Operand::new("x", Mem).with_doc("Value to be stored"),
            Operand::new("DSS", &entities.dynamic_stack_slot),
        ])
        .can_store(),
    );

    ig.push(
        Inst::new(
            "dynamic_stack_addr",
            r#"
        Get the address of a dynamic stack slot.

        Compute the absolute address of the first byte of a dynamic stack slot.
        "#,
            &formats.dynamic_stack_load,
        )
        .operands_in(vec![Operand::new("DSS", &entities.dynamic_stack_slot)])
        .operands_out(vec![Operand::new("addr", iAddr)]),
    );

    ig.push(
        Inst::new(
            "global_value",
            r#"
        Compute the value of global GV.
        "#,
            &formats.unary_global_value,
        )
        .operands_in(vec![Operand::new("GV", &entities.global_value)])
        .operands_out(vec![Operand::new("a", Mem).with_doc("Value loaded")]),
    );

    ig.push(
        Inst::new(
            "symbol_value",
            r#"
        Compute the value of global GV, which is a symbolic value.
        "#,
            &formats.unary_global_value,
        )
        .operands_in(vec![Operand::new("GV", &entities.global_value)])
        .operands_out(vec![Operand::new("a", Mem).with_doc("Value loaded")]),
    );

    ig.push(
        Inst::new(
            "tls_value",
            r#"
        Compute the value of global GV, which is a TLS (thread local storage) value.
        "#,
            &formats.unary_global_value,
        )
        .operands_in(vec![Operand::new("GV", &entities.global_value)])
        .operands_out(vec![Operand::new("a", Mem).with_doc("Value loaded")]),
    );

    // Note this instruction is marked as having other side-effects, so GVN won't try to hoist it,
    // which would result in it being subject to spilling. While not hoisting would generally hurt
    // performance, since a computed value used many times may need to be regenerated before each
    // use, it is not the case here: this instruction doesn't generate any code.  That's because,
    // by definition the pinned register is never used by the register allocator, but is written to
    // and read explicitly and exclusively by set_pinned_reg and get_pinned_reg.
    ig.push(
        Inst::new(
            "get_pinned_reg",
            r#"
            Gets the content of the pinned register, when it's enabled.
        "#,
            &formats.nullary,
        )
        .operands_out(vec![Operand::new("addr", iAddr)])
        .other_side_effects(),
    );

    ig.push(
        Inst::new(
            "set_pinned_reg",
            r#"
        Sets the content of the pinned register, when it's enabled.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("addr", iAddr)])
        .other_side_effects(),
    );

    ig.push(
        Inst::new(
            "get_frame_pointer",
            r#"
        Get the address in the frame pointer register.

        Usage of this instruction requires setting `preserve_frame_pointers` to `true`.
        "#,
            &formats.nullary,
        )
        .operands_out(vec![Operand::new("addr", iAddr)]),
    );

    ig.push(
        Inst::new(
            "get_stack_pointer",
            r#"
        Get the address in the stack pointer register.
        "#,
            &formats.nullary,
        )
        .operands_out(vec![Operand::new("addr", iAddr)]),
    );

    ig.push(
        Inst::new(
            "get_return_address",
            r#"
        Get the PC where this function will transfer control to when it returns.

        Usage of this instruction requires setting `preserve_frame_pointers` to `true`.
        "#,
            &formats.nullary,
        )
        .operands_out(vec![Operand::new("addr", iAddr)]),
    );

    ig.push(
        Inst::new(
            "iconst",
            r#"
        Integer constant.

        Create a scalar integer SSA value with an immediate constant value, or
        an integer vector where all the lanes have the same value.
        "#,
            &formats.unary_imm,
        )
        .operands_in(vec![Operand::new("N", &imm.imm64)])
        .operands_out(vec![
            Operand::new("a", NarrowInt).with_doc("A constant integer scalar or vector value"),
        ]),
    );

    ig.push(
        Inst::new(
            "f16const",
            r#"
        Floating point constant.

        Create a `f16` SSA value with an immediate constant value.
        "#,
            &formats.unary_ieee16,
        )
        .operands_in(vec![Operand::new("N", &imm.ieee16)])
        .operands_out(vec![
            Operand::new("a", f16_).with_doc("A constant f16 scalar value"),
        ]),
    );

    ig.push(
        Inst::new(
            "f32const",
            r#"
        Floating point constant.

        Create a `f32` SSA value with an immediate constant value.
        "#,
            &formats.unary_ieee32,
        )
        .operands_in(vec![Operand::new("N", &imm.ieee32)])
        .operands_out(vec![
            Operand::new("a", f32_).with_doc("A constant f32 scalar value"),
        ]),
    );

    ig.push(
        Inst::new(
            "f64const",
            r#"
        Floating point constant.

        Create a `f64` SSA value with an immediate constant value.
        "#,
            &formats.unary_ieee64,
        )
        .operands_in(vec![Operand::new("N", &imm.ieee64)])
        .operands_out(vec![
            Operand::new("a", f64_).with_doc("A constant f64 scalar value"),
        ]),
    );

    ig.push(
        Inst::new(
            "f128const",
            r#"
        Floating point constant.

        Create a `f128` SSA value with an immediate constant value.
        "#,
            &formats.unary_const,
        )
        .operands_in(vec![Operand::new("N", &entities.pool_constant)])
        .operands_out(vec![
            Operand::new("a", f128_).with_doc("A constant f128 scalar value"),
        ]),
    );

    ig.push(
        Inst::new(
            "vconst",
            r#"
        SIMD vector constant.

        Construct a vector with the given immediate bytes.
        "#,
            &formats.unary_const,
        )
        .operands_in(vec![
            Operand::new("N", &entities.pool_constant)
                .with_doc("The 16 immediate bytes of a 128-bit vector"),
        ])
        .operands_out(vec![
            Operand::new("a", TxN).with_doc("A constant vector value"),
        ]),
    );

    let Tx16 = &TypeVar::new(
        "Tx16",
        "A SIMD vector with exactly 16 lanes of 8-bit values; eventually this may support other \
         lane counts and widths",
        TypeSetBuilder::new()
            .ints(8..8)
            .simd_lanes(16..16)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "shuffle",
            r#"
        SIMD vector shuffle.

        Shuffle two vectors using the given immediate bytes. For each of the 16 bytes of the
        immediate, a value i of 0-15 selects the i-th element of the first vector and a value i of
        16-31 selects the (i-16)th element of the second vector. Immediate values outside of the
        0-31 range are not valid.
        "#,
            &formats.shuffle,
        )
        .operands_in(vec![
            Operand::new("a", Tx16).with_doc("A vector value"),
            Operand::new("b", Tx16).with_doc("A vector value"),
            Operand::new("mask", &entities.uimm128)
                .with_doc("The 16 immediate bytes used for selecting the elements to shuffle"),
        ])
        .operands_out(vec![Operand::new("a", Tx16).with_doc("A vector value")]),
    );

    ig.push(Inst::new(
        "nop",
        r#"
        Just a dummy instruction.

        Note: this doesn't compile to a machine code nop.
        "#,
        &formats.nullary,
    ));

    ig.push(
        Inst::new(
            "select",
            r#"
        Conditional select.

        This instruction selects whole values. Use `bitselect` to choose each
        bit according to a mask.
        "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("c", ScalarTruthy).with_doc("Controlling value to test"),
            Operand::new("x", Any).with_doc("Value to use when `c` is true"),
            Operand::new("y", Any).with_doc("Value to use when `c` is false"),
        ])
        .operands_out(vec![Operand::new("a", Any)]),
    );

    ig.push(
        Inst::new(
            "select_spectre_guard",
            r#"
            Conditional select intended for Spectre guards.

            This operation is semantically equivalent to a select instruction.
            However, this instruction prohibits all speculation on the
            controlling value when determining which input to use as the result.
            As such, it is suitable for use in Spectre guards.

            For example, on a target which may speculatively execute branches,
            the lowering of this instruction is guaranteed to not conditionally
            branch. Instead it will typically lower to a conditional move
            instruction. (No Spectre-vulnerable processors are known to perform
            value speculation on conditional move instructions.)

            Ensure that the instruction you're trying to protect from Spectre
            attacks has a data dependency on the result of this instruction.
            That prevents an out-of-order CPU from evaluating that instruction
            until the result of this one is known, which in turn will be blocked
            until the controlling value is known.

            Typical usage is to use a bounds-check as the controlling value,
            and select between either a null pointer if the bounds-check
            fails, or an in-bounds address otherwise, so that dereferencing
            the resulting address with a load or store instruction will trap if
            the bounds-check failed. When this instruction is used in this way,
            any microarchitectural side effects of the memory access will only
            occur after the bounds-check finishes, which ensures that no Spectre
            vulnerability will exist.

            Optimization opportunities for this instruction are limited compared
            to a normal select instruction, but it is allowed to be replaced
            by other values which are functionally equivalent as long as doing
            so does not introduce any new opportunities to speculate on the
            controlling value.
            "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("c", ScalarTruthy).with_doc("Controlling value to test"),
            Operand::new("x", Any).with_doc("Value to use when `c` is true"),
            Operand::new("y", Any).with_doc("Value to use when `c` is false"),
        ])
        .operands_out(vec![Operand::new("a", Any)]),
    );

    ig.push(
        Inst::new(
            "bitselect",
            r#"
        Conditional select of bits.

        For each bit in `c`, this instruction selects the corresponding bit from `x` if the bit
        in `x` is 1 and the corresponding bit from `y` if the bit in `c` is 0. See also:
        `select`.
        "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("c", Any).with_doc("Controlling value to test"),
            Operand::new("x", Any).with_doc("Value to use when `c` is true"),
            Operand::new("y", Any).with_doc("Value to use when `c` is false"),
        ])
        .operands_out(vec![Operand::new("a", Any)]),
    );

    ig.push(
        Inst::new(
            "x86_blendv",
            r#"
        A bitselect-lookalike instruction except with the semantics of
        `blendv`-related instructions on x86.

        This instruction will use the top bit of each lane in `c`, the condition
        mask. If the bit is 1 then the corresponding lane from `x` is chosen.
        Otherwise the corresponding lane from `y` is chosen.

            "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("c", Any).with_doc("Controlling value to test"),
            Operand::new("x", Any).with_doc("Value to use when `c` is true"),
            Operand::new("y", Any).with_doc("Value to use when `c` is false"),
        ])
        .operands_out(vec![Operand::new("a", Any)]),
    );

    ig.push(
        Inst::new(
            "vany_true",
            r#"
        Reduce a vector to a scalar boolean.

        Return a scalar boolean true if any lane in ``a`` is non-zero, false otherwise.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("a", TxN)])
        .operands_out(vec![Operand::new("s", i8)]),
    );

    ig.push(
        Inst::new(
            "vall_true",
            r#"
        Reduce a vector to a scalar boolean.

        Return a scalar boolean true if all lanes in ``i`` are non-zero, false otherwise.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("a", TxN)])
        .operands_out(vec![Operand::new("s", i8)]),
    );

    ig.push(
        Inst::new(
            "vhigh_bits",
            r#"
        Reduce a vector to a scalar integer.

        Return a scalar integer, consisting of the concatenation of the most significant bit
        of each lane of ``a``.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("a", TxN)])
        .operands_out(vec![Operand::new("x", NarrowInt)]),
    );

    ig.push(
        Inst::new(
            "icmp",
            r#"
        Integer comparison.

        The condition code determines if the operands are interpreted as signed
        or unsigned integers.

        | Signed | Unsigned | Condition             |
        |--------|----------|-----------------------|
        | eq     | eq       | Equal                 |
        | ne     | ne       | Not equal             |
        | slt    | ult      | Less than             |
        | sge    | uge      | Greater than or equal |
        | sgt    | ugt      | Greater than          |
        | sle    | ule      | Less than or equal    |

        When this instruction compares integer vectors, it returns a vector of
        lane-wise comparisons.

        When comparing scalars, the result is:
            - `1` if the condition holds.
            - `0` if the condition does not hold.

        When comparing vectors, the result is:
            - `-1` (i.e. all ones) in each lane where the condition holds.
            - `0` in each lane where the condition does not hold.
        "#,
            &formats.int_compare,
        )
        .operands_in(vec![
            Operand::new("Cond", &imm.intcc),
            Operand::new("x", Int),
            Operand::new("y", Int),
        ])
        .operands_out(vec![Operand::new("a", &Int.as_truthy())]),
    );

    ig.push(
        Inst::new(
            "icmp_imm",
            r#"
        Compare scalar integer to a constant.

        This is the same as the `icmp` instruction, except one operand is
        a sign extended 64 bit immediate constant.

        This instruction can only compare scalars. Use `icmp` for
        lane-wise vector comparisons.
        "#,
            &formats.int_compare_imm,
        )
        .operands_in(vec![
            Operand::new("Cond", &imm.intcc),
            Operand::new("x", iB),
            Operand::new("Y", &imm.imm64),
        ])
        .operands_out(vec![Operand::new("a", i8)]),
    );

    ig.push(
        Inst::new(
            "iadd",
            r#"
        Wrapping integer addition: `a := x + y \pmod{2^B}`.

        This instruction does not depend on the signed/unsigned interpretation
        of the operands.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Int), Operand::new("y", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "isub",
            r#"
        Wrapping integer subtraction: `a := x - y \pmod{2^B}`.

        This instruction does not depend on the signed/unsigned interpretation
        of the operands.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Int), Operand::new("y", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "ineg",
            r#"
        Integer negation: `a := -x \pmod{2^B}`.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "iabs",
            r#"
        Integer absolute value with wrapping: `a := |x|`.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "imul",
            r#"
        Wrapping integer multiplication: `a := x y \pmod{2^B}`.

        This instruction does not depend on the signed/unsigned interpretation
        of the operands.

        Polymorphic over all integer types (vector and scalar).
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Int), Operand::new("y", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "umulhi",
            r#"
        Unsigned integer multiplication, producing the high half of a
        double-length result.

        Polymorphic over all integer types (vector and scalar).
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Int), Operand::new("y", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "smulhi",
            r#"
        Signed integer multiplication, producing the high half of a
        double-length result.

        Polymorphic over all integer types (vector and scalar).
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Int), Operand::new("y", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    let I16or32 = &TypeVar::new(
        "I16or32",
        "A vector integer type with 16- or 32-bit numbers",
        TypeSetBuilder::new().ints(16..32).simd_lanes(4..8).build(),
    );

    ig.push(
        Inst::new(
            "sqmul_round_sat",
            r#"
        Fixed-point multiplication of numbers in the QN format, where N + 1
        is the number bitwidth:
        `a := signed_saturate((x * y + (1 << (Q - 1))) >> Q)`

        Polymorphic over all integer vector types with 16- or 32-bit numbers.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", I16or32), Operand::new("y", I16or32)])
        .operands_out(vec![Operand::new("a", I16or32)]),
    );

    ig.push(
        Inst::new(
            "x86_pmulhrsw",
            r#"
        A similar instruction to `sqmul_round_sat` except with the semantics
        of x86's `pmulhrsw` instruction.

        This is the same as `sqmul_round_sat` except when both input lanes are
        `i16::MIN`.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", I16or32), Operand::new("y", I16or32)])
        .operands_out(vec![Operand::new("a", I16or32)]),
    );

    // Integer division and remainder are scalar-only; most
    // hardware does not directly support vector integer division.

    ig.push(
        Inst::new(
            "udiv",
            r#"
        Unsigned integer division: `a := \lfloor {x \over y} \rfloor`.

        This operation traps if the divisor is zero.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("y", iB)])
        .operands_out(vec![Operand::new("a", iB)])
        .can_trap()
        .side_effects_idempotent(),
    );

    ig.push(
        Inst::new(
            "sdiv",
            r#"
        Signed integer division rounded toward zero: `a := sign(xy)
        \lfloor {|x| \over |y|}\rfloor`.

        This operation traps if the divisor is zero, or if the result is not
        representable in `B` bits two's complement. This only happens
        when `x = -2^{B-1}, y = -1`.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("y", iB)])
        .operands_out(vec![Operand::new("a", iB)])
        .can_trap()
        .side_effects_idempotent(),
    );

    ig.push(
        Inst::new(
            "urem",
            r#"
        Unsigned integer remainder.

        This operation traps if the divisor is zero.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("y", iB)])
        .operands_out(vec![Operand::new("a", iB)])
        .can_trap()
        .side_effects_idempotent(),
    );

    ig.push(
        Inst::new(
            "srem",
            r#"
        Signed integer remainder. The result has the sign of the dividend.

        This operation traps if the divisor is zero.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("y", iB)])
        .operands_out(vec![Operand::new("a", iB)])
        .can_trap()
        .side_effects_idempotent(),
    );

    ig.push(
        Inst::new(
            "iadd_imm",
            r#"
        Add immediate integer.

        Same as `iadd`, but one operand is a sign extended 64 bit immediate constant.

        Polymorphic over all scalar integer types, but does not support vector
        types.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "imul_imm",
            r#"
        Integer multiplication by immediate constant.

        Same as `imul`, but one operand is a sign extended 64 bit immediate constant.

        Polymorphic over all scalar integer types, but does not support vector
        types.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "udiv_imm",
            r#"
        Unsigned integer division by an immediate constant.

        Same as `udiv`, but one operand is a zero extended 64 bit immediate constant.

        This operation traps if the divisor is zero.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "sdiv_imm",
            r#"
        Signed integer division by an immediate constant.

        Same as `sdiv`, but one operand is a sign extended 64 bit immediate constant.

        This operation traps if the divisor is zero, or if the result is not
        representable in `B` bits two's complement. This only happens
        when `x = -2^{B-1}, Y = -1`.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "urem_imm",
            r#"
        Unsigned integer remainder with immediate divisor.

        Same as `urem`, but one operand is a zero extended 64 bit immediate constant.

        This operation traps if the divisor is zero.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "srem_imm",
            r#"
        Signed integer remainder with immediate divisor.

        Same as `srem`, but one operand is a sign extended 64 bit immediate constant.

        This operation traps if the divisor is zero.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "irsub_imm",
            r#"
        Immediate reverse wrapping subtraction: `a := Y - x \pmod{2^B}`.

        The immediate operand is a sign extended 64 bit constant.

        Also works as integer negation when `Y = 0`. Use `iadd_imm`
        with a negative immediate operand for the reverse immediate
        subtraction.

        Polymorphic over all scalar integer types, but does not support vector
        types.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "sadd_overflow_cin",
            r#"
        Add signed integers with carry in and overflow out.

        Same as `sadd_overflow` with an additional carry input. The `c_in` type
        is interpreted as 1 if it's nonzero or 0 if it's zero.
        "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("x", iB),
            Operand::new("y", iB),
            Operand::new("c_in", i8).with_doc("Input carry flag"),
        ])
        .operands_out(vec![
            Operand::new("a", iB),
            Operand::new("c_out", i8).with_doc("Output carry flag"),
        ]),
    );

    ig.push(
        Inst::new(
            "uadd_overflow_cin",
            r#"
        Add unsigned integers with carry in and overflow out.

        Same as `uadd_overflow` with an additional carry input. The `c_in` type
        is interpreted as 1 if it's nonzero or 0 if it's zero.
        "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("x", iB),
            Operand::new("y", iB),
            Operand::new("c_in", i8).with_doc("Input carry flag"),
        ])
        .operands_out(vec![
            Operand::new("a", iB),
            Operand::new("c_out", i8).with_doc("Output carry flag"),
        ]),
    );

    {
        let of_out = Operand::new("of", i8).with_doc("Overflow flag");
        ig.push(
            Inst::new(
                "uadd_overflow",
                r#"
            Add integers unsigned with overflow out.
            ``of`` is set when the addition overflowed.
            ```text
                a &= x + y \pmod 2^B \\
                of &= x+y >= 2^B
            ```
            Polymorphic over all scalar integer types, but does not support vector
            types.
            "#,
                &formats.binary,
            )
            .operands_in(vec![Operand::new("x", iB), Operand::new("y", iB)])
            .operands_out(vec![Operand::new("a", iB), of_out.clone()]),
        );

        ig.push(
            Inst::new(
                "sadd_overflow",
                r#"
            Add integers signed with overflow out.
            ``of`` is set when the addition over- or underflowed.
            Polymorphic over all scalar integer types, but does not support vector
            types.
            "#,
                &formats.binary,
            )
            .operands_in(vec![Operand::new("x", iB), Operand::new("y", iB)])
            .operands_out(vec![Operand::new("a", iB), of_out.clone()]),
        );

        ig.push(
            Inst::new(
                "usub_overflow",
                r#"
            Subtract integers unsigned with overflow out.
            ``of`` is set when the subtraction underflowed.
            ```text
                a &= x - y \pmod 2^B \\
                of &= x - y < 0
            ```
            Polymorphic over all scalar integer types, but does not support vector
            types.
            "#,
                &formats.binary,
            )
            .operands_in(vec![Operand::new("x", iB), Operand::new("y", iB)])
            .operands_out(vec![Operand::new("a", iB), of_out.clone()]),
        );

        ig.push(
            Inst::new(
                "ssub_overflow",
                r#"
            Subtract integers signed with overflow out.
            ``of`` is set when the subtraction over- or underflowed.
            Polymorphic over all scalar integer types, but does not support vector
            types.
            "#,
                &formats.binary,
            )
            .operands_in(vec![Operand::new("x", iB), Operand::new("y", iB)])
            .operands_out(vec![Operand::new("a", iB), of_out.clone()]),
        );

        {
            let NarrowScalar = &TypeVar::new(
                "NarrowScalar",
                "A scalar integer type up to 64 bits",
                TypeSetBuilder::new().ints(8..64).build(),
            );

            ig.push(
                Inst::new(
                    "umul_overflow",
                    r#"
                Multiply integers unsigned with overflow out.
                ``of`` is set when the multiplication overflowed.
                ```text
                    a &= x * y \pmod 2^B \\
                    of &= x * y > 2^B
                ```
                Polymorphic over all scalar integer types except i128, but does not support vector
                types.
                "#,
                    &formats.binary,
                )
                .operands_in(vec![
                    Operand::new("x", NarrowScalar),
                    Operand::new("y", NarrowScalar),
                ])
                .operands_out(vec![Operand::new("a", NarrowScalar), of_out.clone()]),
            );

            ig.push(
                Inst::new(
                    "smul_overflow",
                    r#"
                Multiply integers signed with overflow out.
                ``of`` is set when the multiplication over- or underflowed.
                Polymorphic over all scalar integer types except i128, but does not support vector
                types.
                "#,
                    &formats.binary,
                )
                .operands_in(vec![
                    Operand::new("x", NarrowScalar),
                    Operand::new("y", NarrowScalar),
                ])
                .operands_out(vec![Operand::new("a", NarrowScalar), of_out.clone()]),
            );
        }
    }

    let i32_64 = &TypeVar::new(
        "i32_64",
        "A 32 or 64-bit scalar integer type",
        TypeSetBuilder::new().ints(32..64).build(),
    );

    ig.push(
        Inst::new(
            "uadd_overflow_trap",
            r#"
        Unsigned addition of x and y, trapping if the result overflows.

        Accepts 32 or 64-bit integers, and does not support vector types.
        "#,
            &formats.int_add_trap,
        )
        .operands_in(vec![
            Operand::new("x", i32_64),
            Operand::new("y", i32_64),
            Operand::new("code", &imm.trapcode),
        ])
        .operands_out(vec![Operand::new("a", i32_64)])
        .can_trap()
        .side_effects_idempotent(),
    );

    ig.push(
        Inst::new(
            "ssub_overflow_bin",
            r#"
        Subtract signed integers with borrow in and overflow out.

        Same as `ssub_overflow` with an additional borrow input. The `b_in` type
        is interpreted as 1 if it's nonzero or 0 if it's zero. The computation
        performed here is `x - (y + (b_in != 0))`.
        "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("x", iB),
            Operand::new("y", iB),
            Operand::new("b_in", i8).with_doc("Input borrow flag"),
        ])
        .operands_out(vec![
            Operand::new("a", iB),
            Operand::new("b_out", i8).with_doc("Output borrow flag"),
        ]),
    );

    ig.push(
        Inst::new(
            "usub_overflow_bin",
            r#"
        Subtract unsigned integers with borrow in and overflow out.

        Same as `usub_overflow` with an additional borrow input. The `b_in` type
        is interpreted as 1 if it's nonzero or 0 if it's zero. The computation
        performed here is `x - (y + (b_in != 0))`.
        "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("x", iB),
            Operand::new("y", iB),
            Operand::new("b_in", i8).with_doc("Input borrow flag"),
        ])
        .operands_out(vec![
            Operand::new("a", iB),
            Operand::new("b_out", i8).with_doc("Output borrow flag"),
        ]),
    );

    let bits = &TypeVar::new(
        "bits",
        "Any integer, float, or vector type",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .floats(Interval::All)
            .simd_lanes(Interval::All)
            .includes_scalars(true)
            .build(),
    );

    ig.push(
        Inst::new(
            "band",
            r#"
        Bitwise and.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", bits), Operand::new("y", bits)])
        .operands_out(vec![Operand::new("a", bits)]),
    );

    ig.push(
        Inst::new(
            "bor",
            r#"
        Bitwise or.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", bits), Operand::new("y", bits)])
        .operands_out(vec![Operand::new("a", bits)]),
    );

    ig.push(
        Inst::new(
            "bxor",
            r#"
        Bitwise xor.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", bits), Operand::new("y", bits)])
        .operands_out(vec![Operand::new("a", bits)]),
    );

    ig.push(
        Inst::new(
            "bnot",
            r#"
        Bitwise not.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", bits)])
        .operands_out(vec![Operand::new("a", bits)]),
    );

    ig.push(
        Inst::new(
            "band_not",
            r#"
        Bitwise and not.

        Computes `x & ~y`.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", bits), Operand::new("y", bits)])
        .operands_out(vec![Operand::new("a", bits)]),
    );

    ig.push(
        Inst::new(
            "bor_not",
            r#"
        Bitwise or not.

        Computes `x | ~y`.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", bits), Operand::new("y", bits)])
        .operands_out(vec![Operand::new("a", bits)]),
    );

    ig.push(
        Inst::new(
            "bxor_not",
            r#"
        Bitwise xor not.

        Computes `x ^ ~y`.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", bits), Operand::new("y", bits)])
        .operands_out(vec![Operand::new("a", bits)]),
    );

    ig.push(
        Inst::new(
            "band_imm",
            r#"
        Bitwise and with immediate.

        Same as `band`, but one operand is a zero extended 64 bit immediate constant.

        Polymorphic over all scalar integer types, but does not support vector
        types.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "bor_imm",
            r#"
        Bitwise or with immediate.

        Same as `bor`, but one operand is a zero extended 64 bit immediate constant.

        Polymorphic over all scalar integer types, but does not support vector
        types.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "bxor_imm",
            r#"
        Bitwise xor with immediate.

        Same as `bxor`, but one operand is a zero extended 64 bit immediate constant.

        Polymorphic over all scalar integer types, but does not support vector
        types.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![Operand::new("x", iB), Operand::new("Y", &imm.imm64)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "rotl",
            r#"
        Rotate left.

        Rotate the bits in ``x`` by ``y`` places.
        "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("y", iB).with_doc("Number of bits to shift"),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "rotr",
            r#"
        Rotate right.

        Rotate the bits in ``x`` by ``y`` places.
        "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("y", iB).with_doc("Number of bits to shift"),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "rotl_imm",
            r#"
        Rotate left by immediate.

        Same as `rotl`, but one operand is a zero extended 64 bit immediate constant.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("Y", &imm.imm64),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "rotr_imm",
            r#"
        Rotate right by immediate.

        Same as `rotr`, but one operand is a zero extended 64 bit immediate constant.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("Y", &imm.imm64),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "ishl",
            r#"
        Integer shift left. Shift the bits in ``x`` towards the MSB by ``y``
        places. Shift in zero bits to the LSB.

        The shift amount is masked to the size of ``x``.

        When shifting a B-bits integer type, this instruction computes:

        ```text
            s &:= y \pmod B,
            a &:= x \cdot 2^s \pmod{2^B}.
        ```
        "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("y", iB).with_doc("Number of bits to shift"),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "ushr",
            r#"
        Unsigned shift right. Shift bits in ``x`` towards the LSB by ``y``
        places, shifting in zero bits to the MSB. Also called a *logical
        shift*.

        The shift amount is masked to the size of ``x``.

        When shifting a B-bits integer type, this instruction computes:

        ```text
            s &:= y \pmod B,
            a &:= \lfloor x \cdot 2^{-s} \rfloor.
        ```
        "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("y", iB).with_doc("Number of bits to shift"),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "sshr",
            r#"
        Signed shift right. Shift bits in ``x`` towards the LSB by ``y``
        places, shifting in sign bits to the MSB. Also called an *arithmetic
        shift*.

        The shift amount is masked to the size of ``x``.
        "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("y", iB).with_doc("Number of bits to shift"),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "ishl_imm",
            r#"
        Integer shift left by immediate.

        The shift amount is masked to the size of ``x``.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("Y", &imm.imm64),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "ushr_imm",
            r#"
        Unsigned shift right by immediate.

        The shift amount is masked to the size of ``x``.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("Y", &imm.imm64),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "sshr_imm",
            r#"
        Signed shift right by immediate.

        The shift amount is masked to the size of ``x``.
        "#,
            &formats.binary_imm64,
        )
        .operands_in(vec![
            Operand::new("x", Int).with_doc("Scalar or vector value to shift"),
            Operand::new("Y", &imm.imm64),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "bitrev",
            r#"
        Reverse the bits of a integer.

        Reverses the bits in ``x``.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", iB)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "clz",
            r#"
        Count leading zero bits.

        Starting from the MSB in ``x``, count the number of zero bits before
        reaching the first one bit. When ``x`` is zero, returns the size of x
        in bits.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", iB)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "cls",
            r#"
        Count leading sign bits.

        Starting from the MSB after the sign bit in ``x``, count the number of
        consecutive bits identical to the sign bit. When ``x`` is 0 or -1,
        returns one less than the size of x in bits.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", iB)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "ctz",
            r#"
        Count trailing zeros.

        Starting from the LSB in ``x``, count the number of zero bits before
        reaching the first one bit. When ``x`` is zero, returns the size of x
        in bits.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", iB)])
        .operands_out(vec![Operand::new("a", iB)]),
    );

    ig.push(
        Inst::new(
            "bswap",
            r#"
        Reverse the byte order of an integer.

        Reverses the bytes in ``x``.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", iSwappable)])
        .operands_out(vec![Operand::new("a", iSwappable)]),
    );

    ig.push(
        Inst::new(
            "popcnt",
            r#"
        Population count

        Count the number of one bits in ``x``.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Int)])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    let Float = &TypeVar::new(
        "Float",
        "A scalar or vector floating point number",
        TypeSetBuilder::new()
            .floats(Interval::All)
            .simd_lanes(Interval::All)
            .dynamic_simd_lanes(Interval::All)
            .build(),
    );

    ig.push(
        Inst::new(
            "fcmp",
            r#"
        Floating point comparison.

        Two IEEE 754-2008 floating point numbers, `x` and `y`, relate to each
        other in exactly one of four ways:

        ```text
        == ==========================================
        UN Unordered when one or both numbers is NaN.
        EQ When `x = y`. (And `0.0 = -0.0`).
        LT When `x < y`.
        GT When `x > y`.
        == ==========================================
        ```

        The 14 `floatcc` condition codes each correspond to a subset of
        the four relations, except for the empty set which would always be
        false, and the full set which would always be true.

        The condition codes are divided into 7 'ordered' conditions which don't
        include UN, and 7 unordered conditions which all include UN.

        ```text
        +-------+------------+---------+------------+-------------------------+
        |Ordered             |Unordered             |Condition                |
        +=======+============+=========+============+=========================+
        |ord    |EQ | LT | GT|uno      |UN          |NaNs absent / present.   |
        +-------+------------+---------+------------+-------------------------+
        |eq     |EQ          |ueq      |UN | EQ     |Equal                    |
        +-------+------------+---------+------------+-------------------------+
        |one    |LT | GT     |ne       |UN | LT | GT|Not equal                |
        +-------+------------+---------+------------+-------------------------+
        |lt     |LT          |ult      |UN | LT     |Less than                |
        +-------+------------+---------+------------+-------------------------+
        |le     |LT | EQ     |ule      |UN | LT | EQ|Less than or equal       |
        +-------+------------+---------+------------+-------------------------+
        |gt     |GT          |ugt      |UN | GT     |Greater than             |
        +-------+------------+---------+------------+-------------------------+
        |ge     |GT | EQ     |uge      |UN | GT | EQ|Greater than or equal    |
        +-------+------------+---------+------------+-------------------------+
        ```

        The standard C comparison operators, `<, <=, >, >=`, are all ordered,
        so they are false if either operand is NaN. The C equality operator,
        `==`, is ordered, and since inequality is defined as the logical
        inverse it is *unordered*. They map to the `floatcc` condition
        codes as follows:

        ```text
        ==== ====== ============
        C    `Cond` Subset
        ==== ====== ============
        `==` eq     EQ
        `!=` ne     UN | LT | GT
        `<`  lt     LT
        `<=` le     LT | EQ
        `>`  gt     GT
        `>=` ge     GT | EQ
        ==== ====== ============
        ```

        This subset of condition codes also corresponds to the WebAssembly
        floating point comparisons of the same name.

        When this instruction compares floating point vectors, it returns a
        vector with the results of lane-wise comparisons.

        When comparing scalars, the result is:
            - `1` if the condition holds.
            - `0` if the condition does not hold.

        When comparing vectors, the result is:
            - `-1` (i.e. all ones) in each lane where the condition holds.
            - `0` in each lane where the condition does not hold.
        "#,
            &formats.float_compare,
        )
        .operands_in(vec![
            Operand::new("Cond", &imm.floatcc),
            Operand::new("x", Float),
            Operand::new("y", Float),
        ])
        .operands_out(vec![Operand::new("a", &Float.as_truthy())]),
    );

    ig.push(
        Inst::new(
            "fadd",
            r#"
        Floating point addition.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Float), Operand::new("y", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("Result of applying operator to each lane"),
        ]),
    );

    ig.push(
        Inst::new(
            "fsub",
            r#"
        Floating point subtraction.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Float), Operand::new("y", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("Result of applying operator to each lane"),
        ]),
    );

    ig.push(
        Inst::new(
            "fmul",
            r#"
        Floating point multiplication.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Float), Operand::new("y", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("Result of applying operator to each lane"),
        ]),
    );

    ig.push(
        Inst::new(
            "fdiv",
            r#"
        Floating point division.

        Unlike the integer division instructions ` and
        `udiv`, this can't trap. Division by zero is infinity or
        NaN, depending on the dividend.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Float), Operand::new("y", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("Result of applying operator to each lane"),
        ]),
    );

    ig.push(
        Inst::new(
            "sqrt",
            r#"
        Floating point square root.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("Result of applying operator to each lane"),
        ]),
    );

    ig.push(
        Inst::new(
            "fma",
            r#"
        Floating point fused multiply-and-add.

        Computes `a := xy+z` without any intermediate rounding of the
        product.
        "#,
            &formats.ternary,
        )
        .operands_in(vec![
            Operand::new("x", Float),
            Operand::new("y", Float),
            Operand::new("z", Float),
        ])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("Result of applying operator to each lane"),
        ]),
    );

    ig.push(
        Inst::new(
            "fneg",
            r#"
        Floating point negation.

        Note that this is a pure bitwise operation.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("``x`` with its sign bit inverted"),
        ]),
    );

    ig.push(
        Inst::new(
            "fabs",
            r#"
        Floating point absolute value.

        Note that this is a pure bitwise operation.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("``x`` with its sign bit cleared"),
        ]),
    );

    ig.push(
        Inst::new(
            "fcopysign",
            r#"
        Floating point copy sign.

        Note that this is a pure bitwise operation. The sign bit from ``y`` is
        copied to the sign bit of ``x``.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Float), Operand::new("y", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("``x`` with its sign bit changed to that of ``y``"),
        ]),
    );

    ig.push(
        Inst::new(
            "fmin",
            r#"
        Floating point minimum, propagating NaNs using the WebAssembly rules.

        If either operand is NaN, this returns NaN with an unspecified sign. Furthermore, if
        each input NaN consists of a mantissa whose most significant bit is 1 and the rest is
        0, then the output has the same form. Otherwise, the output mantissa's most significant
        bit is 1 and the rest is unspecified.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Float), Operand::new("y", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("The smaller of ``x`` and ``y``"),
        ]),
    );

    ig.push(
        Inst::new(
            "fmax",
            r#"
        Floating point maximum, propagating NaNs using the WebAssembly rules.

        If either operand is NaN, this returns NaN with an unspecified sign. Furthermore, if
        each input NaN consists of a mantissa whose most significant bit is 1 and the rest is
        0, then the output has the same form. Otherwise, the output mantissa's most significant
        bit is 1 and the rest is unspecified.
        "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", Float), Operand::new("y", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("The larger of ``x`` and ``y``"),
        ]),
    );

    ig.push(
        Inst::new(
            "ceil",
            r#"
        Round floating point round to integral, towards positive infinity.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("``x`` rounded to integral value"),
        ]),
    );

    ig.push(
        Inst::new(
            "floor",
            r#"
        Round floating point round to integral, towards negative infinity.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("``x`` rounded to integral value"),
        ]),
    );

    ig.push(
        Inst::new(
            "trunc",
            r#"
        Round floating point round to integral, towards zero.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("``x`` rounded to integral value"),
        ]),
    );

    ig.push(
        Inst::new(
            "nearest",
            r#"
        Round floating point round to integral, towards nearest with ties to
        even.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![
            Operand::new("a", Float).with_doc("``x`` rounded to integral value"),
        ]),
    );

    ig.push(
        Inst::new(
            "bitcast",
            r#"
        Reinterpret the bits in `x` as a different type.

        The input and output types must be storable to memory and of the same
        size. A bitcast is equivalent to storing one type and loading the other
        type from the same address, both using the specified MemFlags.

        Note that this operation only supports the `big` or `little` MemFlags.
        The specified byte order only affects the result in the case where
        input and output types differ in lane count/size.  In this case, the
        operation is only valid if a byte order specifier is provided.
        "#,
            &formats.load_no_offset,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("x", Mem),
        ])
        .operands_out(vec![
            Operand::new("a", MemTo).with_doc("Bits of `x` reinterpreted"),
        ]),
    );

    ig.push(
        Inst::new(
            "scalar_to_vector",
            r#"
            Copies a scalar value to a vector value.  The scalar is copied into the
            least significant lane of the vector, and all other lanes will be zero.
            "#,
            &formats.unary,
        )
        .operands_in(vec![
            Operand::new("s", &TxN.lane_of()).with_doc("A scalar value"),
        ])
        .operands_out(vec![Operand::new("a", TxN).with_doc("A vector value")]),
    );

    let Truthy = &TypeVar::new(
        "Truthy",
        "A scalar whose values are truthy",
        TypeSetBuilder::new().ints(Interval::All).build(),
    );
    let IntTo = &TypeVar::new(
        "IntTo",
        "An integer type",
        TypeSetBuilder::new().ints(Interval::All).build(),
    );

    ig.push(
        Inst::new(
            "bmask",
            r#"
        Convert `x` to an integer mask.

        Non-zero maps to all 1s and zero maps to all 0s.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Truthy)])
        .operands_out(vec![Operand::new("a", IntTo)]),
    );

    let Int = &TypeVar::new(
        "Int",
        "A scalar integer type",
        TypeSetBuilder::new().ints(Interval::All).build(),
    );

    ig.push(
        Inst::new(
            "ireduce",
            r#"
        Convert `x` to a smaller integer type by discarding
        the most significant bits.

        This is the same as reducing modulo `2^n`.
        "#,
            &formats.unary,
        )
        .operands_in(vec![
            Operand::new("x", &Int.wider())
                .with_doc("A scalar integer type, wider than the controlling type"),
        ])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    let I16or32or64xN = &TypeVar::new(
        "I16or32or64xN",
        "A SIMD vector type containing integer lanes 16, 32, or 64 bits wide",
        TypeSetBuilder::new()
            .ints(16..64)
            .simd_lanes(2..8)
            .dynamic_simd_lanes(2..8)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "snarrow",
            r#"
        Combine `x` and `y` into a vector with twice the lanes but half the integer width while
        saturating overflowing values to the signed maximum and minimum.

        The lanes will be concatenated after narrowing. For example, when `x` and `y` are `i32x4`
        and `x = [x3, x2, x1, x0]` and `y = [y3, y2, y1, y0]`, then after narrowing the value
        returned is an `i16x8`: `a = [y3', y2', y1', y0', x3', x2', x1', x0']`.
            "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", I16or32or64xN),
            Operand::new("y", I16or32or64xN),
        ])
        .operands_out(vec![Operand::new("a", &I16or32or64xN.split_lanes())]),
    );

    ig.push(
        Inst::new(
            "unarrow",
            r#"
        Combine `x` and `y` into a vector with twice the lanes but half the integer width while
        saturating overflowing values to the unsigned maximum and minimum.

        Note that all input lanes are considered signed: any negative lanes will overflow and be
        replaced with the unsigned minimum, `0x00`.

        The lanes will be concatenated after narrowing. For example, when `x` and `y` are `i32x4`
        and `x = [x3, x2, x1, x0]` and `y = [y3, y2, y1, y0]`, then after narrowing the value
        returned is an `i16x8`: `a = [y3', y2', y1', y0', x3', x2', x1', x0']`.
            "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", I16or32or64xN),
            Operand::new("y", I16or32or64xN),
        ])
        .operands_out(vec![Operand::new("a", &I16or32or64xN.split_lanes())]),
    );

    ig.push(
        Inst::new(
            "uunarrow",
            r#"
        Combine `x` and `y` into a vector with twice the lanes but half the integer width while
        saturating overflowing values to the unsigned maximum and minimum.

        Note that all input lanes are considered unsigned: any negative values will be interpreted as unsigned, overflowing and being replaced with the unsigned maximum.

        The lanes will be concatenated after narrowing. For example, when `x` and `y` are `i32x4`
        and `x = [x3, x2, x1, x0]` and `y = [y3, y2, y1, y0]`, then after narrowing the value
        returned is an `i16x8`: `a = [y3', y2', y1', y0', x3', x2', x1', x0']`.
            "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", I16or32or64xN), Operand::new("y", I16or32or64xN)])
        .operands_out(vec![Operand::new("a", &I16or32or64xN.split_lanes())]),
    );

    let I8or16or32xN = &TypeVar::new(
        "I8or16or32xN",
        "A SIMD vector type containing integer lanes 8, 16, or 32 bits wide.",
        TypeSetBuilder::new()
            .ints(8..32)
            .simd_lanes(2..16)
            .dynamic_simd_lanes(2..16)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "swiden_low",
            r#"
        Widen the low lanes of `x` using signed extension.

        This will double the lane width and halve the number of lanes.
            "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", I8or16or32xN)])
        .operands_out(vec![Operand::new("a", &I8or16or32xN.merge_lanes())]),
    );

    ig.push(
        Inst::new(
            "swiden_high",
            r#"
        Widen the high lanes of `x` using signed extension.

        This will double the lane width and halve the number of lanes.
            "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", I8or16or32xN)])
        .operands_out(vec![Operand::new("a", &I8or16or32xN.merge_lanes())]),
    );

    ig.push(
        Inst::new(
            "uwiden_low",
            r#"
        Widen the low lanes of `x` using unsigned extension.

        This will double the lane width and halve the number of lanes.
            "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", I8or16or32xN)])
        .operands_out(vec![Operand::new("a", &I8or16or32xN.merge_lanes())]),
    );

    ig.push(
        Inst::new(
            "uwiden_high",
            r#"
            Widen the high lanes of `x` using unsigned extension.

            This will double the lane width and halve the number of lanes.
            "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", I8or16or32xN)])
        .operands_out(vec![Operand::new("a", &I8or16or32xN.merge_lanes())]),
    );

    ig.push(
        Inst::new(
            "iadd_pairwise",
            r#"
        Does lane-wise integer pairwise addition on two operands, putting the
        combined results into a single vector result. Here a pair refers to adjacent
        lanes in a vector, i.e. i*2 + (i*2+1) for i == num_lanes/2. The first operand
        pairwise add results will make up the low half of the resulting vector while
        the second operand pairwise add results will make up the upper half of the
        resulting vector.
            "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("x", I8or16or32xN),
            Operand::new("y", I8or16or32xN),
        ])
        .operands_out(vec![Operand::new("a", I8or16or32xN)]),
    );

    let I8x16 = &TypeVar::new(
        "I8x16",
        "A SIMD vector type consisting of 16 lanes of 8-bit integers",
        TypeSetBuilder::new()
            .ints(8..8)
            .simd_lanes(16..16)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "x86_pmaddubsw",
            r#"
        An instruction with equivalent semantics to `pmaddubsw` on x86.

        This instruction will take signed bytes from the first argument and
        multiply them against unsigned bytes in the second argument. Adjacent
        pairs are then added, with saturating, to a 16-bit value and are packed
        into the result.
            "#,
            &formats.binary,
        )
        .operands_in(vec![Operand::new("x", I8x16), Operand::new("y", I8x16)])
        .operands_out(vec![Operand::new("a", I16x8)]),
    );

    ig.push(
        Inst::new(
            "uextend",
            r#"
        Convert `x` to a larger integer type by zero-extending.

        Each lane in `x` is converted to a larger integer type by adding
        zeroes. The result has the same numerical value as `x` when both are
        interpreted as unsigned integers.

        The result type must have the same number of vector lanes as the input,
        and each lane must not have fewer bits that the input lanes. If the
        input and output types are the same, this is a no-op.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", &Int.narrower()).with_doc(
            "A scalar integer type, narrower than the controlling type",
        )])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    ig.push(
        Inst::new(
            "sextend",
            r#"
        Convert `x` to a larger integer type by sign-extending.

        Each lane in `x` is converted to a larger integer type by replicating
        the sign bit. The result has the same numerical value as `x` when both
        are interpreted as signed integers.

        The result type must have the same number of vector lanes as the input,
        and each lane must not have fewer bits that the input lanes. If the
        input and output types are the same, this is a no-op.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", &Int.narrower()).with_doc(
            "A scalar integer type, narrower than the controlling type",
        )])
        .operands_out(vec![Operand::new("a", Int)]),
    );

    let FloatScalar = &TypeVar::new(
        "FloatScalar",
        "A scalar only floating point number",
        TypeSetBuilder::new().floats(Interval::All).build(),
    );

    ig.push(
        Inst::new(
            "fpromote",
            r#"
        Convert `x` to a larger floating point format.

        Each lane in `x` is converted to the destination floating point format.
        This is an exact operation.

        Cranelift currently only supports two floating point formats
        - `f32` and `f64`. This may change in the future.

        The result type must have the same number of vector lanes as the input,
        and the result lanes must not have fewer bits than the input lanes.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", &FloatScalar.narrower()).with_doc(
            "A scalar only floating point number, narrower than the controlling type",
        )])
        .operands_out(vec![Operand::new("a", FloatScalar)]),
    );

    ig.push(
        Inst::new(
            "fdemote",
            r#"
        Convert `x` to a smaller floating point format.

        Each lane in `x` is converted to the destination floating point format
        by rounding to nearest, ties to even.

        Cranelift currently only supports two floating point formats
        - `f32` and `f64`. This may change in the future.

        The result type must have the same number of vector lanes as the input,
        and the result lanes must not have more bits than the input lanes.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", &FloatScalar.wider()).with_doc(
            "A scalar only floating point number, wider than the controlling type",
        )])
        .operands_out(vec![Operand::new("a", FloatScalar)]),
    );

    let F64x2 = &TypeVar::new(
        "F64x2",
        "A SIMD vector type consisting of 2 lanes of 64-bit floats",
        TypeSetBuilder::new()
            .floats(64..64)
            .simd_lanes(2..2)
            .includes_scalars(false)
            .build(),
    );
    let F32x4 = &TypeVar::new(
        "F32x4",
        "A SIMD vector type consisting of 4 lanes of 32-bit floats",
        TypeSetBuilder::new()
            .floats(32..32)
            .simd_lanes(4..4)
            .includes_scalars(false)
            .build(),
    );

    ig.push(
        Inst::new(
            "fvdemote",
            r#"
                Convert `x` to a smaller floating point format.

                Each lane in `x` is converted to the destination floating point format
                by rounding to nearest, ties to even.

                Cranelift currently only supports two floating point formats
                - `f32` and `f64`. This may change in the future.

                Fvdemote differs from fdemote in that with fvdemote it targets vectors.
                Fvdemote is constrained to having the input type being F64x2 and the result
                type being F32x4. The result lane that was the upper half of the input lane
                is initialized to zero.
                "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", F64x2)])
        .operands_out(vec![Operand::new("a", F32x4)]),
    );

    ig.push(
        Inst::new(
            "fvpromote_low",
            r#"
        Converts packed single precision floating point to packed double precision floating point.

        Considering only the lower half of the register, the low lanes in `x` are interpreted as
        single precision floats that are then converted to a double precision floats.

        The result type will have half the number of vector lanes as the input. Fvpromote_low is
        constrained to input F32x4 with a result type of F64x2.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("a", F32x4)])
        .operands_out(vec![Operand::new("x", F64x2)]),
    );

    let IntTo = &TypeVar::new(
        "IntTo",
        "An scalar only integer type",
        TypeSetBuilder::new().ints(Interval::All).build(),
    );

    ig.push(
        Inst::new(
            "fcvt_to_uint",
            r#"
        Converts floating point scalars to unsigned integer.

        Only operates on `x` if it is a scalar. If `x` is NaN or if
        the unsigned integral value cannot be represented in the result
        type, this instruction traps.

        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", FloatScalar)])
        .operands_out(vec![Operand::new("a", IntTo)])
        .can_trap()
        .side_effects_idempotent(),
    );

    ig.push(
        Inst::new(
            "fcvt_to_sint",
            r#"
        Converts floating point scalars to signed integer.

        Only operates on `x` if it is a scalar. If `x` is NaN or if
        the unsigned integral value cannot be represented in the result
        type, this instruction traps.

        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", FloatScalar)])
        .operands_out(vec![Operand::new("a", IntTo)])
        .can_trap()
        .side_effects_idempotent(),
    );

    let IntTo = &TypeVar::new(
        "IntTo",
        "A larger integer type with the same number of lanes",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .simd_lanes(Interval::All)
            .build(),
    );

    ig.push(
        Inst::new(
            "fcvt_to_uint_sat",
            r#"
        Convert floating point to unsigned integer as fcvt_to_uint does, but
        saturates the input instead of trapping. NaN and negative values are
        converted to 0.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![Operand::new("a", IntTo)]),
    );

    ig.push(
        Inst::new(
            "fcvt_to_sint_sat",
            r#"
        Convert floating point to signed integer as fcvt_to_sint does, but
        saturates the input instead of trapping. NaN values are converted to 0.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![Operand::new("a", IntTo)]),
    );

    ig.push(
        Inst::new(
            "x86_cvtt2dq",
            r#"
        A float-to-integer conversion instruction for vectors-of-floats which
        has the same semantics as `cvttp{s,d}2dq` on x86. This specifically
        returns `INT_MIN` for NaN or out-of-bounds lanes.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Float)])
        .operands_out(vec![Operand::new("a", IntTo)]),
    );

    let Int = &TypeVar::new(
        "Int",
        "A scalar or vector integer type",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .simd_lanes(Interval::All)
            .build(),
    );

    let FloatTo = &TypeVar::new(
        "FloatTo",
        "A scalar or vector floating point number",
        TypeSetBuilder::new()
            .floats(Interval::All)
            .simd_lanes(Interval::All)
            .build(),
    );

    ig.push(
        Inst::new(
            "fcvt_from_uint",
            r#"
        Convert unsigned integer to floating point.

        Each lane in `x` is interpreted as an unsigned integer and converted to
        floating point using round to nearest, ties to even.

        The result type must have the same number of vector lanes as the input.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Int)])
        .operands_out(vec![Operand::new("a", FloatTo)]),
    );

    ig.push(
        Inst::new(
            "fcvt_from_sint",
            r#"
        Convert signed integer to floating point.

        Each lane in `x` is interpreted as a signed integer and converted to
        floating point using round to nearest, ties to even.

        The result type must have the same number of vector lanes as the input.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", Int)])
        .operands_out(vec![Operand::new("a", FloatTo)]),
    );

    let WideInt = &TypeVar::new(
        "WideInt",
        "An integer type of width `i16` upwards",
        TypeSetBuilder::new().ints(16..128).build(),
    );

    ig.push(
        Inst::new(
            "isplit",
            r#"
        Split an integer into low and high parts.

        Vectors of integers are split lane-wise, so the results have the same
        number of lanes as the input, but the lanes are half the size.

        Returns the low half of `x` and the high half of `x` as two independent
        values.
        "#,
            &formats.unary,
        )
        .operands_in(vec![Operand::new("x", WideInt)])
        .operands_out(vec![
            Operand::new("lo", &WideInt.half_width()).with_doc("The low bits of `x`"),
            Operand::new("hi", &WideInt.half_width()).with_doc("The high bits of `x`"),
        ]),
    );

    ig.push(
        Inst::new(
            "iconcat",
            r#"
        Concatenate low and high bits to form a larger integer type.

        Vectors of integers are concatenated lane-wise such that the result has
        the same number of lanes as the inputs, but the lanes are twice the
        size.
        "#,
            &formats.binary,
        )
        .operands_in(vec![
            Operand::new("lo", NarrowInt),
            Operand::new("hi", NarrowInt),
        ])
        .operands_out(vec![
            Operand::new("a", &NarrowInt.double_width())
                .with_doc("The concatenation of `lo` and `hi`"),
        ]),
    );

    // Instructions relating to atomic memory accesses and fences
    let AtomicMem = &TypeVar::new(
        "AtomicMem",
        "Any type that can be stored in memory, which can be used in an atomic operation",
        TypeSetBuilder::new().ints(8..128).build(),
    );

    ig.push(
        Inst::new(
            "atomic_rmw",
            r#"
        Atomically read-modify-write memory at `p`, with second operand `x`.  The old value is
        returned.  `p` has the type of the target word size, and `x` may be any integer type; note
        that some targets require specific target features to be enabled in order to support 128-bit
        integer atomics.  The type of the returned value is the same as the type of `x`.  This
        operation is sequentially consistent and creates happens-before edges that order normal
        (non-atomic) loads and stores.
        "#,
            &formats.atomic_rmw,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("AtomicRmwOp", &imm.atomic_rmw_op),
            Operand::new("p", iAddr),
            Operand::new("x", AtomicMem).with_doc("Value to be atomically stored"),
        ])
        .operands_out(vec![
            Operand::new("a", AtomicMem).with_doc("Value atomically loaded"),
        ])
        .can_load()
        .can_store()
        .other_side_effects(),
    );

    ig.push(
        Inst::new(
            "atomic_cas",
            r#"
        Perform an atomic compare-and-swap operation on memory at `p`, with expected value `e`,
        storing `x` if the value at `p` equals `e`.  The old value at `p` is returned,
        regardless of whether the operation succeeds or fails.  `p` has the type of the target
        word size, and `x` and `e` must have the same type and the same size, which may be any
        integer type; note that some targets require specific target features to be enabled in order
        to support 128-bit integer atomics.  The type of the returned value is the same as the type
        of `x` and `e`.  This operation is sequentially consistent and creates happens-before edges
        that order normal (non-atomic) loads and stores.
        "#,
            &formats.atomic_cas,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
            Operand::new("e", AtomicMem).with_doc("Expected value in CAS"),
            Operand::new("x", AtomicMem).with_doc("Value to be atomically stored"),
        ])
        .operands_out(vec![
            Operand::new("a", AtomicMem).with_doc("Value atomically loaded"),
        ])
        .can_load()
        .can_store()
        .other_side_effects(),
    );

    ig.push(
        Inst::new(
            "atomic_load",
            r#"
        Atomically load from memory at `p`.

        This is a polymorphic instruction that can load any value type which has a memory
        representation.  It can only be used for integer types; note that some targets require
        specific target features to be enabled in order to support 128-bit integer atomics. This
        operation is sequentially consistent and creates happens-before edges that order normal
        (non-atomic) loads and stores.
        "#,
            &formats.load_no_offset,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("p", iAddr),
        ])
        .operands_out(vec![
            Operand::new("a", AtomicMem).with_doc("Value atomically loaded"),
        ])
        .can_load()
        .other_side_effects(),
    );

    ig.push(
        Inst::new(
            "atomic_store",
            r#"
        Atomically store `x` to memory at `p`.

        This is a polymorphic instruction that can store any value type with a memory
        representation.  It can only be used for integer types; note that some targets require
        specific target features to be enabled in order to support 128-bit integer atomics This
        operation is sequentially consistent and creates happens-before edges that order normal
        (non-atomic) loads and stores.
        "#,
            &formats.store_no_offset,
        )
        .operands_in(vec![
            Operand::new("MemFlags", &imm.memflags),
            Operand::new("x", AtomicMem).with_doc("Value to be atomically stored"),
            Operand::new("p", iAddr),
        ])
        .can_store()
        .other_side_effects(),
    );

    ig.push(
        Inst::new(
            "fence",
            r#"
        A memory fence.  This must provide ordering to ensure that, at a minimum, neither loads
        nor stores of any kind may move forwards or backwards across the fence.  This operation
        is sequentially consistent.
        "#,
            &formats.nullary,
        )
        .other_side_effects(),
    );

    let TxN = &TypeVar::new(
        "TxN",
        "A dynamic vector type",
        TypeSetBuilder::new()
            .ints(Interval::All)
            .floats(Interval::All)
            .dynamic_simd_lanes(Interval::All)
            .build(),
    );

    ig.push(
        Inst::new(
            "extract_vector",
            r#"
        Return a fixed length sub vector, extracted from a dynamic vector.
        "#,
            &formats.binary_imm8,
        )
        .operands_in(vec![
            Operand::new("x", TxN).with_doc("The dynamic vector to extract from"),
            Operand::new("y", &imm.uimm8).with_doc("128-bit vector index"),
        ])
        .operands_out(vec![
            Operand::new("a", &TxN.dynamic_to_vector()).with_doc("New fixed vector"),
        ]),
    );
}
