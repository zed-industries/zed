//! This module is the central place for machine code emission.
//! It defines an implementation of wasmparser's Visitor trait
//! for `CodeGen`; which defines a visitor per op-code,
//! which validates and dispatches to the corresponding
//! machine code emitter.

use crate::abi::RetArea;
use crate::codegen::{
    Callee, CodeGen, CodeGenError, ConditionalBranch, ControlStackFrame, Emission, FnCall,
    UnconditionalBranch, control_index,
};
use crate::masm::{
    AtomicWaitKind, DivKind, Extend, ExtractLaneKind, FloatCmpKind, IntCmpKind, LoadKind,
    MacroAssembler, MulWideKind, OperandSize, RegImm, RemKind, ReplaceLaneKind, RmwOp,
    RoundingMode, SPOffset, ShiftKind, Signed, SplatKind, SplatLoadKind, StoreKind, TruncKind,
    V128AbsKind, V128AddKind, V128ConvertKind, V128ExtAddKind, V128ExtMulKind, V128ExtendKind,
    V128LoadExtendKind, V128MaxKind, V128MinKind, V128MulKind, V128NarrowKind, V128NegKind,
    V128SubKind, V128TruncKind, VectorCompareKind, VectorEqualityKind, Zero,
};

use crate::reg::{Reg, writable};
use crate::stack::{TypedReg, Val};
use anyhow::{Result, anyhow, bail, ensure, format_err};
use regalloc2::RegClass;
use smallvec::{SmallVec, smallvec};
use wasmparser::{
    BlockType, BrTable, Ieee32, Ieee64, MemArg, V128, VisitOperator, VisitSimdOperator,
};
use wasmtime_cranelift::TRAP_INDIRECT_CALL_TO_NULL;
use wasmtime_environ::{
    FUNCREF_INIT_BIT, FuncIndex, GlobalIndex, IndexType, MemoryIndex, TableIndex, TypeIndex,
    WasmHeapType, WasmValType,
};

/// A macro to define unsupported WebAssembly operators.
///
/// This macro calls itself recursively;
/// 1. It no-ops when matching a supported operator.
/// 2. Defines the visitor function and panics when
///    matching an unsupported operator.
macro_rules! def_unsupported {
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident $ann:tt)*) => {
        $(
            def_unsupported!(
                emit
                    $op

                fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {
                    $($(let _ = $arg;)*)?

                    Err(anyhow!(CodeGenError::unimplemented_wasm_instruction()))
                }
            );
        )*
    };

    (emit I32Const $($rest:tt)*) => {};
    (emit I64Const $($rest:tt)*) => {};
    (emit F32Const $($rest:tt)*) => {};
    (emit F64Const $($rest:tt)*) => {};
    (emit V128Const $($rest:tt)*) => {};
    (emit F32Add $($rest:tt)*) => {};
    (emit F64Add $($rest:tt)*) => {};
    (emit F32Sub $($rest:tt)*) => {};
    (emit F64Sub $($rest:tt)*) => {};
    (emit F32Mul $($rest:tt)*) => {};
    (emit F64Mul $($rest:tt)*) => {};
    (emit F32Div $($rest:tt)*) => {};
    (emit F64Div $($rest:tt)*) => {};
    (emit F32Min $($rest:tt)*) => {};
    (emit F64Min $($rest:tt)*) => {};
    (emit F32Max $($rest:tt)*) => {};
    (emit F64Max $($rest:tt)*) => {};
    (emit F32Copysign $($rest:tt)*) => {};
    (emit F64Copysign $($rest:tt)*) => {};
    (emit F32Abs $($rest:tt)*) => {};
    (emit F64Abs $($rest:tt)*) => {};
    (emit F32Neg $($rest:tt)*) => {};
    (emit F64Neg $($rest:tt)*) => {};
    (emit F32Floor $($rest:tt)*) => {};
    (emit F64Floor $($rest:tt)*) => {};
    (emit F32Ceil $($rest:tt)*) => {};
    (emit F64Ceil $($rest:tt)*) => {};
    (emit F32Nearest $($rest:tt)*) => {};
    (emit F64Nearest $($rest:tt)*) => {};
    (emit F32Trunc $($rest:tt)*) => {};
    (emit F64Trunc $($rest:tt)*) => {};
    (emit F32Sqrt $($rest:tt)*) => {};
    (emit F64Sqrt $($rest:tt)*) => {};
    (emit F32Eq $($rest:tt)*) => {};
    (emit F64Eq $($rest:tt)*) => {};
    (emit F32Ne $($rest:tt)*) => {};
    (emit F64Ne $($rest:tt)*) => {};
    (emit F32Lt $($rest:tt)*) => {};
    (emit F64Lt $($rest:tt)*) => {};
    (emit F32Gt $($rest:tt)*) => {};
    (emit F64Gt $($rest:tt)*) => {};
    (emit F32Le $($rest:tt)*) => {};
    (emit F64Le $($rest:tt)*) => {};
    (emit F32Ge $($rest:tt)*) => {};
    (emit F64Ge $($rest:tt)*) => {};
    (emit F32ConvertI32S $($rest:tt)*) => {};
    (emit F32ConvertI32U $($rest:tt)*) => {};
    (emit F32ConvertI64S $($rest:tt)*) => {};
    (emit F32ConvertI64U $($rest:tt)*) => {};
    (emit F64ConvertI32S $($rest:tt)*) => {};
    (emit F64ConvertI32U $($rest:tt)*) => {};
    (emit F64ConvertI64S $($rest:tt)*) => {};
    (emit F64ConvertI64U $($rest:tt)*) => {};
    (emit F32ReinterpretI32 $($rest:tt)*) => {};
    (emit F64ReinterpretI64 $($rest:tt)*) => {};
    (emit F32DemoteF64 $($rest:tt)*) => {};
    (emit F64PromoteF32 $($rest:tt)*) => {};
    (emit I32Add $($rest:tt)*) => {};
    (emit I64Add $($rest:tt)*) => {};
    (emit I32Sub $($rest:tt)*) => {};
    (emit I32Mul $($rest:tt)*) => {};
    (emit I32DivS $($rest:tt)*) => {};
    (emit I32DivU $($rest:tt)*) => {};
    (emit I64DivS $($rest:tt)*) => {};
    (emit I64DivU $($rest:tt)*) => {};
    (emit I64RemU $($rest:tt)*) => {};
    (emit I64RemS $($rest:tt)*) => {};
    (emit I32RemU $($rest:tt)*) => {};
    (emit I32RemS $($rest:tt)*) => {};
    (emit I64Mul $($rest:tt)*) => {};
    (emit I64Sub $($rest:tt)*) => {};
    (emit I32Eq $($rest:tt)*) => {};
    (emit I64Eq $($rest:tt)*) => {};
    (emit I32Ne $($rest:tt)*) => {};
    (emit I64Ne $($rest:tt)*) => {};
    (emit I32LtS $($rest:tt)*) => {};
    (emit I64LtS $($rest:tt)*) => {};
    (emit I32LtU $($rest:tt)*) => {};
    (emit I64LtU $($rest:tt)*) => {};
    (emit I32LeS $($rest:tt)*) => {};
    (emit I64LeS $($rest:tt)*) => {};
    (emit I32LeU $($rest:tt)*) => {};
    (emit I64LeU $($rest:tt)*) => {};
    (emit I32GtS $($rest:tt)*) => {};
    (emit I64GtS $($rest:tt)*) => {};
    (emit I32GtU $($rest:tt)*) => {};
    (emit I64GtU $($rest:tt)*) => {};
    (emit I32GeS $($rest:tt)*) => {};
    (emit I64GeS $($rest:tt)*) => {};
    (emit I32GeU $($rest:tt)*) => {};
    (emit I64GeU $($rest:tt)*) => {};
    (emit I32Eqz $($rest:tt)*) => {};
    (emit I64Eqz $($rest:tt)*) => {};
    (emit I32And $($rest:tt)*) => {};
    (emit I64And $($rest:tt)*) => {};
    (emit I32Or $($rest:tt)*) => {};
    (emit I64Or $($rest:tt)*) => {};
    (emit I32Xor $($rest:tt)*) => {};
    (emit I64Xor $($rest:tt)*) => {};
    (emit I32Shl $($rest:tt)*) => {};
    (emit I64Shl $($rest:tt)*) => {};
    (emit I32ShrS $($rest:tt)*) => {};
    (emit I64ShrS $($rest:tt)*) => {};
    (emit I32ShrU $($rest:tt)*) => {};
    (emit I64ShrU $($rest:tt)*) => {};
    (emit I32Rotl $($rest:tt)*) => {};
    (emit I64Rotl $($rest:tt)*) => {};
    (emit I32Rotr $($rest:tt)*) => {};
    (emit I64Rotr $($rest:tt)*) => {};
    (emit I32Clz $($rest:tt)*) => {};
    (emit I64Clz $($rest:tt)*) => {};
    (emit I32Ctz $($rest:tt)*) => {};
    (emit I64Ctz $($rest:tt)*) => {};
    (emit I32Popcnt $($rest:tt)*) => {};
    (emit I64Popcnt $($rest:tt)*) => {};
    (emit I32WrapI64 $($rest:tt)*) => {};
    (emit I64ExtendI32S $($rest:tt)*) => {};
    (emit I64ExtendI32U $($rest:tt)*) => {};
    (emit I32Extend8S $($rest:tt)*) => {};
    (emit I32Extend16S $($rest:tt)*) => {};
    (emit I64Extend8S $($rest:tt)*) => {};
    (emit I64Extend16S $($rest:tt)*) => {};
    (emit I64Extend32S $($rest:tt)*) => {};
    (emit I32TruncF32S $($rest:tt)*) => {};
    (emit I32TruncF32U $($rest:tt)*) => {};
    (emit I32TruncF64S $($rest:tt)*) => {};
    (emit I32TruncF64U $($rest:tt)*) => {};
    (emit I64TruncF32S $($rest:tt)*) => {};
    (emit I64TruncF32U $($rest:tt)*) => {};
    (emit I64TruncF64S $($rest:tt)*) => {};
    (emit I64TruncF64U $($rest:tt)*) => {};
    (emit I32ReinterpretF32 $($rest:tt)*) => {};
    (emit I64ReinterpretF64 $($rest:tt)*) => {};
    (emit LocalGet $($rest:tt)*) => {};
    (emit LocalSet $($rest:tt)*) => {};
    (emit Call $($rest:tt)*) => {};
    (emit End $($rest:tt)*) => {};
    (emit Nop $($rest:tt)*) => {};
    (emit If $($rest:tt)*) => {};
    (emit Else $($rest:tt)*) => {};
    (emit Block $($rest:tt)*) => {};
    (emit Loop $($rest:tt)*) => {};
    (emit Br $($rest:tt)*) => {};
    (emit BrIf $($rest:tt)*) => {};
    (emit Return $($rest:tt)*) => {};
    (emit Unreachable $($rest:tt)*) => {};
    (emit LocalTee $($rest:tt)*) => {};
    (emit GlobalGet $($rest:tt)*) => {};
    (emit GlobalSet $($rest:tt)*) => {};
    (emit Select $($rest:tt)*) => {};
    (emit Drop $($rest:tt)*) => {};
    (emit BrTable $($rest:tt)*) => {};
    (emit CallIndirect $($rest:tt)*) => {};
    (emit TableInit $($rest:tt)*) => {};
    (emit TableCopy $($rest:tt)*) => {};
    (emit TableGet $($rest:tt)*) => {};
    (emit TableSet $($rest:tt)*) => {};
    (emit TableGrow $($rest:tt)*) => {};
    (emit TableSize $($rest:tt)*) => {};
    (emit TableFill $($rest:tt)*) => {};
    (emit ElemDrop $($rest:tt)*) => {};
    (emit MemoryInit $($rest:tt)*) => {};
    (emit MemoryCopy $($rest:tt)*) => {};
    (emit DataDrop $($rest:tt)*) => {};
    (emit MemoryFill $($rest:tt)*) => {};
    (emit MemorySize $($rest:tt)*) => {};
    (emit MemoryGrow $($rest:tt)*) => {};
    (emit I32Load $($rest:tt)*) => {};
    (emit I32Load8S $($rest:tt)*) => {};
    (emit I32Load8U $($rest:tt)*) => {};
    (emit I32Load16S $($rest:tt)*) => {};
    (emit I32Load16U $($rest:tt)*) => {};
    (emit I64Load8S $($rest:tt)*) => {};
    (emit I64Load8U $($rest:tt)*) => {};
    (emit I64Load16S $($rest:tt)*) => {};
    (emit I64Load16U $($rest:tt)*) => {};
    (emit I64Load32S $($rest:tt)*) => {};
    (emit I64Load32U $($rest:tt)*) => {};
    (emit I64Load $($rest:tt)*) => {};
    (emit I32Store $($rest:tt)*) => {};
    (emit I32Store8 $($rest:tt)*) => {};
    (emit I32Store16 $($rest:tt)*) => {};
    (emit I64Store $($rest:tt)*) => {};
    (emit I64Store8 $($rest:tt)*) => {};
    (emit I64Store16 $($rest:tt)*) => {};
    (emit I64Store32 $($rest:tt)*) => {};
    (emit F32Load $($rest:tt)*) => {};
    (emit F32Store $($rest:tt)*) => {};
    (emit F64Load $($rest:tt)*) => {};
    (emit F64Store $($rest:tt)*) => {};
    (emit I32TruncSatF32S $($rest:tt)*) => {};
    (emit I32TruncSatF32U $($rest:tt)*) => {};
    (emit I32TruncSatF64S $($rest:tt)*) => {};
    (emit I32TruncSatF64U $($rest:tt)*) => {};
    (emit I64TruncSatF32S $($rest:tt)*) => {};
    (emit I64TruncSatF32U $($rest:tt)*) => {};
    (emit I64TruncSatF64S $($rest:tt)*) => {};
    (emit I64TruncSatF64U $($rest:tt)*) => {};
    (emit V128Load $($rest:tt)*) => {};
    (emit V128Store $($rest:tt)*) => {};
    (emit I64Add128 $($rest:tt)*) => {};
    (emit I64Sub128 $($rest:tt)*) => {};
    (emit I64MulWideS $($rest:tt)*) => {};
    (emit I64MulWideU $($rest:tt)*) => {};
    (emit I32AtomicLoad8U $($rest:tt)*) => {};
    (emit I32AtomicLoad16U $($rest:tt)*) => {};
    (emit I32AtomicLoad $($rest:tt)*) => {};
    (emit I64AtomicLoad8U $($rest:tt)*) => {};
    (emit I64AtomicLoad16U $($rest:tt)*) => {};
    (emit I64AtomicLoad32U $($rest:tt)*) => {};
    (emit I64AtomicLoad $($rest:tt)*) => {};
    (emit V128Load8x8S $($rest:tt)*) => {};
    (emit V128Load8x8U $($rest:tt)*) => {};
    (emit V128Load16x4S $($rest:tt)*) => {};
    (emit V128Load16x4U $($rest:tt)*) => {};
    (emit V128Load32x2S $($rest:tt)*) => {};
    (emit V128Load32x2U $($rest:tt)*) => {};
    (emit V128Load8Splat $($rest:tt)*) => {};
    (emit V128Load16Splat $($rest:tt)*) => {};
    (emit V128Load32Splat $($rest:tt)*) => {};
    (emit V128Load64Splat $($rest:tt)*) => {};
    (emit I8x16Splat $($rest:tt)*) => {};
    (emit I16x8Splat $($rest:tt)*) => {};
    (emit I32x4Splat $($rest:tt)*) => {};
    (emit I64x2Splat $($rest:tt)*) => {};
    (emit F32x4Splat $($rest:tt)*) => {};
    (emit F64x2Splat $($rest:tt)*) => {};
    (emit I32AtomicStore8 $($rest:tt)*) => {};
    (emit I32AtomicStore16 $($rest:tt)*) => {};
    (emit I32AtomicStore $($rest:tt)*) => {};
    (emit I64AtomicStore8 $($rest:tt)*) => {};
    (emit I64AtomicStore16 $($rest:tt)*) => {};
    (emit I64AtomicStore32 $($rest:tt)*) => {};
    (emit I64AtomicStore $($rest:tt)*) => {};
    (emit I32AtomicRmw8AddU $($rest:tt)*) => {};
    (emit I32AtomicRmw16AddU $($rest:tt)*) => {};
    (emit I32AtomicRmwAdd $($rest:tt)*) => {};
    (emit I64AtomicRmw8AddU $($rest:tt)*) => {};
    (emit I64AtomicRmw16AddU $($rest:tt)*) => {};
    (emit I64AtomicRmw32AddU $($rest:tt)*) => {};
    (emit I64AtomicRmwAdd $($rest:tt)*) => {};
    (emit I8x16Shuffle $($rest:tt)*) => {};
    (emit I8x16Swizzle $($rest:tt)*) => {};
    (emit I32AtomicRmw8SubU $($rest:tt)*) => {};
    (emit I32AtomicRmw16SubU $($rest:tt)*) => {};
    (emit I32AtomicRmwSub $($rest:tt)*) => {};
    (emit I64AtomicRmw8SubU $($rest:tt)*) => {};
    (emit I64AtomicRmw16SubU $($rest:tt)*) => {};
    (emit I64AtomicRmw32SubU $($rest:tt)*) => {};
    (emit I64AtomicRmwSub $($rest:tt)*) => {};
    (emit I32AtomicRmw8XchgU $($rest:tt)*) => {};
    (emit I32AtomicRmw16XchgU $($rest:tt)*) => {};
    (emit I32AtomicRmwXchg $($rest:tt)*) => {};
    (emit I64AtomicRmw8XchgU $($rest:tt)*) => {};
    (emit I64AtomicRmw16XchgU $($rest:tt)*) => {};
    (emit I64AtomicRmw32XchgU $($rest:tt)*) => {};
    (emit I64AtomicRmwXchg $($rest:tt)*) => {};
    (emit I8x16ExtractLaneS $($rest:tt)*) => {};
    (emit I8x16ExtractLaneU $($rest:tt)*) => {};
    (emit I16x8ExtractLaneS $($rest:tt)*) => {};
    (emit I16x8ExtractLaneU $($rest:tt)*) => {};
    (emit I32x4ExtractLane $($rest:tt)*) => {};
    (emit I64x2ExtractLane $($rest:tt)*) => {};
    (emit F32x4ExtractLane $($rest:tt)*) => {};
    (emit F64x2ExtractLane $($rest:tt)*) => {};
    (emit I32AtomicRmw8AndU $($rest:tt)*) => {};
    (emit I32AtomicRmw16AndU $($rest:tt)*) => {};
    (emit I32AtomicRmwAnd $($rest:tt)*) => {};
    (emit I64AtomicRmw8AndU $($rest:tt)*) => {};
    (emit I64AtomicRmw16AndU $($rest:tt)*) => {};
    (emit I64AtomicRmw32AndU $($rest:tt)*) => {};
    (emit I64AtomicRmwAnd $($rest:tt)*) => {};
    (emit I32AtomicRmw8OrU $($rest:tt)*) => {};
    (emit I32AtomicRmw16OrU $($rest:tt)*) => {};
    (emit I32AtomicRmwOr $($rest:tt)*) => {};
    (emit I64AtomicRmw8OrU $($rest:tt)*) => {};
    (emit I64AtomicRmw16OrU $($rest:tt)*) => {};
    (emit I64AtomicRmw32OrU $($rest:tt)*) => {};
    (emit I64AtomicRmwOr $($rest:tt)*) => {};
    (emit I32AtomicRmw8XorU $($rest:tt)*) => {};
    (emit I32AtomicRmw16XorU $($rest:tt)*) => {};
    (emit I32AtomicRmwXor $($rest:tt)*) => {};
    (emit I64AtomicRmw8XorU $($rest:tt)*) => {};
    (emit I64AtomicRmw16XorU $($rest:tt)*) => {};
    (emit I64AtomicRmw32XorU $($rest:tt)*) => {};
    (emit I64AtomicRmwXor $($rest:tt)*) => {};
    (emit I8x16ReplaceLane $($rest:tt)*) => {};
    (emit I16x8ReplaceLane $($rest:tt)*) => {};
    (emit I32x4ReplaceLane $($rest:tt)*) => {};
    (emit I64x2ReplaceLane $($rest:tt)*) => {};
    (emit F32x4ReplaceLane $($rest:tt)*) => {};
    (emit F64x2ReplaceLane $($rest:tt)*) => {};
    (emit I32AtomicRmw8CmpxchgU $($rest:tt)*) => {};
    (emit I32AtomicRmw16CmpxchgU $($rest:tt)*) => {};
    (emit I32AtomicRmwCmpxchg $($rest:tt)*) => {};
    (emit I64AtomicRmw8CmpxchgU $($rest:tt)*) => {};
    (emit I64AtomicRmw16CmpxchgU $($rest:tt)*) => {};
    (emit I64AtomicRmw32CmpxchgU $($rest:tt)*) => {};
    (emit I64AtomicRmwCmpxchg $($rest:tt)*) => {};
    (emit I8x16Eq $($rest:tt)*) => {};
    (emit I16x8Eq $($rest:tt)*) => {};
    (emit I32x4Eq $($rest:tt)*) => {};
    (emit I64x2Eq $($rest:tt)*) => {};
    (emit F32x4Eq $($rest:tt)*) => {};
    (emit F64x2Eq $($rest:tt)*) => {};
    (emit I8x16Ne $($rest:tt)*) => {};
    (emit I16x8Ne $($rest:tt)*) => {};
    (emit I32x4Ne $($rest:tt)*) => {};
    (emit I64x2Ne $($rest:tt)*) => {};
    (emit F32x4Ne $($rest:tt)*) => {};
    (emit F64x2Ne $($rest:tt)*) => {};
    (emit I8x16LtS $($rest:tt)*) => {};
    (emit I8x16LtU $($rest:tt)*) => {};
    (emit I16x8LtS $($rest:tt)*) => {};
    (emit I16x8LtU $($rest:tt)*) => {};
    (emit I32x4LtS $($rest:tt)*) => {};
    (emit I32x4LtU $($rest:tt)*) => {};
    (emit I64x2LtS $($rest:tt)*) => {};
    (emit F32x4Lt $($rest:tt)*) => {};
    (emit F64x2Lt $($rest:tt)*) => {};
    (emit I8x16LeS $($rest:tt)*) => {};
    (emit I8x16LeU $($rest:tt)*) => {};
    (emit I16x8LeS $($rest:tt)*) => {};
    (emit I16x8LeU $($rest:tt)*) => {};
    (emit I32x4LeS $($rest:tt)*) => {};
    (emit I32x4LeU $($rest:tt)*) => {};
    (emit I64x2LeS $($rest:tt)*) => {};
    (emit F32x4Le $($rest:tt)*) => {};
    (emit F64x2Le $($rest:tt)*) => {};
    (emit I8x16GtS $($rest:tt)*) => {};
    (emit I8x16GtU $($rest:tt)*) => {};
    (emit I16x8GtS $($rest:tt)*) => {};
    (emit I16x8GtU $($rest:tt)*) => {};
    (emit I32x4GtS $($rest:tt)*) => {};
    (emit I32x4GtU $($rest:tt)*) => {};
    (emit I64x2GtS $($rest:tt)*) => {};
    (emit F32x4Gt $($rest:tt)*) => {};
    (emit F64x2Gt $($rest:tt)*) => {};
    (emit I8x16GeS $($rest:tt)*) => {};
    (emit I8x16GeU $($rest:tt)*) => {};
    (emit I16x8GeS $($rest:tt)*) => {};
    (emit I16x8GeU $($rest:tt)*) => {};
    (emit I32x4GeS $($rest:tt)*) => {};
    (emit I32x4GeU $($rest:tt)*) => {};
    (emit I64x2GeS $($rest:tt)*) => {};
    (emit F32x4Ge $($rest:tt)*) => {};
    (emit F64x2Ge $($rest:tt)*) => {};
    (emit MemoryAtomicWait32 $($rest:tt)*) => {};
    (emit MemoryAtomicWait64 $($rest:tt)*) => {};
    (emit MemoryAtomicNotify $($rest:tt)*) => {};
    (emit AtomicFence $($rest:tt)*) => {};
    (emit V128Not $($rest:tt)*) => {};
    (emit V128And $($rest:tt)*) => {};
    (emit V128AndNot $($rest:tt)*) => {};
    (emit V128Or $($rest:tt)*) => {};
    (emit V128Xor $($rest:tt)*) => {};
    (emit V128Bitselect $($rest:tt)*) => {};
    (emit V128AnyTrue $($rest:tt)*) => {};
    (emit V128Load8Lane $($rest:tt)*) => {};
    (emit V128Load16Lane $($rest:tt)*) => {};
    (emit V128Load32Lane $($rest:tt)*) => {};
    (emit V128Load64Lane $($rest:tt)*) => {};
    (emit V128Store8Lane $($rest:tt)*) => {};
    (emit V128Store16Lane $($rest:tt)*) => {};
    (emit V128Store32Lane $($rest:tt)*) => {};
    (emit V128Store64Lane $($rest:tt)*) => {};
    (emit F32x4ConvertI32x4S $($rest:tt)*) => {};
    (emit F32x4ConvertI32x4U $($rest:tt)*) => {};
    (emit F64x2ConvertLowI32x4S $($rest:tt)*) => {};
    (emit F64x2ConvertLowI32x4U $($rest:tt)*) => {};
    (emit I8x16NarrowI16x8S $($rest:tt)*) => {};
    (emit I8x16NarrowI16x8U $($rest:tt)*) => {};
    (emit I16x8NarrowI32x4S $($rest:tt)*) => {};
    (emit I16x8NarrowI32x4U $($rest:tt)*) => {};
    (emit F32x4DemoteF64x2Zero $($rest:tt)*) => {};
    (emit F64x2PromoteLowF32x4 $($rest:tt)*) => {};
    (emit I16x8ExtendLowI8x16S $($rest:tt)*) => {};
    (emit I16x8ExtendHighI8x16S $($rest:tt)*) => {};
    (emit I16x8ExtendLowI8x16U $($rest:tt)*) => {};
    (emit I16x8ExtendHighI8x16U $($rest:tt)*) => {};
    (emit I32x4ExtendLowI16x8S $($rest:tt)*) => {};
    (emit I32x4ExtendHighI16x8S $($rest:tt)*) => {};
    (emit I32x4ExtendLowI16x8U $($rest:tt)*) => {};
    (emit I32x4ExtendHighI16x8U $($rest:tt)*) => {};
    (emit I64x2ExtendLowI32x4S $($rest:tt)*) => {};
    (emit I64x2ExtendHighI32x4S $($rest:tt)*) => {};
    (emit I64x2ExtendLowI32x4U $($rest:tt)*) => {};
    (emit I64x2ExtendHighI32x4U $($rest:tt)*) => {};
    (emit I8x16Add $($rest:tt)*) => {};
    (emit I16x8Add $($rest:tt)*) => {};
    (emit I32x4Add $($rest:tt)*) => {};
    (emit I64x2Add $($rest:tt)*) => {};
    (emit I8x16Sub $($rest:tt)*) => {};
    (emit I16x8Sub $($rest:tt)*) => {};
    (emit I32x4Sub $($rest:tt)*) => {};
    (emit I64x2Sub $($rest:tt)*) => {};
    (emit I16x8Mul $($rest:tt)*) => {};
    (emit I32x4Mul $($rest:tt)*) => {};
    (emit I64x2Mul $($rest:tt)*) => {};
    (emit I8x16AddSatS $($rest:tt)*) => {};
    (emit I16x8AddSatS $($rest:tt)*) => {};
    (emit I8x16AddSatU $($rest:tt)*) => {};
    (emit I16x8AddSatU $($rest:tt)*) => {};
    (emit I8x16SubSatS $($rest:tt)*) => {};
    (emit I16x8SubSatS $($rest:tt)*) => {};
    (emit I8x16SubSatU $($rest:tt)*) => {};
    (emit I16x8SubSatU $($rest:tt)*) => {};
    (emit I8x16Abs $($rest:tt)*) => {};
    (emit I16x8Abs $($rest:tt)*) => {};
    (emit I32x4Abs $($rest:tt)*) => {};
    (emit I64x2Abs $($rest:tt)*) => {};
    (emit F32x4Abs $($rest:tt)*) => {};
    (emit F64x2Abs $($rest:tt)*) => {};
    (emit I8x16Neg $($rest:tt)*) => {};
    (emit I16x8Neg $($rest:tt)*) => {};
    (emit I32x4Neg $($rest:tt)*) => {};
    (emit I64x2Neg $($rest:tt)*) => {};
    (emit I8x16Shl $($rest:tt)*) => {};
    (emit I16x8Shl $($rest:tt)*) => {};
    (emit I32x4Shl $($rest:tt)*) => {};
    (emit I64x2Shl $($rest:tt)*) => {};
    (emit I8x16ShrU $($rest:tt)*) => {};
    (emit I16x8ShrU $($rest:tt)*) => {};
    (emit I32x4ShrU $($rest:tt)*) => {};
    (emit I64x2ShrU $($rest:tt)*) => {};
    (emit I8x16ShrS $($rest:tt)*) => {};
    (emit I16x8ShrS $($rest:tt)*) => {};
    (emit I32x4ShrS $($rest:tt)*) => {};
    (emit I64x2ShrS $($rest:tt)*) => {};
    (emit I16x8Q15MulrSatS $($rest:tt)*) => {};
    (emit I8x16AllTrue $($rest:tt)*) => {};
    (emit I16x8AllTrue $($rest:tt)*) => {};
    (emit I32x4AllTrue $($rest:tt)*) => {};
    (emit I64x2AllTrue $($rest:tt)*) => {};
    (emit I8x16Bitmask $($rest:tt)*) => {};
    (emit I16x8Bitmask $($rest:tt)*) => {};
    (emit I32x4Bitmask $($rest:tt)*) => {};
    (emit I64x2Bitmask $($rest:tt)*) => {};
    (emit I32x4TruncSatF32x4S $($rest:tt)*) => {};
    (emit I32x4TruncSatF32x4U $($rest:tt)*) => {};
    (emit I32x4TruncSatF64x2SZero $($rest:tt)*) => {};
    (emit I32x4TruncSatF64x2UZero $($rest:tt)*) => {};
    (emit I8x16MinU $($rest:tt)*) => {};
    (emit I16x8MinU $($rest:tt)*) => {};
    (emit I32x4MinU $($rest:tt)*) => {};
    (emit I8x16MinS $($rest:tt)*) => {};
    (emit I16x8MinS $($rest:tt)*) => {};
    (emit I32x4MinS $($rest:tt)*) => {};
    (emit I8x16MaxU $($rest:tt)*) => {};
    (emit I16x8MaxU $($rest:tt)*) => {};
    (emit I32x4MaxU $($rest:tt)*) => {};
    (emit I8x16MaxS $($rest:tt)*) => {};
    (emit I16x8MaxS $($rest:tt)*) => {};
    (emit I32x4MaxS $($rest:tt)*) => {};
    (emit I16x8ExtMulLowI8x16S $($rest:tt)*) => {};
    (emit I32x4ExtMulLowI16x8S $($rest:tt)*) => {};
    (emit I64x2ExtMulLowI32x4S $($rest:tt)*) => {};
    (emit I16x8ExtMulHighI8x16S $($rest:tt)*) => {};
    (emit I32x4ExtMulHighI16x8S $($rest:tt)*) => {};
    (emit I64x2ExtMulHighI32x4S $($rest:tt)*) => {};
    (emit I16x8ExtMulLowI8x16U $($rest:tt)*) => {};
    (emit I32x4ExtMulLowI16x8U $($rest:tt)*) => {};
    (emit I64x2ExtMulLowI32x4U $($rest:tt)*) => {};
    (emit I16x8ExtMulHighI8x16U $($rest:tt)*) => {};
    (emit I32x4ExtMulHighI16x8U $($rest:tt)*) => {};
    (emit I64x2ExtMulHighI32x4U $($rest:tt)*) => {};
    (emit I16x8ExtAddPairwiseI8x16U $($rest:tt)*) => {};
    (emit I16x8ExtAddPairwiseI8x16S $($rest:tt)*) => {};
    (emit I32x4ExtAddPairwiseI16x8U $($rest:tt)*) => {};
    (emit I32x4ExtAddPairwiseI16x8S $($rest:tt)*) => {};
    (emit I32x4DotI16x8S $($rest:tt)*) => {};
    (emit I8x16Popcnt $($rest:tt)*) => {};
    (emit I8x16AvgrU $($rest:tt)*) => {};
    (emit I16x8AvgrU $($rest:tt)*) => {};
    (emit F32x4Add $($rest:tt)*) => {};
    (emit F64x2Add $($rest:tt)*) => {};
    (emit F32x4Sub $($rest:tt)*) => {};
    (emit F64x2Sub $($rest:tt)*) => {};
    (emit F32x4Mul $($rest:tt)*) => {};
    (emit F64x2Mul $($rest:tt)*) => {};
    (emit F32x4Div $($rest:tt)*) => {};
    (emit F64x2Div $($rest:tt)*) => {};
    (emit F32x4Neg $($rest:tt)*) => {};
    (emit F64x2Neg $($rest:tt)*) => {};
    (emit F32x4Sqrt $($rest:tt)*) => {};
    (emit F64x2Sqrt $($rest:tt)*) => {};
    (emit F32x4Ceil $($rest:tt)*) => {};
    (emit F64x2Ceil $($rest:tt)*) => {};
    (emit F32x4Floor $($rest:tt)*) => {};
    (emit F64x2Floor $($rest:tt)*) => {};
    (emit F32x4Nearest $($rest:tt)*) => {};
    (emit F64x2Nearest $($rest:tt)*) => {};
    (emit F32x4Trunc $($rest:tt)*) => {};
    (emit F64x2Trunc $($rest:tt)*) => {};
    (emit V128Load32Zero $($rest:tt)*) => {};
    (emit V128Load64Zero $($rest:tt)*) => {};
    (emit F32x4PMin $($rest:tt)*) => {};
    (emit F64x2PMin $($rest:tt)*) => {};
    (emit F32x4PMax $($rest:tt)*) => {};
    (emit F64x2PMax $($rest:tt)*) => {};
    (emit F32x4Min $($rest:tt)*) => {};
    (emit F64x2Min $($rest:tt)*) => {};
    (emit F32x4Max $($rest:tt)*) => {};
    (emit F64x2Max $($rest:tt)*) => {};

    (emit $unsupported:tt $($rest:tt)*) => {$($rest)*};
}

impl<'a, 'translation, 'data, M> VisitOperator<'a> for CodeGen<'a, 'translation, 'data, M, Emission>
where
    M: MacroAssembler,
{
    type Output = Result<()>;

    fn visit_i32_const(&mut self, val: i32) -> Self::Output {
        self.context.stack.push(Val::i32(val));

        Ok(())
    }

    fn visit_i64_const(&mut self, val: i64) -> Self::Output {
        self.context.stack.push(Val::i64(val));
        Ok(())
    }

    fn visit_f32_const(&mut self, val: Ieee32) -> Self::Output {
        self.context.stack.push(Val::f32(val));
        Ok(())
    }

    fn visit_f64_const(&mut self, val: Ieee64) -> Self::Output {
        self.context.stack.push(Val::f64(val));
        Ok(())
    }

    fn visit_f32_add(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_add(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f32(dst))
            },
        )
    }

    fn visit_f64_add(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_add(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f64(dst))
            },
        )
    }

    fn visit_f32_sub(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_sub(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f32(dst))
            },
        )
    }

    fn visit_f64_sub(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_sub(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f64(dst))
            },
        )
    }

    fn visit_f32_mul(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_mul(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f32(dst))
            },
        )
    }

    fn visit_f64_mul(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_mul(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f64(dst))
            },
        )
    }

    fn visit_f32_div(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_div(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f32(dst))
            },
        )
    }

    fn visit_f64_div(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_div(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f64(dst))
            },
        )
    }

    fn visit_f32_min(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_min(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f32(dst))
            },
        )
    }

    fn visit_f64_min(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_min(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f64(dst))
            },
        )
    }

    fn visit_f32_max(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_max(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f32(dst))
            },
        )
    }

    fn visit_f64_max(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_max(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f64(dst))
            },
        )
    }

    fn visit_f32_copysign(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_copysign(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f32(dst))
            },
        )
    }

    fn visit_f64_copysign(&mut self) -> Self::Output {
        self.context.binop(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src, size| {
                masm.float_copysign(writable!(dst), dst, src, size)?;
                Ok(TypedReg::f64(dst))
            },
        )
    }

    fn visit_f32_abs(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.float_abs(writable!(reg), OperandSize::S32)?;
            Ok(TypedReg::f32(reg))
        })
    }

    fn visit_f64_abs(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.float_abs(writable!(reg), OperandSize::S64)?;
            Ok(TypedReg::f64(reg))
        })
    }

    fn visit_f32_neg(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.float_neg(writable!(reg), OperandSize::S32)?;
            Ok(TypedReg::f32(reg))
        })
    }

    fn visit_f64_neg(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.float_neg(writable!(reg), OperandSize::S64)?;
            Ok(TypedReg::f64(reg))
        })
    }

    fn visit_f32_floor(&mut self) -> Self::Output {
        self.masm.float_round(
            RoundingMode::Down,
            &mut self.env,
            &mut self.context,
            OperandSize::S32,
            |env, cx, masm| {
                let builtin = env.builtins.floor_f32::<M::ABI, M::Ptr>()?;
                FnCall::emit::<M>(env, masm, cx, Callee::Builtin(builtin))
            },
        )
    }

    fn visit_f64_floor(&mut self) -> Self::Output {
        self.masm.float_round(
            RoundingMode::Down,
            &mut self.env,
            &mut self.context,
            OperandSize::S64,
            |env, cx, masm| {
                let builtin = env.builtins.floor_f64::<M::ABI, M::Ptr>()?;
                FnCall::emit::<M>(env, masm, cx, Callee::Builtin(builtin))
            },
        )
    }

    fn visit_f32_ceil(&mut self) -> Self::Output {
        self.masm.float_round(
            RoundingMode::Up,
            &mut self.env,
            &mut self.context,
            OperandSize::S32,
            |env, cx, masm| {
                let builtin = env.builtins.ceil_f32::<M::ABI, M::Ptr>()?;
                FnCall::emit::<M>(env, masm, cx, Callee::Builtin(builtin))
            },
        )
    }

    fn visit_f64_ceil(&mut self) -> Self::Output {
        self.masm.float_round(
            RoundingMode::Up,
            &mut self.env,
            &mut self.context,
            OperandSize::S64,
            |env, cx, masm| {
                let builtin = env.builtins.ceil_f64::<M::ABI, M::Ptr>()?;
                FnCall::emit::<M>(env, masm, cx, Callee::Builtin(builtin))
            },
        )
    }

    fn visit_f32_nearest(&mut self) -> Self::Output {
        self.masm.float_round(
            RoundingMode::Nearest,
            &mut self.env,
            &mut self.context,
            OperandSize::S32,
            |env, cx, masm| {
                let builtin = env.builtins.nearest_f32::<M::ABI, M::Ptr>()?;
                FnCall::emit::<M>(env, masm, cx, Callee::Builtin(builtin))
            },
        )
    }

    fn visit_f64_nearest(&mut self) -> Self::Output {
        self.masm.float_round(
            RoundingMode::Nearest,
            &mut self.env,
            &mut self.context,
            OperandSize::S64,
            |env, cx, masm| {
                let builtin = env.builtins.nearest_f64::<M::ABI, M::Ptr>()?;
                FnCall::emit::<M>(env, masm, cx, Callee::Builtin(builtin))
            },
        )
    }

    fn visit_f32_trunc(&mut self) -> Self::Output {
        self.masm.float_round(
            RoundingMode::Zero,
            &mut self.env,
            &mut self.context,
            OperandSize::S32,
            |env, cx, masm| {
                let builtin = env.builtins.trunc_f32::<M::ABI, M::Ptr>()?;
                FnCall::emit::<M>(env, masm, cx, Callee::Builtin(builtin))
            },
        )
    }

    fn visit_f64_trunc(&mut self) -> Self::Output {
        self.masm.float_round(
            RoundingMode::Zero,
            &mut self.env,
            &mut self.context,
            OperandSize::S64,
            |env, cx, masm| {
                let builtin = env.builtins.trunc_f64::<M::ABI, M::Ptr>()?;
                FnCall::emit::<M>(env, masm, cx, Callee::Builtin(builtin))
            },
        )
    }

    fn visit_f32_sqrt(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.float_sqrt(writable!(reg), reg, OperandSize::S32)?;
            Ok(TypedReg::f32(reg))
        })
    }

    fn visit_f64_sqrt(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.float_sqrt(writable!(reg), reg, OperandSize::S64)?;
            Ok(TypedReg::f64(reg))
        })
    }

    fn visit_f32_eq(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Eq, size)
            },
        )
    }

    fn visit_f64_eq(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Eq, size)
            },
        )
    }

    fn visit_f32_ne(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Ne, size)
            },
        )
    }

    fn visit_f64_ne(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Ne, size)
            },
        )
    }

    fn visit_f32_lt(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Lt, size)
            },
        )
    }

    fn visit_f64_lt(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Lt, size)
            },
        )
    }

    fn visit_f32_gt(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Gt, size)
            },
        )
    }

    fn visit_f64_gt(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Gt, size)
            },
        )
    }

    fn visit_f32_le(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Le, size)
            },
        )
    }

    fn visit_f64_le(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Le, size)
            },
        )
    }

    fn visit_f32_ge(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S32,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Ge, size)
            },
        )
    }

    fn visit_f64_ge(&mut self) -> Self::Output {
        self.context.float_cmp_op(
            self.masm,
            OperandSize::S64,
            &mut |masm: &mut M, dst, src1, src2, size| {
                masm.float_cmp_with_set(writable!(dst), src1, src2, FloatCmpKind::Ge, size)
            },
        )
    }

    fn visit_f32_convert_i32_s(&mut self) -> Self::Output {
        self.context
            .convert_op(self.masm, WasmValType::F32, |masm, dst, src, dst_size| {
                masm.signed_convert(writable!(dst), src, OperandSize::S32, dst_size)
            })
    }

    fn visit_f32_convert_i32_u(&mut self) -> Self::Output {
        self.context.convert_op_with_tmp_reg(
            self.masm,
            WasmValType::F32,
            RegClass::Int,
            |masm, dst, src, tmp_gpr, dst_size| {
                masm.unsigned_convert(writable!(dst), src, tmp_gpr, OperandSize::S32, dst_size)
            },
        )
    }

    fn visit_f32_convert_i64_s(&mut self) -> Self::Output {
        self.context
            .convert_op(self.masm, WasmValType::F32, |masm, dst, src, dst_size| {
                masm.signed_convert(writable!(dst), src, OperandSize::S64, dst_size)
            })
    }

    fn visit_f32_convert_i64_u(&mut self) -> Self::Output {
        self.context.convert_op_with_tmp_reg(
            self.masm,
            WasmValType::F32,
            RegClass::Int,
            |masm, dst, src, tmp_gpr, dst_size| {
                masm.unsigned_convert(writable!(dst), src, tmp_gpr, OperandSize::S64, dst_size)
            },
        )
    }

    fn visit_f64_convert_i32_s(&mut self) -> Self::Output {
        self.context
            .convert_op(self.masm, WasmValType::F64, |masm, dst, src, dst_size| {
                masm.signed_convert(writable!(dst), src, OperandSize::S32, dst_size)
            })
    }

    fn visit_f64_convert_i32_u(&mut self) -> Self::Output {
        self.context.convert_op_with_tmp_reg(
            self.masm,
            WasmValType::F64,
            RegClass::Int,
            |masm, dst, src, tmp_gpr, dst_size| {
                masm.unsigned_convert(writable!(dst), src, tmp_gpr, OperandSize::S32, dst_size)
            },
        )
    }

    fn visit_f64_convert_i64_s(&mut self) -> Self::Output {
        self.context
            .convert_op(self.masm, WasmValType::F64, |masm, dst, src, dst_size| {
                masm.signed_convert(writable!(dst), src, OperandSize::S64, dst_size)
            })
    }

    fn visit_f64_convert_i64_u(&mut self) -> Self::Output {
        self.context.convert_op_with_tmp_reg(
            self.masm,
            WasmValType::F64,
            RegClass::Int,
            |masm, dst, src, tmp_gpr, dst_size| {
                masm.unsigned_convert(writable!(dst), src, tmp_gpr, OperandSize::S64, dst_size)
            },
        )
    }

    fn visit_f32_reinterpret_i32(&mut self) -> Self::Output {
        self.context
            .convert_op(self.masm, WasmValType::F32, |masm, dst, src, size| {
                masm.reinterpret_int_as_float(writable!(dst), src, size)
            })
    }

    fn visit_f64_reinterpret_i64(&mut self) -> Self::Output {
        self.context
            .convert_op(self.masm, WasmValType::F64, |masm, dst, src, size| {
                masm.reinterpret_int_as_float(writable!(dst), src, size)
            })
    }

    fn visit_f32_demote_f64(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.demote(writable!(reg), reg)?;
            Ok(TypedReg::f32(reg))
        })
    }

    fn visit_f64_promote_f32(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.promote(writable!(reg), reg)?;
            Ok(TypedReg::f64(reg))
        })
    }

    fn visit_i32_add(&mut self) -> Self::Output {
        self.context.i32_binop(self.masm, |masm, dst, src, size| {
            masm.add(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i32(dst))
        })
    }

    fn visit_i64_add(&mut self) -> Self::Output {
        self.context.i64_binop(self.masm, |masm, dst, src, size| {
            masm.add(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i64(dst))
        })
    }

    fn visit_i32_sub(&mut self) -> Self::Output {
        self.context.i32_binop(self.masm, |masm, dst, src, size| {
            masm.sub(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i32(dst))
        })
    }

    fn visit_i64_sub(&mut self) -> Self::Output {
        self.context.i64_binop(self.masm, |masm, dst, src, size| {
            masm.sub(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i64(dst))
        })
    }

    fn visit_i32_mul(&mut self) -> Self::Output {
        self.context.i32_binop(self.masm, |masm, dst, src, size| {
            masm.mul(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i32(dst))
        })
    }

    fn visit_i64_mul(&mut self) -> Self::Output {
        self.context.i64_binop(self.masm, |masm, dst, src, size| {
            masm.mul(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i64(dst))
        })
    }

    fn visit_i32_div_s(&mut self) -> Self::Output {
        use DivKind::*;
        use OperandSize::*;

        self.masm.div(&mut self.context, Signed, S32)
    }

    fn visit_i32_div_u(&mut self) -> Self::Output {
        use DivKind::*;
        use OperandSize::*;

        self.masm.div(&mut self.context, Unsigned, S32)
    }

    fn visit_i64_div_s(&mut self) -> Self::Output {
        use DivKind::*;
        use OperandSize::*;

        self.masm.div(&mut self.context, Signed, S64)
    }

    fn visit_i64_div_u(&mut self) -> Self::Output {
        use DivKind::*;
        use OperandSize::*;

        self.masm.div(&mut self.context, Unsigned, S64)
    }

    fn visit_i32_rem_s(&mut self) -> Self::Output {
        use OperandSize::*;
        use RemKind::*;

        self.masm.rem(&mut self.context, Signed, S32)
    }

    fn visit_i32_rem_u(&mut self) -> Self::Output {
        use OperandSize::*;
        use RemKind::*;

        self.masm.rem(&mut self.context, Unsigned, S32)
    }

    fn visit_i64_rem_s(&mut self) -> Self::Output {
        use OperandSize::*;
        use RemKind::*;

        self.masm.rem(&mut self.context, Signed, S64)
    }

    fn visit_i64_rem_u(&mut self) -> Self::Output {
        use OperandSize::*;
        use RemKind::*;

        self.masm.rem(&mut self.context, Unsigned, S64)
    }

    fn visit_i32_eq(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::Eq)
    }

    fn visit_i64_eq(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::Eq)
    }

    fn visit_i32_ne(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::Ne)
    }

    fn visit_i64_ne(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::Ne)
    }

    fn visit_i32_lt_s(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::LtS)
    }

    fn visit_i64_lt_s(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::LtS)
    }

    fn visit_i32_lt_u(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::LtU)
    }

    fn visit_i64_lt_u(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::LtU)
    }

    fn visit_i32_le_s(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::LeS)
    }

    fn visit_i64_le_s(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::LeS)
    }

    fn visit_i32_le_u(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::LeU)
    }

    fn visit_i64_le_u(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::LeU)
    }

    fn visit_i32_gt_s(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::GtS)
    }

    fn visit_i64_gt_s(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::GtS)
    }

    fn visit_i32_gt_u(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::GtU)
    }

    fn visit_i64_gt_u(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::GtU)
    }

    fn visit_i32_ge_s(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::GeS)
    }

    fn visit_i64_ge_s(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::GeS)
    }

    fn visit_i32_ge_u(&mut self) -> Self::Output {
        self.cmp_i32s(IntCmpKind::GeU)
    }

    fn visit_i64_ge_u(&mut self) -> Self::Output {
        self.cmp_i64s(IntCmpKind::GeU)
    }

    fn visit_i32_eqz(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context.unop(self.masm, |masm, reg| {
            masm.cmp_with_set(writable!(reg), RegImm::i32(0), IntCmpKind::Eq, S32)?;
            Ok(TypedReg::i32(reg))
        })
    }

    fn visit_i64_eqz(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context.unop(self.masm, |masm, reg| {
            masm.cmp_with_set(writable!(reg), RegImm::i64(0), IntCmpKind::Eq, S64)?;
            Ok(TypedReg::i32(reg)) // Return value for `i64.eqz` is an `i32`.
        })
    }

    fn visit_i32_clz(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context.unop(self.masm, |masm, reg| {
            masm.clz(writable!(reg), reg, S32)?;
            Ok(TypedReg::i32(reg))
        })
    }

    fn visit_i64_clz(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context.unop(self.masm, |masm, reg| {
            masm.clz(writable!(reg), reg, S64)?;
            Ok(TypedReg::i64(reg))
        })
    }

    fn visit_i32_ctz(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context.unop(self.masm, |masm, reg| {
            masm.ctz(writable!(reg), reg, S32)?;
            Ok(TypedReg::i32(reg))
        })
    }

    fn visit_i64_ctz(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context.unop(self.masm, |masm, reg| {
            masm.ctz(writable!(reg), reg, S64)?;
            Ok(TypedReg::i64(reg))
        })
    }

    fn visit_i32_and(&mut self) -> Self::Output {
        self.context.i32_binop(self.masm, |masm, dst, src, size| {
            masm.and(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i32(dst))
        })
    }

    fn visit_i64_and(&mut self) -> Self::Output {
        self.context.i64_binop(self.masm, |masm, dst, src, size| {
            masm.and(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i64(dst))
        })
    }

    fn visit_i32_or(&mut self) -> Self::Output {
        self.context.i32_binop(self.masm, |masm, dst, src, size| {
            masm.or(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i32(dst))
        })
    }

    fn visit_i64_or(&mut self) -> Self::Output {
        self.context.i64_binop(self.masm, |masm, dst, src, size| {
            masm.or(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i64(dst))
        })
    }

    fn visit_i32_xor(&mut self) -> Self::Output {
        self.context.i32_binop(self.masm, |masm, dst, src, size| {
            masm.xor(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i32(dst))
        })
    }

    fn visit_i64_xor(&mut self) -> Self::Output {
        self.context.i64_binop(self.masm, |masm, dst, src, size| {
            masm.xor(writable!(dst), dst, src, size)?;
            Ok(TypedReg::i64(dst))
        })
    }

    fn visit_i32_shl(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i32_shift(self.masm, Shl)
    }

    fn visit_i64_shl(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i64_shift(self.masm, Shl)
    }

    fn visit_i32_shr_s(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i32_shift(self.masm, ShrS)
    }

    fn visit_i64_shr_s(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i64_shift(self.masm, ShrS)
    }

    fn visit_i32_shr_u(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i32_shift(self.masm, ShrU)
    }

    fn visit_i64_shr_u(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i64_shift(self.masm, ShrU)
    }

    fn visit_i32_rotl(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i32_shift(self.masm, Rotl)
    }

    fn visit_i64_rotl(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i64_shift(self.masm, Rotl)
    }

    fn visit_i32_rotr(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i32_shift(self.masm, Rotr)
    }

    fn visit_i64_rotr(&mut self) -> Self::Output {
        use ShiftKind::*;

        self.context.i64_shift(self.masm, Rotr)
    }

    fn visit_end(&mut self) -> Self::Output {
        if !self.context.reachable {
            self.handle_unreachable_end()
        } else {
            let mut control = self.pop_control_frame()?;
            control.emit_end(self.masm, &mut self.context)
        }
    }

    fn visit_i32_popcnt(&mut self) -> Self::Output {
        use OperandSize::*;
        self.masm.popcnt(&mut self.context, S32)
    }

    fn visit_i64_popcnt(&mut self) -> Self::Output {
        use OperandSize::*;

        self.masm.popcnt(&mut self.context, S64)
    }

    fn visit_i32_wrap_i64(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.wrap(writable!(reg), reg)?;
            Ok(TypedReg::i32(reg))
        })
    }

    fn visit_i64_extend_i32_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.extend(writable!(reg), reg, Extend::<Signed>::I64Extend32.into())?;
            Ok(TypedReg::i64(reg))
        })
    }

    fn visit_i64_extend_i32_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.extend(writable!(reg), reg, Extend::<Zero>::I64Extend32.into())?;
            Ok(TypedReg::i64(reg))
        })
    }

    fn visit_i32_extend8_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.extend(writable!(reg), reg, Extend::<Signed>::I32Extend8.into())?;
            Ok(TypedReg::i32(reg))
        })
    }

    fn visit_i32_extend16_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.extend(writable!(reg), reg, Extend::<Signed>::I32Extend16.into())?;
            Ok(TypedReg::i32(reg))
        })
    }

    fn visit_i64_extend8_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.extend(writable!(reg), reg, Extend::<Signed>::I64Extend8.into())?;
            Ok(TypedReg::i64(reg))
        })
    }

    fn visit_i64_extend16_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.extend(writable!(reg), reg, Extend::<Signed>::I64Extend16.into())?;
            Ok(TypedReg::i64(reg))
        })
    }

    fn visit_i64_extend32_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.extend(writable!(reg), reg, Extend::<Signed>::I64Extend32.into())?;
            Ok(TypedReg::i64(reg))
        })
    }

    fn visit_i32_trunc_f32_s(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context
            .convert_op(self.masm, WasmValType::I32, |masm, dst, src, dst_size| {
                masm.signed_truncate(writable!(dst), src, S32, dst_size, TruncKind::Unchecked)
            })
    }

    fn visit_i32_trunc_f32_u(&mut self) -> Self::Output {
        use OperandSize::*;

        self.masm
            .unsigned_truncate(&mut self.context, S32, S32, TruncKind::Unchecked)
    }

    fn visit_i32_trunc_f64_s(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context
            .convert_op(self.masm, WasmValType::I32, |masm, dst, src, dst_size| {
                masm.signed_truncate(writable!(dst), src, S64, dst_size, TruncKind::Unchecked)
            })
    }

    fn visit_i32_trunc_f64_u(&mut self) -> Self::Output {
        use OperandSize::*;
        self.masm
            .unsigned_truncate(&mut self.context, S64, S32, TruncKind::Unchecked)
    }

    fn visit_i64_trunc_f32_s(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context
            .convert_op(self.masm, WasmValType::I64, |masm, dst, src, dst_size| {
                masm.signed_truncate(writable!(dst), src, S32, dst_size, TruncKind::Unchecked)
            })
    }

    fn visit_i64_trunc_f32_u(&mut self) -> Self::Output {
        use OperandSize::*;

        self.masm
            .unsigned_truncate(&mut self.context, S32, S64, TruncKind::Unchecked)
    }

    fn visit_i64_trunc_f64_s(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context
            .convert_op(self.masm, WasmValType::I64, |masm, dst, src, dst_size| {
                masm.signed_truncate(writable!(dst), src, S64, dst_size, TruncKind::Unchecked)
            })
    }

    fn visit_i64_trunc_f64_u(&mut self) -> Self::Output {
        use OperandSize::*;

        self.masm
            .unsigned_truncate(&mut self.context, S64, S64, TruncKind::Unchecked)
    }

    fn visit_i32_reinterpret_f32(&mut self) -> Self::Output {
        self.context
            .convert_op(self.masm, WasmValType::I32, |masm, dst, src, size| {
                masm.reinterpret_float_as_int(writable!(dst), src, size)
            })
    }

    fn visit_i64_reinterpret_f64(&mut self) -> Self::Output {
        self.context
            .convert_op(self.masm, WasmValType::I64, |masm, dst, src, size| {
                masm.reinterpret_float_as_int(writable!(dst), src, size)
            })
    }

    fn visit_local_get(&mut self, index: u32) -> Self::Output {
        use WasmValType::*;
        let context = &mut self.context;
        let slot = context.frame.get_wasm_local(index);
        match slot.ty {
            I32 | I64 | F32 | F64 | V128 => context.stack.push(Val::local(index, slot.ty)),
            Ref(rt) => match rt.heap_type {
                WasmHeapType::Func => context.stack.push(Val::local(index, slot.ty)),
                _ => bail!(CodeGenError::unsupported_wasm_type()),
            },
        }

        Ok(())
    }

    fn visit_local_set(&mut self, index: u32) -> Self::Output {
        let src = self.emit_set_local(index)?;
        self.context.free_reg(src);
        Ok(())
    }

    fn visit_call(&mut self, index: u32) -> Self::Output {
        let callee = self.env.callee_from_index(FuncIndex::from_u32(index));
        FnCall::emit::<M>(&mut self.env, self.masm, &mut self.context, callee)?;
        Ok(())
    }

    fn visit_call_indirect(&mut self, type_index: u32, table_index: u32) -> Self::Output {
        // Spill now because `emit_lazy_init_funcref` and the `FnCall::emit`
        // invocations will both trigger spills since they both call functions.
        // However, the machine instructions for the spill emitted by
        // `emit_lazy_funcref` will be jumped over if the funcref was previously
        // initialized which may result in the machine stack becoming
        // unbalanced.
        self.context.spill(self.masm)?;

        let type_index = TypeIndex::from_u32(type_index);
        let table_index = TableIndex::from_u32(table_index);

        self.emit_lazy_init_funcref(table_index)?;

        // Perform the indirect call.
        // This code assumes that [`Self::emit_lazy_init_funcref`] will
        // push the funcref to the value stack.
        let funcref_ptr = self
            .context
            .stack
            .peek()
            .map(|v| v.unwrap_reg())
            .ok_or_else(|| CodeGenError::missing_values_in_stack())?;
        self.masm
            .trapz(funcref_ptr.into(), TRAP_INDIRECT_CALL_TO_NULL)?;
        self.emit_typecheck_funcref(funcref_ptr.into(), type_index)?;

        let callee = self.env.funcref(type_index);
        FnCall::emit::<M>(&mut self.env, self.masm, &mut self.context, callee)?;
        Ok(())
    }

    fn visit_table_init(&mut self, elem: u32, table: u32) -> Self::Output {
        let at = self.context.stack.ensure_index_at(3)?;

        self.context
            .stack
            .insert_many(at, &[table.try_into()?, elem.try_into()?]);

        let builtin = self.env.builtins.table_init::<M::ABI, M::Ptr>()?;
        FnCall::emit::<M>(
            &mut self.env,
            self.masm,
            &mut self.context,
            Callee::Builtin(builtin.clone()),
        )?;
        self.context.pop_and_free(self.masm)
    }

    fn visit_table_copy(&mut self, dst: u32, src: u32) -> Self::Output {
        let at = self.context.stack.ensure_index_at(3)?;
        self.context
            .stack
            .insert_many(at, &[dst.try_into()?, src.try_into()?]);

        let builtin = self.env.builtins.table_copy::<M::ABI, M::Ptr>()?;
        FnCall::emit::<M>(
            &mut self.env,
            self.masm,
            &mut self.context,
            Callee::Builtin(builtin),
        )?;
        self.context.pop_and_free(self.masm)
    }

    fn visit_table_get(&mut self, table: u32) -> Self::Output {
        let table_index = TableIndex::from_u32(table);
        let table = self.env.table(table_index);
        let heap_type = table.ref_type.heap_type;

        match heap_type {
            WasmHeapType::Func => self.emit_lazy_init_funcref(table_index),
            _ => Err(anyhow!(CodeGenError::unsupported_wasm_type())),
        }
    }

    fn visit_table_grow(&mut self, table: u32) -> Self::Output {
        let table_index = TableIndex::from_u32(table);
        let ptr_type = self.env.ptr_type();
        let (heap_type, idx_type) = {
            let table_ty = self.env.table(table_index);
            (table_ty.ref_type.heap_type, table_ty.idx_type)
        };
        let builtin = match heap_type {
            WasmHeapType::Func => self.env.builtins.table_grow_func_ref::<M::ABI, M::Ptr>()?,
            _ => bail!(CodeGenError::unsupported_wasm_type()),
        };

        let len = self.context.stack.len();
        // table.grow` requires at least 2 elements on the value stack.
        let at = self.context.stack.ensure_index_at(2)?;

        // The table_grow builtin expects the parameters in a different
        // order.
        // The value stack at this point should contain:
        // [ init_value | delta ] (stack top)
        // but the builtin function expects the init value as the last
        // argument.
        self.context.stack.inner_mut().swap(len - 1, len - 2);

        let builtin = self.prepare_builtin_defined_table_arg(table_index, at, builtin)?;

        FnCall::emit::<M>(&mut self.env, self.masm, &mut self.context, builtin)?;

        // Similar to the memory.grow builtin, `table.grow` returns a
        // pointer, however, we need to ensure that the returned index
        // is representative of the address space for tables.
        match (ptr_type, idx_type) {
            (WasmValType::I64, IndexType::I64) => Ok(()),
            (WasmValType::I64, IndexType::I32) => {
                let top: Reg = self.context.pop_to_reg(self.masm, None)?.into();
                self.masm.wrap(writable!(top), top)?;
                self.context.stack.push(TypedReg::i32(top).into());
                Ok(())
            }

            _ => Err(format_err!(CodeGenError::unsupported_32_bit_platform())),
        }
    }

    fn visit_table_size(&mut self, table: u32) -> Self::Output {
        let table_index = TableIndex::from_u32(table);
        let table_data = self.env.resolve_table_data(table_index);
        self.emit_compute_table_size(&table_data)
    }

    fn visit_table_fill(&mut self, table: u32) -> Self::Output {
        let table_index = TableIndex::from_u32(table);
        let table_ty = self.env.table(table_index);

        ensure!(
            table_ty.ref_type.heap_type == WasmHeapType::Func,
            CodeGenError::unsupported_wasm_type()
        );

        let builtin = self.env.builtins.table_fill_func_ref::<M::ABI, M::Ptr>()?;

        let at = self.context.stack.ensure_index_at(3)?;

        let callee = self.prepare_builtin_defined_table_arg(table_index, at, builtin)?;
        FnCall::emit::<M>(&mut self.env, self.masm, &mut self.context, callee)?;

        self.context.pop_and_free(self.masm)
    }

    fn visit_table_set(&mut self, table: u32) -> Self::Output {
        let ptr_type = self.env.ptr_type();
        let table_index = TableIndex::from_u32(table);
        let table_data = self.env.resolve_table_data(table_index);
        let table = self.env.table(table_index);
        match table.ref_type.heap_type {
            WasmHeapType::Func => {
                ensure!(
                    self.tunables.table_lazy_init,
                    CodeGenError::unsupported_table_eager_init()
                );
                let value = self.context.pop_to_reg(self.masm, None)?;
                let index = self.context.pop_to_reg(self.masm, None)?;
                let base = self.context.any_gpr(self.masm)?;
                let elem_addr =
                    self.emit_compute_table_elem_addr(index.into(), base, &table_data)?;
                // Set the initialized bit.
                self.masm.or(
                    writable!(value.into()),
                    value.into(),
                    RegImm::i64(FUNCREF_INIT_BIT as i64),
                    ptr_type.try_into()?,
                )?;

                self.masm.store_ptr(value.into(), elem_addr)?;

                self.context.free_reg(value);
                self.context.free_reg(index);
                self.context.free_reg(base);
                Ok(())
            }
            _ => Err(anyhow!(CodeGenError::unsupported_wasm_type())),
        }
    }

    fn visit_elem_drop(&mut self, index: u32) -> Self::Output {
        let elem_drop = self.env.builtins.elem_drop::<M::ABI, M::Ptr>()?;
        self.context.stack.extend([index.try_into()?]);
        FnCall::emit::<M>(
            &mut self.env,
            self.masm,
            &mut self.context,
            Callee::Builtin(elem_drop),
        )?;
        Ok(())
    }

    fn visit_memory_init(&mut self, data_index: u32, mem: u32) -> Self::Output {
        let at = self.context.stack.ensure_index_at(3)?;
        self.context
            .stack
            .insert_many(at, &[mem.try_into()?, data_index.try_into()?]);
        let builtin = self.env.builtins.memory_init::<M::ABI, M::Ptr>()?;
        FnCall::emit::<M>(
            &mut self.env,
            self.masm,
            &mut self.context,
            Callee::Builtin(builtin),
        )?;
        self.context.pop_and_free(self.masm)
    }

    fn visit_memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> Self::Output {
        // At this point, the stack is expected to contain:
        //     [ dst_offset, src_offset, len ]
        // The following code inserts the missing params, so that stack contains:
        //     [ vmctx, dst_mem, dst_offset, src_mem, src_offset, len ]
        // Which is the order expected by the builtin function.
        let _ = self.context.stack.ensure_index_at(3)?;
        let at = self.context.stack.ensure_index_at(2)?;
        self.context.stack.insert_many(at, &[src_mem.try_into()?]);

        // One element was inserted above, so instead of 3, we use 4.
        let at = self.context.stack.ensure_index_at(4)?;
        self.context.stack.insert_many(at, &[dst_mem.try_into()?]);

        let builtin = self.env.builtins.memory_copy::<M::ABI, M::Ptr>()?;

        FnCall::emit::<M>(
            &mut self.env,
            self.masm,
            &mut self.context,
            Callee::Builtin(builtin),
        )?;
        self.context.pop_and_free(self.masm)
    }

    fn visit_memory_fill(&mut self, mem: u32) -> Self::Output {
        let at = self.context.stack.ensure_index_at(3)?;
        let mem = MemoryIndex::from_u32(mem);

        let builtin = self.env.builtins.memory_fill::<M::ABI, M::Ptr>()?;
        let builtin = self.prepare_builtin_defined_memory_arg(mem, at, builtin)?;

        FnCall::emit::<M>(&mut self.env, self.masm, &mut self.context, builtin)?;
        self.context.pop_and_free(self.masm)
    }

    fn visit_memory_size(&mut self, mem: u32) -> Self::Output {
        let heap = self.env.resolve_heap(MemoryIndex::from_u32(mem));
        self.emit_compute_memory_size(&heap)
    }

    fn visit_memory_grow(&mut self, mem: u32) -> Self::Output {
        let at = self.context.stack.ensure_index_at(1)?;
        let mem = MemoryIndex::from_u32(mem);
        // The stack at this point contains: [ delta ]
        // The desired state is
        //   [ vmctx, delta, index ]
        let builtin = self.env.builtins.memory_grow::<M::ABI, M::Ptr>()?;
        let builtin = self.prepare_builtin_defined_memory_arg(mem, at + 1, builtin)?;

        let heap = self.env.resolve_heap(mem);
        FnCall::emit::<M>(&mut self.env, self.masm, &mut self.context, builtin)?;

        // The memory32_grow builtin returns a pointer type, therefore we must
        // ensure that the return type is representative of the address space of
        // the heap type.
        match (self.env.ptr_type(), heap.index_type()) {
            (WasmValType::I64, WasmValType::I64) => Ok(()),
            // When the heap type is smaller than the pointer type, we adjust
            // the result of the memory32_grow builtin.
            (WasmValType::I64, WasmValType::I32) => {
                let top: Reg = self.context.pop_to_reg(self.masm, None)?.into();
                self.masm.wrap(writable!(top), top)?;
                self.context.stack.push(TypedReg::i32(top).into());
                Ok(())
            }
            _ => Err(anyhow!(CodeGenError::unsupported_32_bit_platform())),
        }
    }

    fn visit_data_drop(&mut self, data_index: u32) -> Self::Output {
        self.context.stack.extend([data_index.try_into()?]);

        let builtin = self.env.builtins.data_drop::<M::ABI, M::Ptr>()?;
        FnCall::emit::<M>(
            &mut self.env,
            self.masm,
            &mut self.context,
            Callee::Builtin(builtin),
        )
    }

    fn visit_nop(&mut self) -> Self::Output {
        Ok(())
    }

    fn visit_if(&mut self, blockty: BlockType) -> Self::Output {
        self.control_frames.push(ControlStackFrame::r#if(
            self.env.resolve_block_sig(blockty)?,
            self.masm,
            &mut self.context,
        )?);

        Ok(())
    }

    fn visit_else(&mut self) -> Self::Output {
        if !self.context.reachable {
            self.handle_unreachable_else()
        } else {
            let control = self
                .control_frames
                .last_mut()
                .ok_or_else(|| CodeGenError::control_frame_expected())?;
            control.emit_else(self.masm, &mut self.context)
        }
    }

    fn visit_block(&mut self, blockty: BlockType) -> Self::Output {
        self.control_frames.push(ControlStackFrame::block(
            self.env.resolve_block_sig(blockty)?,
            self.masm,
            &mut self.context,
        )?);

        Ok(())
    }

    fn visit_loop(&mut self, blockty: BlockType) -> Self::Output {
        self.control_frames.push(ControlStackFrame::r#loop(
            self.env.resolve_block_sig(blockty)?,
            self.masm,
            &mut self.context,
        )?);

        self.maybe_emit_epoch_check()?;
        self.maybe_emit_fuel_check()
    }

    fn visit_br(&mut self, depth: u32) -> Self::Output {
        let index = control_index(depth, self.control_frames.len())?;
        let frame = &mut self.control_frames[index];
        self.context
            .br::<_, _, UnconditionalBranch>(frame, self.masm, |masm, cx, frame| {
                frame.pop_abi_results::<M, _>(cx, masm, |results, _, _| {
                    Ok(results.ret_area().copied())
                })
            })
    }

    fn visit_br_if(&mut self, depth: u32) -> Self::Output {
        let index = control_index(depth, self.control_frames.len())?;
        let frame = &mut self.control_frames[index];
        frame.set_as_target();

        let top = {
            let top = self.context.without::<Result<TypedReg>, M, _>(
                frame.results::<M>()?.regs(),
                self.masm,
                |ctx, masm| ctx.pop_to_reg(masm, None),
            )??;
            // Explicitly save any live registers and locals before setting up
            // the branch state.
            // In some cases, calculating the `top` value above, will result in
            // a spill, thus the following one will result in a no-op.
            self.context.spill(self.masm)?;
            frame.top_abi_results::<M, _>(
                &mut self.context,
                self.masm,
                |results, context, masm| {
                    // In the case of `br_if` there's a possibility that we'll
                    // exit early from the block or fallthrough, for
                    // a fallthrough, we cannot rely on the pre-computed return area;
                    // it must be recalculated so that any values that are
                    // generated are correctly placed near the current stack
                    // pointer.
                    if results.on_stack() {
                        let stack_consumed = context.stack.sizeof(results.stack_operands_len());
                        let base = masm.sp_offset()?.as_u32() - stack_consumed;
                        let offs = base + results.size();
                        Ok(Some(RetArea::sp(SPOffset::from_u32(offs))))
                    } else {
                        Ok(None)
                    }
                },
            )?;
            top
        };

        // Emit instructions to balance the machine stack.
        let current_sp_offset = self.masm.sp_offset()?;
        let unbalanced = frame.unbalanced(self.masm)?;
        let (label, cmp) = if unbalanced {
            (self.masm.get_label()?, IntCmpKind::Eq)
        } else {
            (*frame.label(), IntCmpKind::Ne)
        };

        self.masm
            .branch(cmp, top.reg, top.reg.into(), label, OperandSize::S32)?;
        self.context.free_reg(top);

        if unbalanced {
            self.context
                .br::<_, _, ConditionalBranch>(frame, self.masm, |_, _, _| Ok(()))?;

            // Restore sp_offset to what it was for falling through and emit
            // fallthrough label.
            self.masm.reset_stack_pointer(current_sp_offset)?;
            self.masm.bind(label)?;
        }

        Ok(())
    }

    fn visit_br_table(&mut self, targets: BrTable<'a>) -> Self::Output {
        // +1 to account for the default target.
        let len = targets.len() + 1;
        // SmallVec<[_; 5]> to match the binary emission layer (e.g
        // see `JmpTableSeq'), but here we use 5 instead since we
        // bundle the default target as the last element in the array.
        let mut labels: SmallVec<[_; 5]> = smallvec![];
        for _ in 0..len {
            labels.push(self.masm.get_label()?);
        }

        // Find the innermost target and use it as the relative frame
        // for result handling below.
        //
        // This approach ensures that
        // 1. The stack pointer offset is correctly positioned
        //    according to the expectations of the innermost block end
        //    sequence.
        // 2. We meet the jump site invariants introduced by
        //    `CodegenContext::br`, which take advantage of Wasm
        //    semantics given that all jumps are "outward".
        let mut innermost = targets.default();
        for target in targets.targets() {
            let target = target?;
            if target < innermost {
                innermost = target;
            }
        }

        let innermost_index = control_index(innermost, self.control_frames.len())?;
        let innermost_frame = &mut self.control_frames[innermost_index];
        let innermost_result = innermost_frame.results::<M>()?;

        let (index, tmp) = {
            let index_and_tmp = self.context.without::<Result<(TypedReg, _)>, M, _>(
                innermost_result.regs(),
                self.masm,
                |cx, masm| Ok((cx.pop_to_reg(masm, None)?, cx.any_gpr(masm)?)),
            )??;

            // Materialize any constants or locals into their result
            // representation, so that when reachability is restored,
            // they are correctly located.  NB: the results are popped
            // in function of the innermost branch specified for
            // `br_table`, which implies that the machine stack will
            // be correctly balanced, by virtue of calling
            // `pop_abi_results`.

            // It's possible that we need to balance the stack for the
            // rest of the targets, which will be done before emitting
            // the unconditional jump below.
            innermost_frame.pop_abi_results::<M, _>(
                &mut self.context,
                self.masm,
                |results, _, _| Ok(results.ret_area().copied()),
            )?;
            index_and_tmp
        };

        self.masm.jmp_table(&labels, index.into(), tmp)?;
        // Save the original stack pointer offset; we will reset the stack
        // pointer to this offset after jumping to each of the targets. Each
        // jump might adjust the stack according to the base offset of the
        // target.
        let current_sp = self.masm.sp_offset()?;

        for (t, l) in targets
            .targets()
            .chain(std::iter::once(Ok(targets.default())))
            .zip(labels.iter())
        {
            let control_index = control_index(t?, self.control_frames.len())?;
            let frame = &mut self.control_frames[control_index];
            // Reset the stack pointer to its original offset. This is needed
            // because each jump will potentially adjust the stack pointer
            // according to the base offset of the target.
            self.masm.reset_stack_pointer(current_sp)?;

            // NB: We don't perform any result handling as it was
            // already taken care of above before jumping to the
            // jump table.
            self.masm.bind(*l)?;
            // Ensure that the stack pointer is correctly positioned before
            // jumping to the jump table code.
            self.context
                .br::<_, _, UnconditionalBranch>(frame, self.masm, |_, _, _| Ok(()))?;
        }
        // Finally reset the stack pointer to the original location.
        // The reachability analysis, will ensure it's correctly located
        // once reachability is restored.
        self.masm.reset_stack_pointer(current_sp)?;
        self.context.reachable = false;
        self.context.free_reg(index.reg);
        self.context.free_reg(tmp);

        Ok(())
    }

    fn visit_return(&mut self) -> Self::Output {
        // Grab the outermost frame, which is the function's body
        // frame. We don't rely on [`codegen::control_index`] since
        // this frame is implicit and we know that it should exist at
        // index 0.
        let outermost = &mut self.control_frames[0];
        self.context
            .br::<_, _, UnconditionalBranch>(outermost, self.masm, |masm, cx, frame| {
                frame.pop_abi_results::<M, _>(cx, masm, |results, _, _| {
                    Ok(results.ret_area().copied())
                })
            })
    }

    fn visit_unreachable(&mut self) -> Self::Output {
        self.masm.unreachable()?;
        self.context.reachable = false;
        // Set the implicit outermost frame as target to perform the necessary
        // stack clean up.
        let outermost = &mut self.control_frames[0];
        outermost.set_as_target();

        Ok(())
    }

    fn visit_local_tee(&mut self, index: u32) -> Self::Output {
        let typed_reg = self.emit_set_local(index)?;
        self.context.stack.push(typed_reg.into());

        Ok(())
    }

    fn visit_global_get(&mut self, global_index: u32) -> Self::Output {
        let index = GlobalIndex::from_u32(global_index);
        let (ty, base, offset) = self.emit_get_global_addr(index)?;
        let addr = self.masm.address_at_reg(base, offset)?;
        let dst = self.context.reg_for_type(ty, self.masm)?;
        self.masm.load(addr, writable!(dst), ty.try_into()?)?;
        self.context.stack.push(Val::reg(dst, ty));

        self.context.free_reg(base);

        Ok(())
    }

    fn visit_global_set(&mut self, global_index: u32) -> Self::Output {
        let index = GlobalIndex::from_u32(global_index);
        let (ty, base, offset) = self.emit_get_global_addr(index)?;
        let addr = self.masm.address_at_reg(base, offset)?;

        let typed_reg = self.context.pop_to_reg(self.masm, None)?;
        self.masm
            .store(typed_reg.reg.into(), addr, ty.try_into()?)?;
        self.context.free_reg(typed_reg.reg);
        self.context.free_reg(base);

        Ok(())
    }

    fn visit_drop(&mut self) -> Self::Output {
        self.context.drop_last(1, |regalloc, val| match val {
            Val::Reg(tr) => Ok(regalloc.free(tr.reg)),
            Val::Memory(m) => self.masm.free_stack(m.slot.size),
            _ => Ok(()),
        })
    }

    fn visit_select(&mut self) -> Self::Output {
        let cond = self.context.pop_to_reg(self.masm, None)?;
        let val2 = self.context.pop_to_reg(self.masm, None)?;
        let val1 = self.context.pop_to_reg(self.masm, None)?;
        self.masm.cmp(cond.reg, RegImm::i32(0), OperandSize::S32)?;
        // Conditionally move val1 to val2 if the comparison is
        // not zero.
        self.masm.cmov(
            writable!(val2.into()),
            val1.into(),
            IntCmpKind::Ne,
            val1.ty.try_into()?,
        )?;
        self.context.stack.push(val2.into());
        self.context.free_reg(val1.reg);
        self.context.free_reg(cond);

        Ok(())
    }

    fn visit_i32_load(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I32,
            LoadKind::Operand(OperandSize::S32),
        )
    }

    fn visit_i32_load8_s(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I32,
            LoadKind::ScalarExtend(Extend::<Signed>::I32Extend8.into()),
        )
    }

    fn visit_i32_load8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I32,
            LoadKind::ScalarExtend(Extend::<Zero>::I32Extend8.into()),
        )
    }

    fn visit_i32_load16_s(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I32,
            LoadKind::ScalarExtend(Extend::<Signed>::I32Extend16.into()),
        )
    }

    fn visit_i32_load16_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I32,
            LoadKind::ScalarExtend(Extend::<Zero>::I32Extend16.into()),
        )
    }

    fn visit_i32_store(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S32))
    }

    fn visit_i32_store8(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S8))
    }

    fn visit_i32_store16(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S16))
    }

    fn visit_i64_load8_s(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::ScalarExtend(Extend::<Signed>::I64Extend8.into()),
        )
    }

    fn visit_i64_load8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::ScalarExtend(Extend::<Zero>::I64Extend8.into()),
        )
    }

    fn visit_i64_load16_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::ScalarExtend(Extend::<Zero>::I64Extend16.into()),
        )
    }

    fn visit_i64_load16_s(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::ScalarExtend(Extend::<Signed>::I64Extend16.into()),
        )
    }

    fn visit_i64_load32_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::ScalarExtend(Extend::<Zero>::I64Extend32.into()),
        )
    }

    fn visit_i64_load32_s(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::ScalarExtend(Extend::<Signed>::I64Extend32.into()),
        )
    }

    fn visit_i64_load(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::Operand(OperandSize::S64),
        )
    }

    fn visit_i64_store(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S64))
    }

    fn visit_i64_store8(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S8))
    }

    fn visit_i64_store16(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S16))
    }

    fn visit_i64_store32(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S32))
    }

    fn visit_f32_load(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::F32,
            LoadKind::Operand(OperandSize::S32),
        )
    }

    fn visit_f32_store(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S32))
    }

    fn visit_f64_load(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::F64,
            LoadKind::Operand(OperandSize::S64),
        )
    }

    fn visit_f64_store(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S64))
    }

    fn visit_i32_trunc_sat_f32_s(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context
            .convert_op(self.masm, WasmValType::I32, |masm, dst, src, dst_size| {
                masm.signed_truncate(writable!(dst), src, S32, dst_size, TruncKind::Checked)
            })
    }

    fn visit_i32_trunc_sat_f32_u(&mut self) -> Self::Output {
        use OperandSize::*;

        self.masm
            .unsigned_truncate(&mut self.context, S32, S32, TruncKind::Checked)
    }

    fn visit_i32_trunc_sat_f64_s(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context
            .convert_op(self.masm, WasmValType::I32, |masm, dst, src, dst_size| {
                masm.signed_truncate(writable!(dst), src, S64, dst_size, TruncKind::Checked)
            })
    }

    fn visit_i32_trunc_sat_f64_u(&mut self) -> Self::Output {
        use OperandSize::*;

        self.masm
            .unsigned_truncate(&mut self.context, S64, S32, TruncKind::Checked)
    }

    fn visit_i64_trunc_sat_f32_s(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context
            .convert_op(self.masm, WasmValType::I64, |masm, dst, src, dst_size| {
                masm.signed_truncate(writable!(dst), src, S32, dst_size, TruncKind::Checked)
            })
    }

    fn visit_i64_trunc_sat_f32_u(&mut self) -> Self::Output {
        use OperandSize::*;

        self.masm
            .unsigned_truncate(&mut self.context, S32, S64, TruncKind::Checked)
    }

    fn visit_i64_trunc_sat_f64_s(&mut self) -> Self::Output {
        use OperandSize::*;

        self.context
            .convert_op(self.masm, WasmValType::I64, |masm, dst, src, dst_size| {
                masm.signed_truncate(writable!(dst), src, S64, dst_size, TruncKind::Checked)
            })
    }

    fn visit_i64_trunc_sat_f64_u(&mut self) -> Self::Output {
        use OperandSize::*;

        self.masm
            .unsigned_truncate(&mut self.context, S64, S64, TruncKind::Checked)
    }

    fn visit_i64_add128(&mut self) -> Self::Output {
        self.context
            .binop128(self.masm, |masm, lhs_lo, lhs_hi, rhs_lo, rhs_hi| {
                masm.add128(
                    writable!(lhs_lo),
                    writable!(lhs_hi),
                    lhs_lo,
                    lhs_hi,
                    rhs_lo,
                    rhs_hi,
                )?;
                Ok((TypedReg::i64(lhs_lo), TypedReg::i64(lhs_hi)))
            })
    }

    fn visit_i64_sub128(&mut self) -> Self::Output {
        self.context
            .binop128(self.masm, |masm, lhs_lo, lhs_hi, rhs_lo, rhs_hi| {
                masm.sub128(
                    writable!(lhs_lo),
                    writable!(lhs_hi),
                    lhs_lo,
                    lhs_hi,
                    rhs_lo,
                    rhs_hi,
                )?;
                Ok((TypedReg::i64(lhs_lo), TypedReg::i64(lhs_hi)))
            })
    }

    fn visit_i64_mul_wide_s(&mut self) -> Self::Output {
        self.masm.mul_wide(&mut self.context, MulWideKind::Signed)
    }

    fn visit_i64_mul_wide_u(&mut self) -> Self::Output {
        self.masm.mul_wide(&mut self.context, MulWideKind::Unsigned)
    }

    fn visit_i32_atomic_load8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I32,
            LoadKind::Atomic(OperandSize::S8, Some(Extend::<Zero>::I32Extend8.into())),
        )
    }

    fn visit_i32_atomic_load16_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I32,
            LoadKind::Atomic(OperandSize::S16, Some(Extend::<Zero>::I32Extend16.into())),
        )
    }

    fn visit_i32_atomic_load(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I32,
            LoadKind::Atomic(OperandSize::S32, None),
        )
    }

    fn visit_i64_atomic_load8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::Atomic(OperandSize::S8, Some(Extend::<Zero>::I64Extend8.into())),
        )
    }

    fn visit_i64_atomic_load16_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::Atomic(OperandSize::S16, Some(Extend::<Zero>::I64Extend16.into())),
        )
    }

    fn visit_i64_atomic_load32_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::Atomic(OperandSize::S32, Some(Extend::<Zero>::I64Extend32.into())),
        )
    }

    fn visit_i64_atomic_load(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::I64,
            LoadKind::Atomic(OperandSize::S64, None),
        )
    }

    fn visit_i32_atomic_store(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Atomic(OperandSize::S32))
    }

    fn visit_i64_atomic_store(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Atomic(OperandSize::S64))
    }

    fn visit_i32_atomic_store8(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Atomic(OperandSize::S8))
    }

    fn visit_i32_atomic_store16(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Atomic(OperandSize::S16))
    }

    fn visit_i64_atomic_store8(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Atomic(OperandSize::S8))
    }

    fn visit_i64_atomic_store16(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Atomic(OperandSize::S16))
    }

    fn visit_i64_atomic_store32(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Atomic(OperandSize::S32))
    }

    fn visit_i32_atomic_rmw8_add_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Add,
            OperandSize::S8,
            Some(Extend::<Zero>::I32Extend8),
        )
    }

    fn visit_i32_atomic_rmw16_add_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Add,
            OperandSize::S16,
            Some(Extend::<Zero>::I32Extend16),
        )
    }

    fn visit_i32_atomic_rmw_add(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Add, OperandSize::S32, None)
    }

    fn visit_i64_atomic_rmw8_add_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Add,
            OperandSize::S8,
            Some(Extend::<Zero>::I64Extend8),
        )
    }

    fn visit_i64_atomic_rmw16_add_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Add,
            OperandSize::S16,
            Some(Extend::<Zero>::I64Extend16),
        )
    }

    fn visit_i64_atomic_rmw32_add_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Add,
            OperandSize::S32,
            Some(Extend::<Zero>::I64Extend32),
        )
    }

    fn visit_i64_atomic_rmw_add(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Add, OperandSize::S64, None)
    }

    fn visit_i32_atomic_rmw8_sub_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Sub,
            OperandSize::S8,
            Some(Extend::<Zero>::I32Extend8),
        )
    }
    fn visit_i32_atomic_rmw16_sub_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Sub,
            OperandSize::S16,
            Some(Extend::<Zero>::I32Extend16),
        )
    }

    fn visit_i32_atomic_rmw_sub(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Sub, OperandSize::S32, None)
    }

    fn visit_i64_atomic_rmw8_sub_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Sub,
            OperandSize::S8,
            Some(Extend::<Zero>::I64Extend8),
        )
    }

    fn visit_i64_atomic_rmw16_sub_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Sub,
            OperandSize::S16,
            Some(Extend::<Zero>::I64Extend16),
        )
    }

    fn visit_i64_atomic_rmw32_sub_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Sub,
            OperandSize::S32,
            Some(Extend::<Zero>::I64Extend32),
        )
    }

    fn visit_i64_atomic_rmw_sub(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Sub, OperandSize::S64, None)
    }

    fn visit_i32_atomic_rmw8_xchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xchg,
            OperandSize::S8,
            Some(Extend::<Zero>::I32Extend8),
        )
    }

    fn visit_i32_atomic_rmw16_xchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xchg,
            OperandSize::S16,
            Some(Extend::<Zero>::I32Extend16),
        )
    }

    fn visit_i32_atomic_rmw_xchg(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Xchg, OperandSize::S32, None)
    }

    fn visit_i64_atomic_rmw8_xchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xchg,
            OperandSize::S8,
            Some(Extend::<Zero>::I64Extend8),
        )
    }

    fn visit_i64_atomic_rmw16_xchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xchg,
            OperandSize::S16,
            Some(Extend::<Zero>::I64Extend16),
        )
    }

    fn visit_i64_atomic_rmw32_xchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xchg,
            OperandSize::S32,
            Some(Extend::<Zero>::I64Extend32),
        )
    }

    fn visit_i64_atomic_rmw_xchg(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Xchg, OperandSize::S64, None)
    }

    fn visit_i32_atomic_rmw8_and_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::And,
            OperandSize::S8,
            Some(Extend::<Zero>::I32Extend8),
        )
    }

    fn visit_i32_atomic_rmw16_and_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::And,
            OperandSize::S16,
            Some(Extend::<Zero>::I32Extend16),
        )
    }

    fn visit_i32_atomic_rmw_and(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::And, OperandSize::S32, None)
    }

    fn visit_i64_atomic_rmw8_and_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::And,
            OperandSize::S8,
            Some(Extend::<Zero>::I64Extend8),
        )
    }

    fn visit_i64_atomic_rmw16_and_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::And,
            OperandSize::S16,
            Some(Extend::<Zero>::I64Extend16),
        )
    }

    fn visit_i64_atomic_rmw32_and_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::And,
            OperandSize::S32,
            Some(Extend::<Zero>::I64Extend32),
        )
    }

    fn visit_i64_atomic_rmw_and(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::And, OperandSize::S64, None)
    }

    fn visit_i32_atomic_rmw8_or_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Or,
            OperandSize::S8,
            Some(Extend::<Zero>::I32Extend8),
        )
    }

    fn visit_i32_atomic_rmw16_or_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Or,
            OperandSize::S16,
            Some(Extend::<Zero>::I32Extend16),
        )
    }

    fn visit_i32_atomic_rmw_or(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Or, OperandSize::S32, None)
    }

    fn visit_i64_atomic_rmw8_or_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Or,
            OperandSize::S8,
            Some(Extend::<Zero>::I64Extend8),
        )
    }

    fn visit_i64_atomic_rmw16_or_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Or,
            OperandSize::S16,
            Some(Extend::<Zero>::I64Extend16),
        )
    }

    fn visit_i64_atomic_rmw32_or_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Or,
            OperandSize::S32,
            Some(Extend::<Zero>::I64Extend32),
        )
    }

    fn visit_i64_atomic_rmw_or(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Or, OperandSize::S64, None)
    }

    fn visit_i32_atomic_rmw8_xor_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xor,
            OperandSize::S8,
            Some(Extend::<Zero>::I32Extend8),
        )
    }

    fn visit_i32_atomic_rmw16_xor_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xor,
            OperandSize::S16,
            Some(Extend::<Zero>::I32Extend16),
        )
    }

    fn visit_i32_atomic_rmw_xor(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Xor, OperandSize::S32, None)
    }

    fn visit_i64_atomic_rmw8_xor_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xor,
            OperandSize::S8,
            Some(Extend::<Zero>::I64Extend8),
        )
    }

    fn visit_i64_atomic_rmw16_xor_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xor,
            OperandSize::S16,
            Some(Extend::<Zero>::I64Extend16),
        )
    }

    fn visit_i64_atomic_rmw32_xor_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(
            &arg,
            RmwOp::Xor,
            OperandSize::S32,
            Some(Extend::<Zero>::I64Extend32),
        )
    }

    fn visit_i64_atomic_rmw_xor(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_rmw(&arg, RmwOp::Xor, OperandSize::S64, None)
    }

    fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_cmpxchg(&arg, OperandSize::S8, Some(Extend::I32Extend8))
    }

    fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_cmpxchg(&arg, OperandSize::S16, Some(Extend::I32Extend16))
    }

    fn visit_i32_atomic_rmw_cmpxchg(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_cmpxchg(&arg, OperandSize::S32, None)
    }

    fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_cmpxchg(&arg, OperandSize::S8, Some(Extend::I64Extend8))
    }

    fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_cmpxchg(&arg, OperandSize::S16, Some(Extend::I64Extend16))
    }

    fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_cmpxchg(&arg, OperandSize::S32, Some(Extend::I64Extend32))
    }

    fn visit_i64_atomic_rmw_cmpxchg(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_cmpxchg(&arg, OperandSize::S64, None)
    }

    fn visit_memory_atomic_wait32(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_wait(&arg, AtomicWaitKind::Wait32)
    }

    fn visit_memory_atomic_wait64(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_wait(&arg, AtomicWaitKind::Wait64)
    }

    fn visit_memory_atomic_notify(&mut self, arg: MemArg) -> Self::Output {
        self.emit_atomic_notify(&arg)
    }

    fn visit_atomic_fence(&mut self) -> Self::Output {
        self.masm.fence()
    }

    wasmparser::for_each_visit_operator!(def_unsupported);
}

impl<'a, 'translation, 'data, M> VisitSimdOperator<'a>
    for CodeGen<'a, 'translation, 'data, M, Emission>
where
    M: MacroAssembler,
{
    fn visit_v128_const(&mut self, val: V128) -> Self::Output {
        self.context.stack.push(Val::v128(val.i128()));
        Ok(())
    }

    fn visit_v128_load(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::Operand(OperandSize::S128),
        )
    }

    fn visit_v128_store(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_store(&memarg, StoreKind::Operand(OperandSize::S128))
    }

    fn visit_v128_load8x8_s(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::VectorExtend(V128LoadExtendKind::E8x8S),
        )
    }

    fn visit_v128_load8x8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::VectorExtend(V128LoadExtendKind::E8x8U),
        )
    }

    fn visit_v128_load16x4_s(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::VectorExtend(V128LoadExtendKind::E16x4S),
        )
    }

    fn visit_v128_load16x4_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::VectorExtend(V128LoadExtendKind::E16x4U),
        )
    }

    fn visit_v128_load32x2_s(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::VectorExtend(V128LoadExtendKind::E32x2S),
        )
    }

    fn visit_v128_load32x2_u(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::VectorExtend(V128LoadExtendKind::E32x2U),
        )
    }

    fn visit_v128_load8_splat(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::Splat(SplatLoadKind::S8),
        )
    }

    fn visit_v128_load16_splat(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::Splat(SplatLoadKind::S16),
        )
    }

    fn visit_v128_load32_splat(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::Splat(SplatLoadKind::S32),
        )
    }

    fn visit_v128_load64_splat(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::Splat(SplatLoadKind::S64),
        )
    }

    fn visit_i8x16_splat(&mut self) -> Self::Output {
        self.masm.splat(&mut self.context, SplatKind::I8x16)
    }

    fn visit_i16x8_splat(&mut self) -> Self::Output {
        self.masm.splat(&mut self.context, SplatKind::I16x8)
    }

    fn visit_i32x4_splat(&mut self) -> Self::Output {
        self.masm.splat(&mut self.context, SplatKind::I32x4)
    }

    fn visit_i64x2_splat(&mut self) -> Self::Output {
        self.masm.splat(&mut self.context, SplatKind::I64x2)
    }

    fn visit_f32x4_splat(&mut self) -> Self::Output {
        self.masm.splat(&mut self.context, SplatKind::F32x4)
    }

    fn visit_f64x2_splat(&mut self) -> Self::Output {
        self.masm.splat(&mut self.context, SplatKind::F64x2)
    }

    fn visit_i8x16_shuffle(&mut self, lanes: [u8; 16]) -> Self::Output {
        let rhs = self.context.pop_to_reg(self.masm, None)?;
        let lhs = self.context.pop_to_reg(self.masm, None)?;
        self.masm
            .shuffle(writable!(lhs.into()), lhs.into(), rhs.into(), lanes)?;
        self.context.stack.push(TypedReg::v128(lhs.into()).into());
        self.context.free_reg(rhs);
        Ok(())
    }

    fn visit_i8x16_swizzle(&mut self) -> Self::Output {
        let rhs = self.context.pop_to_reg(self.masm, None)?;
        let lhs = self.context.pop_to_reg(self.masm, None)?;
        self.masm
            .swizzle(writable!(lhs.into()), lhs.into(), rhs.into())?;
        self.context.stack.push(TypedReg::v128(lhs.into()).into());
        self.context.free_reg(rhs);
        Ok(())
    }

    fn visit_i8x16_extract_lane_s(&mut self, lane: u8) -> Self::Output {
        self.context.extract_lane_op(
            self.masm,
            ExtractLaneKind::I8x16S,
            |masm, src, dst, kind| masm.extract_lane(src, dst, lane, kind),
        )
    }

    fn visit_i8x16_extract_lane_u(&mut self, lane: u8) -> Self::Output {
        self.context.extract_lane_op(
            self.masm,
            ExtractLaneKind::I8x16U,
            |masm, src, dst, kind| masm.extract_lane(src, dst, lane, kind),
        )
    }

    fn visit_i16x8_extract_lane_s(&mut self, lane: u8) -> Self::Output {
        self.context.extract_lane_op(
            self.masm,
            ExtractLaneKind::I16x8S,
            |masm, src, dst, kind| masm.extract_lane(src, dst, lane, kind),
        )
    }

    fn visit_i16x8_extract_lane_u(&mut self, lane: u8) -> Self::Output {
        self.context.extract_lane_op(
            self.masm,
            ExtractLaneKind::I16x8U,
            |masm, src, dst, kind| masm.extract_lane(src, dst, lane, kind),
        )
    }

    fn visit_i32x4_extract_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .extract_lane_op(self.masm, ExtractLaneKind::I32x4, |masm, src, dst, kind| {
                masm.extract_lane(src, dst, lane, kind)
            })
    }

    fn visit_i64x2_extract_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .extract_lane_op(self.masm, ExtractLaneKind::I64x2, |masm, src, dst, kind| {
                masm.extract_lane(src, dst, lane, kind)
            })
    }

    fn visit_f32x4_extract_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .extract_lane_op(self.masm, ExtractLaneKind::F32x4, |masm, src, dst, kind| {
                masm.extract_lane(src, dst, lane, kind)
            })
    }

    fn visit_f64x2_extract_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .extract_lane_op(self.masm, ExtractLaneKind::F64x2, |masm, src, dst, kind| {
                masm.extract_lane(src, dst, lane, kind)
            })
    }

    fn visit_i8x16_eq(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_eq(writable!(dst), dst, src, VectorEqualityKind::I8x16)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_eq(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_eq(writable!(dst), dst, src, VectorEqualityKind::I16x8)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_eq(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_eq(writable!(dst), dst, src, VectorEqualityKind::I32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i64x2_eq(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_eq(writable!(dst), dst, src, VectorEqualityKind::I64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_eq(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_eq(writable!(dst), dst, src, VectorEqualityKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_eq(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_eq(writable!(dst), dst, src, VectorEqualityKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_ne(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_ne(writable!(dst), dst, src, VectorEqualityKind::I8x16)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_ne(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_ne(writable!(dst), dst, src, VectorEqualityKind::I16x8)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_ne(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_ne(writable!(dst), dst, src, VectorEqualityKind::I32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i64x2_ne(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_ne(writable!(dst), dst, src, VectorEqualityKind::I64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_ne(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_ne(writable!(dst), dst, src, VectorEqualityKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_ne(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_ne(writable!(dst), dst, src, VectorEqualityKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_lt_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_lt(writable!(dst), dst, src, VectorCompareKind::I8x16S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_lt_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_lt(writable!(dst), dst, src, VectorCompareKind::I8x16U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_lt_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_lt(writable!(dst), dst, src, VectorCompareKind::I16x8S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_lt_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_lt(writable!(dst), dst, src, VectorCompareKind::I16x8U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_lt_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_lt(writable!(dst), dst, src, VectorCompareKind::I32x4S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_lt_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_lt(writable!(dst), dst, src, VectorCompareKind::I32x4U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i64x2_lt_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_lt(writable!(dst), dst, src, VectorCompareKind::I64x2S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_lt(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_lt(writable!(dst), dst, src, VectorCompareKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_lt(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_lt(writable!(dst), dst, src, VectorCompareKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_le_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_le(writable!(dst), dst, src, VectorCompareKind::I8x16S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_le_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_le(writable!(dst), dst, src, VectorCompareKind::I8x16U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_le_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_le(writable!(dst), dst, src, VectorCompareKind::I16x8S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_le_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_le(writable!(dst), dst, src, VectorCompareKind::I16x8U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_le_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_le(writable!(dst), dst, src, VectorCompareKind::I32x4S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_le_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_le(writable!(dst), dst, src, VectorCompareKind::I32x4U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i64x2_le_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_le(writable!(dst), dst, src, VectorCompareKind::I64x2S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_le(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_le(writable!(dst), dst, src, VectorCompareKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_le(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_le(writable!(dst), dst, src, VectorCompareKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_gt_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_gt(writable!(dst), dst, src, VectorCompareKind::I8x16S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_gt_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_gt(writable!(dst), dst, src, VectorCompareKind::I8x16U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_gt_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_gt(writable!(dst), dst, src, VectorCompareKind::I16x8S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_gt_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_gt(writable!(dst), dst, src, VectorCompareKind::I16x8U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_gt_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_gt(writable!(dst), dst, src, VectorCompareKind::I32x4S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_gt_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_gt(writable!(dst), dst, src, VectorCompareKind::I32x4U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i64x2_gt_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_gt(writable!(dst), dst, src, VectorCompareKind::I64x2S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_gt(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_gt(writable!(dst), dst, src, VectorCompareKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_gt(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_gt(writable!(dst), dst, src, VectorCompareKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_ge_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_ge(writable!(dst), dst, src, VectorCompareKind::I8x16S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_ge_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_ge(writable!(dst), dst, src, VectorCompareKind::I8x16U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_ge_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_ge(writable!(dst), dst, src, VectorCompareKind::I16x8S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_ge_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_ge(writable!(dst), dst, src, VectorCompareKind::I16x8U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_ge_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_ge(writable!(dst), dst, src, VectorCompareKind::I32x4S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_ge_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_ge(writable!(dst), dst, src, VectorCompareKind::I32x4U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i64x2_ge_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_ge(writable!(dst), dst, src, VectorCompareKind::I64x2S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_ge(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_ge(writable!(dst), dst, src, VectorCompareKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_ge(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_ge(writable!(dst), dst, src, VectorCompareKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .replace_lane_op(self.masm, ReplaceLaneKind::I8x16, |masm, src, dst, kind| {
                masm.replace_lane(src, dst, lane, kind)
            })
    }

    fn visit_i16x8_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .replace_lane_op(self.masm, ReplaceLaneKind::I16x8, |masm, src, dst, kind| {
                masm.replace_lane(src, dst, lane, kind)
            })
    }

    fn visit_i32x4_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .replace_lane_op(self.masm, ReplaceLaneKind::I32x4, |masm, src, dst, kind| {
                masm.replace_lane(src, dst, lane, kind)
            })
    }

    fn visit_i64x2_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .replace_lane_op(self.masm, ReplaceLaneKind::I64x2, |masm, src, dst, kind| {
                masm.replace_lane(src, dst, lane, kind)
            })
    }

    fn visit_f32x4_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .replace_lane_op(self.masm, ReplaceLaneKind::F32x4, |masm, src, dst, kind| {
                masm.replace_lane(src, dst, lane, kind)
            })
    }

    fn visit_f64x2_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.context
            .replace_lane_op(self.masm, ReplaceLaneKind::F64x2, |masm, src, dst, kind| {
                masm.replace_lane(src, dst, lane, kind)
            })
    }

    fn visit_v128_not(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_not(writable!(reg))?;
            Ok(TypedReg::new(WasmValType::V128, reg))
        })
    }

    fn visit_v128_and(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S128, |masm, dst, src, _size| {
                masm.v128_and(dst, src, writable!(dst))?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_v128_andnot(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S128, |masm, dst, src, _size| {
                // careful here: and_not is *not* commutative: dst = !src1 & src2
                masm.v128_and_not(src, dst, writable!(dst))?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_v128_or(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S128, |masm, dst, src, _size| {
                // careful here: and_not is *not* commutative: dst = !src1 & src2
                masm.v128_or(src, dst, writable!(dst))?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_v128_xor(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S128, |masm, dst, src, _size| {
                // careful here: and_not is *not* commutative: dst = !src1 & src2
                masm.v128_xor(src, dst, writable!(dst))?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_v128_bitselect(&mut self) -> Self::Output {
        let mask = self.context.pop_to_reg(self.masm, None)?;
        let op2 = self.context.pop_to_reg(self.masm, None)?;
        let op1 = self.context.pop_to_reg(self.masm, None)?;
        let dst = self.context.any_fpr(self.masm)?;

        // careful here: bitselect is *not* commutative.
        self.masm
            .v128_bitselect(op1.reg, op2.reg, mask.reg, writable!(dst))?;

        self.context
            .stack
            .push(TypedReg::new(WasmValType::V128, dst).into());
        self.context.free_reg(op1);
        self.context.free_reg(op2);
        self.context.free_reg(mask);

        Ok(())
    }

    fn visit_v128_any_true(&mut self) -> Self::Output {
        let src = self.context.pop_to_reg(self.masm, None)?;
        let dst = self.context.any_gpr(self.masm)?;

        self.masm.v128_any_true(src.reg, writable!(dst))?;

        self.context
            .stack
            .push(TypedReg::new(WasmValType::I32, dst).into());
        self.context.free_reg(src);

        Ok(())
    }

    fn visit_v128_load8_lane(&mut self, arg: MemArg, lane: u8) -> Self::Output {
        self.emit_wasm_load(
            &arg,
            WasmValType::V128,
            LoadKind::vector_lane(lane, OperandSize::S8),
        )
    }

    fn visit_v128_load16_lane(&mut self, arg: MemArg, lane: u8) -> Self::Output {
        self.emit_wasm_load(
            &arg,
            WasmValType::V128,
            LoadKind::vector_lane(lane, OperandSize::S16),
        )
    }

    fn visit_v128_load32_lane(&mut self, arg: MemArg, lane: u8) -> Self::Output {
        self.emit_wasm_load(
            &arg,
            WasmValType::V128,
            LoadKind::vector_lane(lane, OperandSize::S32),
        )
    }

    fn visit_v128_load64_lane(&mut self, arg: MemArg, lane: u8) -> Self::Output {
        self.emit_wasm_load(
            &arg,
            WasmValType::V128,
            LoadKind::vector_lane(lane, OperandSize::S64),
        )
    }

    fn visit_v128_store8_lane(&mut self, arg: MemArg, lane: u8) -> Self::Output {
        self.emit_wasm_store(&arg, StoreKind::vector_lane(lane, OperandSize::S8))
    }

    fn visit_v128_store16_lane(&mut self, arg: MemArg, lane: u8) -> Self::Output {
        self.emit_wasm_store(&arg, StoreKind::vector_lane(lane, OperandSize::S16))
    }

    fn visit_v128_store32_lane(&mut self, arg: MemArg, lane: u8) -> Self::Output {
        self.emit_wasm_store(&arg, StoreKind::vector_lane(lane, OperandSize::S32))
    }

    fn visit_v128_store64_lane(&mut self, arg: MemArg, lane: u8) -> Self::Output {
        self.emit_wasm_store(&arg, StoreKind::vector_lane(lane, OperandSize::S64))
    }

    fn visit_f32x4_convert_i32x4_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_convert(reg, writable!(reg), V128ConvertKind::I32x4S)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f32x4_convert_i32x4_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_convert(reg, writable!(reg), V128ConvertKind::I32x4U)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f64x2_convert_low_i32x4_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_convert(reg, writable!(reg), V128ConvertKind::I32x4LowS)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f64x2_convert_low_i32x4_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_convert(reg, writable!(reg), V128ConvertKind::I32x4LowU)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i8x16_narrow_i16x8_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_narrow(dst, src, writable!(dst), V128NarrowKind::I16x8S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_narrow_i16x8_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_narrow(dst, src, writable!(dst), V128NarrowKind::I16x8U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_narrow_i32x4_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_narrow(dst, src, writable!(dst), V128NarrowKind::I32x4S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_narrow_i32x4_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_narrow(dst, src, writable!(dst), V128NarrowKind::I32x4U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_demote_f64x2_zero(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_demote(reg, writable!(reg))?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f64x2_promote_low_f32x4(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_promote(reg, writable!(reg))?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i16x8_extend_low_i8x16_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::LowI8x16S)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i16x8_extend_high_i8x16_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::HighI8x16S)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i16x8_extend_low_i8x16_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::LowI8x16U)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i16x8_extend_high_i8x16_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::HighI8x16U)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i32x4_extend_low_i16x8_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::LowI16x8S)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i32x4_extend_high_i16x8_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::HighI16x8S)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i32x4_extend_low_i16x8_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::LowI16x8U)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i32x4_extend_high_i16x8_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::HighI16x8U)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i64x2_extend_low_i32x4_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::LowI32x4S)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i64x2_extend_high_i32x4_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::HighI32x4S)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i64x2_extend_low_i32x4_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::LowI32x4U)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i64x2_extend_high_i32x4_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_extend(reg, writable!(reg), V128ExtendKind::HighI32x4U)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_i8x16_add(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::I8x16)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i16x8_add(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::I16x8)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i32x4_add(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::I32x4)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i64x2_add(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::I64x2)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i8x16_sub(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::I8x16)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i16x8_sub(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::I16x8)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i32x4_sub(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::I32x4)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i64x2_sub(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::I64x2)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i16x8_mul(&mut self) -> Self::Output {
        self.masm.v128_mul(&mut self.context, V128MulKind::I16x8)
    }

    fn visit_i32x4_mul(&mut self) -> Self::Output {
        self.masm.v128_mul(&mut self.context, V128MulKind::I32x4)
    }

    fn visit_i64x2_mul(&mut self) -> Self::Output {
        self.masm.v128_mul(&mut self.context, V128MulKind::I64x2)
    }

    fn visit_i8x16_add_sat_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::I8x16SatS)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i16x8_add_sat_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::I16x8SatS)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i8x16_add_sat_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::I8x16SatU)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i16x8_add_sat_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::I16x8SatU)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i8x16_sub_sat_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::I8x16SatS)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i16x8_sub_sat_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::I16x8SatS)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i8x16_sub_sat_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::I8x16SatU)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i16x8_sub_sat_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::I16x8SatU)?;
                Ok(TypedReg::new(WasmValType::V128, dst))
            })
    }

    fn visit_i8x16_abs(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_abs(reg, writable!(reg), V128AbsKind::I8x16)?;
            Ok(TypedReg::new(WasmValType::V128, reg))
        })
    }

    fn visit_i16x8_abs(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_abs(reg, writable!(reg), V128AbsKind::I16x8)?;
            Ok(TypedReg::new(WasmValType::V128, reg))
        })
    }

    fn visit_i32x4_abs(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_abs(reg, writable!(reg), V128AbsKind::I32x4)?;
            Ok(TypedReg::new(WasmValType::V128, reg))
        })
    }

    fn visit_i64x2_abs(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_abs(reg, writable!(reg), V128AbsKind::I64x2)?;
            Ok(TypedReg::new(WasmValType::V128, reg))
        })
    }

    fn visit_f32x4_abs(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_abs(reg, writable!(reg), V128AbsKind::F32x4)?;
            Ok(TypedReg::new(WasmValType::V128, reg))
        })
    }

    fn visit_f64x2_abs(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_abs(reg, writable!(reg), V128AbsKind::F64x2)?;
            Ok(TypedReg::new(WasmValType::V128, reg))
        })
    }

    fn visit_i8x16_neg(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, op| {
            masm.v128_neg(writable!(op), V128NegKind::I8x16)?;
            Ok(TypedReg::new(WasmValType::V128, op))
        })
    }

    fn visit_i16x8_neg(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, op| {
            masm.v128_neg(writable!(op), V128NegKind::I16x8)?;
            Ok(TypedReg::new(WasmValType::V128, op))
        })
    }

    fn visit_i32x4_neg(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, op| {
            masm.v128_neg(writable!(op), V128NegKind::I32x4)?;
            Ok(TypedReg::new(WasmValType::V128, op))
        })
    }

    fn visit_i64x2_neg(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, op| {
            masm.v128_neg(writable!(op), V128NegKind::I64x2)?;
            Ok(TypedReg::new(WasmValType::V128, op))
        })
    }

    fn visit_i8x16_shl(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S8, ShiftKind::Shl)
    }

    fn visit_i16x8_shl(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S16, ShiftKind::Shl)
    }

    fn visit_i32x4_shl(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S32, ShiftKind::Shl)
    }

    fn visit_i64x2_shl(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S64, ShiftKind::Shl)
    }

    fn visit_i8x16_shr_u(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S8, ShiftKind::ShrU)
    }

    fn visit_i16x8_shr_u(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S16, ShiftKind::ShrU)
    }

    fn visit_i32x4_shr_u(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S32, ShiftKind::ShrU)
    }

    fn visit_i64x2_shr_u(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S64, ShiftKind::ShrU)
    }

    fn visit_i8x16_shr_s(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S8, ShiftKind::ShrS)
    }

    fn visit_i16x8_shr_s(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S16, ShiftKind::ShrS)
    }

    fn visit_i32x4_shr_s(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S32, ShiftKind::ShrS)
    }

    fn visit_i64x2_shr_s(&mut self) -> Self::Output {
        self.masm
            .v128_shift(&mut self.context, OperandSize::S64, ShiftKind::ShrS)
    }

    fn visit_i16x8_q15mulr_sat_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, size| {
                masm.v128_q15mulr_sat_s(dst, src, writable!(dst), size)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_min_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_min(src, dst, writable!(dst), V128MinKind::I8x16S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_all_true(&mut self) -> Self::Output {
        self.context.v128_all_true_op(self.masm, |masm, src, dst| {
            masm.v128_all_true(src, writable!(dst), OperandSize::S8)
        })
    }

    fn visit_i16x8_all_true(&mut self) -> Self::Output {
        self.context.v128_all_true_op(self.masm, |masm, src, dst| {
            masm.v128_all_true(src, writable!(dst), OperandSize::S16)
        })
    }

    fn visit_i32x4_all_true(&mut self) -> Self::Output {
        self.context.v128_all_true_op(self.masm, |masm, src, dst| {
            masm.v128_all_true(src, writable!(dst), OperandSize::S32)
        })
    }

    fn visit_i64x2_all_true(&mut self) -> Self::Output {
        self.context.v128_all_true_op(self.masm, |masm, src, dst| {
            masm.v128_all_true(src, writable!(dst), OperandSize::S64)
        })
    }

    fn visit_i8x16_bitmask(&mut self) -> Self::Output {
        self.context.v128_bitmask_op(self.masm, |masm, src, dst| {
            masm.v128_bitmask(src, writable!(dst), OperandSize::S8)
        })
    }

    fn visit_i16x8_bitmask(&mut self) -> Self::Output {
        self.context.v128_bitmask_op(self.masm, |masm, src, dst| {
            masm.v128_bitmask(src, writable!(dst), OperandSize::S16)
        })
    }

    fn visit_i32x4_bitmask(&mut self) -> Self::Output {
        self.context.v128_bitmask_op(self.masm, |masm, src, dst| {
            masm.v128_bitmask(src, writable!(dst), OperandSize::S32)
        })
    }

    fn visit_i64x2_bitmask(&mut self) -> Self::Output {
        self.context.v128_bitmask_op(self.masm, |masm, src, dst| {
            masm.v128_bitmask(src, writable!(dst), OperandSize::S64)
        })
    }

    fn visit_i32x4_trunc_sat_f32x4_s(&mut self) -> Self::Output {
        self.masm
            .v128_trunc(&mut self.context, V128TruncKind::I32x4FromF32x4S)
    }

    fn visit_i32x4_trunc_sat_f32x4_u(&mut self) -> Self::Output {
        self.masm
            .v128_trunc(&mut self.context, V128TruncKind::I32x4FromF32x4U)
    }

    fn visit_i32x4_trunc_sat_f64x2_s_zero(&mut self) -> Self::Output {
        self.masm
            .v128_trunc(&mut self.context, V128TruncKind::I32x4FromF64x2SZero)
    }

    fn visit_i32x4_trunc_sat_f64x2_u_zero(&mut self) -> Self::Output {
        self.masm
            .v128_trunc(&mut self.context, V128TruncKind::I32x4FromF64x2UZero)
    }

    fn visit_i16x8_min_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_min(src, dst, writable!(dst), V128MinKind::I16x8S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_dot_i16x8_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_dot(dst, src, writable!(dst))?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_popcnt(&mut self) -> Self::Output {
        self.masm.v128_popcnt(&mut self.context)
    }

    fn visit_i8x16_avgr_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, size| {
                masm.v128_avgr(dst, src, writable!(dst), size)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_min_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_min(src, dst, writable!(dst), V128MinKind::I32x4S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_min_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_min(src, dst, writable!(dst), V128MinKind::I8x16U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_avgr_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, size| {
                masm.v128_avgr(dst, src, writable!(dst), size)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_min_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_min(src, dst, writable!(dst), V128MinKind::I16x8U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_min_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_min(src, dst, writable!(dst), V128MinKind::I32x4U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_max_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_max(src, dst, writable!(dst), V128MaxKind::I8x16S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_max_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_max(src, dst, writable!(dst), V128MaxKind::I16x8S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_max_s(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_max(src, dst, writable!(dst), V128MaxKind::I32x4S)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i8x16_max_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S8, |masm, dst, src, _size| {
                masm.v128_max(src, dst, writable!(dst), V128MaxKind::I8x16U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_max_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S16, |masm, dst, src, _size| {
                masm.v128_max(src, dst, writable!(dst), V128MaxKind::I16x8U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i32x4_max_u(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_max(src, dst, writable!(dst), V128MaxKind::I32x4U)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_i16x8_extmul_low_i8x16_s(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::LowI8x16S)
    }

    fn visit_i32x4_extmul_low_i16x8_s(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::LowI16x8S)
    }

    fn visit_i64x2_extmul_low_i32x4_s(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::LowI32x4S)
    }

    fn visit_i16x8_extmul_low_i8x16_u(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::LowI8x16U)
    }

    fn visit_i32x4_extmul_low_i16x8_u(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::LowI16x8U)
    }

    fn visit_i64x2_extmul_low_i32x4_u(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::LowI32x4U)
    }

    fn visit_i16x8_extmul_high_i8x16_u(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::HighI8x16U)
    }

    fn visit_i32x4_extmul_high_i16x8_u(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::HighI16x8U)
    }

    fn visit_i64x2_extmul_high_i32x4_u(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::HighI32x4U)
    }

    fn visit_i16x8_extmul_high_i8x16_s(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::HighI8x16S)
    }

    fn visit_i32x4_extmul_high_i16x8_s(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::HighI16x8S)
    }

    fn visit_i64x2_extmul_high_i32x4_s(&mut self) -> Self::Output {
        self.masm
            .v128_extmul(&mut self.context, V128ExtMulKind::HighI32x4S)
    }

    fn visit_i16x8_extadd_pairwise_i8x16_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, op| {
            masm.v128_extadd_pairwise(op, writable!(op), V128ExtAddKind::I8x16S)?;
            Ok(TypedReg::v128(op))
        })
    }

    fn visit_i16x8_extadd_pairwise_i8x16_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, op| {
            masm.v128_extadd_pairwise(op, writable!(op), V128ExtAddKind::I8x16U)?;
            Ok(TypedReg::v128(op))
        })
    }

    fn visit_i32x4_extadd_pairwise_i16x8_s(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, op| {
            masm.v128_extadd_pairwise(op, writable!(op), V128ExtAddKind::I16x8S)?;
            Ok(TypedReg::v128(op))
        })
    }

    fn visit_i32x4_extadd_pairwise_i16x8_u(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, op| {
            masm.v128_extadd_pairwise(op, writable!(op), V128ExtAddKind::I16x8U)?;
            Ok(TypedReg::v128(op))
        })
    }

    fn visit_f32x4_add(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_add(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_add(dst, src, writable!(dst), V128AddKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_sub(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_sub(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_sub(dst, src, writable!(dst), V128SubKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_mul(&mut self) -> Self::Output {
        self.masm.v128_mul(&mut self.context, V128MulKind::F32x4)
    }

    fn visit_f64x2_mul(&mut self) -> Self::Output {
        self.masm.v128_mul(&mut self.context, V128MulKind::F64x2)
    }

    fn visit_f32x4_div(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, size| {
                masm.v128_div(dst, src, writable!(dst), size)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_div(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, size| {
                masm.v128_div(dst, src, writable!(dst), size)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_neg(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_neg(writable!(reg), V128NegKind::F32x4)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f32x4_ceil(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_ceil(reg, writable!(reg), OperandSize::S32)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f64x2_neg(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_neg(writable!(reg), V128NegKind::F64x2)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f64x2_ceil(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_ceil(reg, writable!(reg), OperandSize::S64)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f32x4_sqrt(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_sqrt(reg, writable!(reg), OperandSize::S32)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f32x4_floor(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_floor(reg, writable!(reg), OperandSize::S32)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f64x2_sqrt(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_sqrt(reg, writable!(reg), OperandSize::S64)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f64x2_floor(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_floor(reg, writable!(reg), OperandSize::S64)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f32x4_nearest(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_nearest(reg, writable!(reg), OperandSize::S32)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f64x2_nearest(&mut self) -> Self::Output {
        self.context.unop(self.masm, |masm, reg| {
            masm.v128_nearest(reg, writable!(reg), OperandSize::S64)?;
            Ok(TypedReg::v128(reg))
        })
    }

    fn visit_f32x4_trunc(&mut self) -> Self::Output {
        self.masm
            .v128_trunc(&mut self.context, V128TruncKind::F32x4)
    }

    fn visit_f64x2_trunc(&mut self) -> Self::Output {
        self.masm
            .v128_trunc(&mut self.context, V128TruncKind::F64x2)
    }

    fn visit_v128_load32_zero(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::VectorZero(OperandSize::S32),
        )
    }

    fn visit_v128_load64_zero(&mut self, memarg: MemArg) -> Self::Output {
        self.emit_wasm_load(
            &memarg,
            WasmValType::V128,
            LoadKind::VectorZero(OperandSize::S64),
        )
    }

    fn visit_f32x4_pmin(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, size| {
                masm.v128_pmin(dst, src, writable!(dst), size)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_pmin(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, size| {
                masm.v128_pmin(dst, src, writable!(dst), size)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_pmax(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, size| {
                masm.v128_pmax(dst, src, writable!(dst), size)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_pmax(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, size| {
                masm.v128_pmax(dst, src, writable!(dst), size)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_min(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_min(dst, src, writable!(dst), V128MinKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_min(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_min(dst, src, writable!(dst), V128MinKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f32x4_max(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S32, |masm, dst, src, _size| {
                masm.v128_max(dst, src, writable!(dst), V128MaxKind::F32x4)?;
                Ok(TypedReg::v128(dst))
            })
    }

    fn visit_f64x2_max(&mut self) -> Self::Output {
        self.context
            .binop(self.masm, OperandSize::S64, |masm, dst, src, _size| {
                masm.v128_max(dst, src, writable!(dst), V128MaxKind::F64x2)?;
                Ok(TypedReg::v128(dst))
            })
    }

    wasmparser::for_each_visit_simd_operator!(def_unsupported);
}

impl<'a, 'translation, 'data, M> CodeGen<'a, 'translation, 'data, M, Emission>
where
    M: MacroAssembler,
{
    fn cmp_i32s(&mut self, kind: IntCmpKind) -> Result<()> {
        self.context.i32_binop(self.masm, |masm, dst, src, size| {
            masm.cmp_with_set(writable!(dst), src, kind, size)?;
            Ok(TypedReg::i32(dst))
        })
    }

    fn cmp_i64s(&mut self, kind: IntCmpKind) -> Result<()> {
        self.context
            .i64_binop(self.masm, move |masm, dst, src, size| {
                masm.cmp_with_set(writable!(dst), src, kind, size)?;
                Ok(TypedReg::i32(dst)) // Return value for comparisons is an `i32`.
            })
    }
}

impl TryFrom<WasmValType> for OperandSize {
    type Error = anyhow::Error;
    fn try_from(ty: WasmValType) -> Result<OperandSize> {
        let ty = match ty {
            WasmValType::I32 | WasmValType::F32 => OperandSize::S32,
            WasmValType::I64 | WasmValType::F64 => OperandSize::S64,
            WasmValType::V128 => OperandSize::S128,
            WasmValType::Ref(rt) => {
                match rt.heap_type {
                    // TODO: Hardcoded size, assuming 64-bit support only. Once
                    // Wasmtime supports 32-bit architectures, this will need
                    // to be updated in such a way that the calculation of the
                    // OperandSize will depend on the target's  pointer size.
                    WasmHeapType::Func => OperandSize::S64,
                    WasmHeapType::Extern => OperandSize::S64,
                    _ => bail!(CodeGenError::unsupported_wasm_type()),
                }
            }
        };
        Ok(ty)
    }
}
