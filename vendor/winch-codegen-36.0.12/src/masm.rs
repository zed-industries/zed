use crate::abi::{self, LocalSlot, align_to};
use crate::codegen::{CodeGenContext, Emission, FuncEnv};
use crate::isa::{
    CallingConvention,
    reg::{Reg, RegClass, WritableReg, writable},
};
use anyhow::Result;
use cranelift_codegen::{
    Final, MachBufferFinalized, MachLabel,
    binemit::CodeOffset,
    ir::{Endianness, MemFlags, RelSourceLoc, SourceLoc, UserExternalNameRef},
};
use std::{fmt::Debug, ops::Range};
use wasmtime_environ::{PtrSize, WasmHeapType, WasmRefType, WasmValType};

pub(crate) use cranelift_codegen::ir::TrapCode;

#[derive(Eq, PartialEq)]
pub(crate) enum DivKind {
    /// Signed division.
    Signed,
    /// Unsigned division.
    Unsigned,
}

/// Represents the `memory.atomic.wait*` kind.
#[derive(Debug, Clone, Copy)]
pub(crate) enum AtomicWaitKind {
    Wait32,
    Wait64,
}

/// Remainder kind.
#[derive(Copy, Clone)]
pub(crate) enum RemKind {
    /// Signed remainder.
    Signed,
    /// Unsigned remainder.
    Unsigned,
}

impl RemKind {
    pub fn is_signed(&self) -> bool {
        matches!(self, Self::Signed)
    }
}

/// Kinds of vector min operation supported by WebAssembly.
pub(crate) enum V128MinKind {
    /// 4 lanes of 32-bit floats.
    F32x4,
    /// 2 lanes of 64-bit floats.
    F64x2,
    /// 16 lanes of signed 8-bit integers.
    I8x16S,
    /// 16 lanes of unsigned 8-bit integers.
    I8x16U,
    /// 8 lanes of signed 16-bit integers.
    I16x8S,
    /// 8 lanes of unsigned 16-bit integers.
    I16x8U,
    /// 4 lanes of signed 32-bit integers.
    I32x4S,
    /// 4 lanes of unsigned 32-bit integers.
    I32x4U,
}

impl V128MinKind {
    /// The size of each lane.
    pub(crate) fn lane_size(&self) -> OperandSize {
        match self {
            Self::F32x4 | Self::I32x4S | Self::I32x4U => OperandSize::S32,
            Self::F64x2 => OperandSize::S64,
            Self::I8x16S | Self::I8x16U => OperandSize::S8,
            Self::I16x8S | Self::I16x8U => OperandSize::S16,
        }
    }
}

/// Kinds of vector max operation supported by WebAssembly.
pub(crate) enum V128MaxKind {
    /// 4 lanes of 32-bit floats.
    F32x4,
    /// 2 lanes of 64-bit floats.
    F64x2,
    /// 16 lanes of signed 8-bit integers.
    I8x16S,
    /// 16 lanes of unsigned 8-bit integers.
    I8x16U,
    /// 8 lanes of signed 16-bit integers.
    I16x8S,
    /// 8 lanes of unsigned 16-bit integers.
    I16x8U,
    /// 4 lanes of signed 32-bit integers.
    I32x4S,
    /// 4 lanes of unsigned 32-bit integers.
    I32x4U,
}

impl V128MaxKind {
    /// The size of each lane.
    pub(crate) fn lane_size(&self) -> OperandSize {
        match self {
            Self::F32x4 | Self::I32x4S | Self::I32x4U => OperandSize::S32,
            Self::F64x2 => OperandSize::S64,
            Self::I8x16S | Self::I8x16U => OperandSize::S8,
            Self::I16x8S | Self::I16x8U => OperandSize::S16,
        }
    }
}

#[derive(Eq, PartialEq)]
pub(crate) enum MulWideKind {
    Signed,
    Unsigned,
}

/// Type of operation for a read-modify-write instruction.
pub(crate) enum RmwOp {
    Add,
    Sub,
    Xchg,
    And,
    Or,
    Xor,
}

/// The direction to perform the memory move.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum MemMoveDirection {
    /// From high memory addresses to low memory addresses.
    /// Invariant: the source location is closer to the FP than the destination
    /// location, which will be closer to the SP.
    HighToLow,
    /// From low memory addresses to high memory addresses.
    /// Invariant: the source location is closer to the SP than the destination
    /// location, which will be closer to the FP.
    LowToHigh,
}

/// Classifies how to treat float-to-int conversions.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum TruncKind {
    /// Saturating conversion. If the source value is greater than the maximum
    /// value of the destination type, the result is clamped to the
    /// destination maximum value.
    Checked,
    /// An exception is raised if the source value is greater than the maximum
    /// value of the destination type.
    Unchecked,
}

impl TruncKind {
    /// Returns true if the truncation kind is checked.
    pub(crate) fn is_checked(&self) -> bool {
        *self == TruncKind::Checked
    }

    /// Returns `true` if the trunc kind is [`Unchecked`].
    ///
    /// [`Unchecked`]: TruncKind::Unchecked
    #[must_use]
    pub(crate) fn is_unchecked(&self) -> bool {
        matches!(self, Self::Unchecked)
    }
}

/// Representation of the stack pointer offset.
#[derive(Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Default)]
pub struct SPOffset(u32);

impl SPOffset {
    pub fn from_u32(offs: u32) -> Self {
        Self(offs)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// A stack slot.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StackSlot {
    /// The location of the slot, relative to the stack pointer.
    pub offset: SPOffset,
    /// The size of the slot, in bytes.
    pub size: u32,
}

impl StackSlot {
    pub fn new(offs: SPOffset, size: u32) -> Self {
        Self { offset: offs, size }
    }
}

pub trait ScratchType {
    /// Derive the register class from the scratch register type.
    fn reg_class() -> RegClass;
}

/// A scratch register type of integer class.
pub struct IntScratch;
/// A scratch register type of floating point class.
pub struct FloatScratch;

impl ScratchType for IntScratch {
    fn reg_class() -> RegClass {
        RegClass::Int
    }
}

impl ScratchType for FloatScratch {
    fn reg_class() -> RegClass {
        RegClass::Float
    }
}

/// A scratch register scope.
pub struct Scratch(Reg);

impl Scratch {
    pub fn new(r: Reg) -> Self {
        Self(r)
    }

    #[inline]
    pub fn inner(&self) -> Reg {
        self.0
    }

    #[inline]
    pub fn writable(&self) -> WritableReg {
        writable!(self.0)
    }
}

/// Kinds of integer binary comparison in WebAssembly. The [`MacroAssembler`]
/// implementation for each ISA is responsible for emitting the correct
/// sequence of instructions when lowering to machine code.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum IntCmpKind {
    /// Equal.
    Eq,
    /// Not equal.
    Ne,
    /// Signed less than.
    LtS,
    /// Unsigned less than.
    LtU,
    /// Signed greater than.
    GtS,
    /// Unsigned greater than.
    GtU,
    /// Signed less than or equal.
    LeS,
    /// Unsigned less than or equal.
    LeU,
    /// Signed greater than or equal.
    GeS,
    /// Unsigned greater than or equal.
    GeU,
}

/// Kinds of float binary comparison in WebAssembly. The [`MacroAssembler`]
/// implementation for each ISA is responsible for emitting the correct
/// sequence of instructions when lowering code.
#[derive(Debug)]
pub(crate) enum FloatCmpKind {
    /// Equal.
    Eq,
    /// Not equal.
    Ne,
    /// Less than.
    Lt,
    /// Greater than.
    Gt,
    /// Less than or equal.
    Le,
    /// Greater than or equal.
    Ge,
}

/// Kinds of shifts in WebAssembly.The [`masm`] implementation for each ISA is
/// responsible for emitting the correct sequence of instructions when
/// lowering to machine code.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ShiftKind {
    /// Left shift.
    Shl,
    /// Signed right shift.
    ShrS,
    /// Unsigned right shift.
    ShrU,
    /// Left rotate.
    Rotl,
    /// Right rotate.
    Rotr,
}

/// Kinds of extends in WebAssembly. Each MacroAssembler implementation
/// is responsible for emitting the correct sequence of instructions when
/// lowering to machine code.
#[derive(Copy, Clone)]
pub(crate) enum ExtendKind {
    Signed(Extend<Signed>),
    Unsigned(Extend<Zero>),
}

#[derive(Copy, Clone)]
pub(crate) enum Signed {}
#[derive(Copy, Clone)]
pub(crate) enum Zero {}

pub(crate) trait ExtendType {}

impl ExtendType for Signed {}
impl ExtendType for Zero {}

#[derive(Copy, Clone)]
pub(crate) enum Extend<T: ExtendType> {
    /// 8 to 32 bit extend.
    I32Extend8,
    /// 16 to 32 bit extend.
    I32Extend16,
    /// 8 to 64 bit extend.
    I64Extend8,
    /// 16 to 64 bit extend.
    I64Extend16,
    /// 32 to 64 bit extend.
    I64Extend32,

    /// Variant to hold the kind of extend marker.
    ///
    /// This is `Signed` or `Zero`, that are empty enums, which means that this variant cannot be
    /// constructed.
    __Kind(T),
}

impl From<Extend<Zero>> for ExtendKind {
    fn from(value: Extend<Zero>) -> Self {
        ExtendKind::Unsigned(value)
    }
}

impl<T: ExtendType> Extend<T> {
    pub fn from_size(&self) -> OperandSize {
        match self {
            Extend::I32Extend8 | Extend::I64Extend8 => OperandSize::S8,
            Extend::I32Extend16 | Extend::I64Extend16 => OperandSize::S16,
            Extend::I64Extend32 => OperandSize::S32,
            Extend::__Kind(_) => unreachable!(),
        }
    }

    pub fn to_size(&self) -> OperandSize {
        match self {
            Extend::I32Extend8 | Extend::I32Extend16 => OperandSize::S32,
            Extend::I64Extend8 | Extend::I64Extend16 | Extend::I64Extend32 => OperandSize::S64,
            Extend::__Kind(_) => unreachable!(),
        }
    }

    pub fn from_bits(&self) -> u8 {
        self.from_size().num_bits()
    }

    pub fn to_bits(&self) -> u8 {
        self.to_size().num_bits()
    }
}

impl From<Extend<Signed>> for ExtendKind {
    fn from(value: Extend<Signed>) -> Self {
        ExtendKind::Signed(value)
    }
}

impl ExtendKind {
    pub fn signed(&self) -> bool {
        match self {
            Self::Signed(_) => true,
            _ => false,
        }
    }

    pub fn from_bits(&self) -> u8 {
        match self {
            Self::Signed(s) => s.from_bits(),
            Self::Unsigned(u) => u.from_bits(),
        }
    }

    pub fn to_bits(&self) -> u8 {
        match self {
            Self::Signed(s) => s.to_bits(),
            Self::Unsigned(u) => u.to_bits(),
        }
    }
}

/// Kinds of vector load and extends in WebAssembly. Each MacroAssembler
/// implementation is responsible for emitting the correct sequence of
/// instructions when lowering to machine code.
#[derive(Copy, Clone)]
pub(crate) enum V128LoadExtendKind {
    /// Sign extends eight 8 bit integers to eight 16 bit lanes.
    E8x8S,
    /// Zero extends eight 8 bit integers to eight 16 bit lanes.
    E8x8U,
    /// Sign extends four 16 bit integers to four 32 bit lanes.
    E16x4S,
    /// Zero extends four 16 bit integers to four 32 bit lanes.
    E16x4U,
    /// Sign extends two 32 bit integers to two 64 bit lanes.
    E32x2S,
    /// Zero extends two 32 bit integers to two 64 bit lanes.
    E32x2U,
}

/// Kinds of splat loads supported by WebAssembly.
pub(crate) enum SplatLoadKind {
    /// 8 bits.
    S8,
    /// 16 bits.
    S16,
    /// 32 bits.
    S32,
    /// 64 bits.
    S64,
}

/// Kinds of splat supported by WebAssembly.
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub(crate) enum SplatKind {
    /// 8 bit integer.
    I8x16,
    /// 16 bit integer.
    I16x8,
    /// 32 bit integer.
    I32x4,
    /// 64 bit integer.
    I64x2,
    /// 32 bit float.
    F32x4,
    /// 64 bit float.
    F64x2,
}

impl SplatKind {
    /// The lane size to use for different kinds of splats.
    pub(crate) fn lane_size(&self) -> OperandSize {
        match self {
            SplatKind::I8x16 => OperandSize::S8,
            SplatKind::I16x8 => OperandSize::S16,
            SplatKind::I32x4 | SplatKind::F32x4 => OperandSize::S32,
            SplatKind::I64x2 | SplatKind::F64x2 => OperandSize::S64,
        }
    }
}

/// Kinds of extract lane supported by WebAssembly.
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub(crate) enum ExtractLaneKind {
    /// 16 lanes of 8-bit integers sign extended to 32-bits.
    I8x16S,
    /// 16 lanes of 8-bit integers zero extended to 32-bits.
    I8x16U,
    /// 8 lanes of 16-bit integers sign extended to 32-bits.
    I16x8S,
    /// 8 lanes of 16-bit integers zero extended to 32-bits.
    I16x8U,
    /// 4 lanes of 32-bit integers.
    I32x4,
    /// 2 lanes of 64-bit integers.
    I64x2,
    /// 4 lanes of 32-bit floats.
    F32x4,
    /// 2 lanes of 64-bit floats.
    F64x2,
}

impl ExtractLaneKind {
    /// The lane size to use for different kinds of extract lane kinds.
    pub(crate) fn lane_size(&self) -> OperandSize {
        match self {
            ExtractLaneKind::I8x16S | ExtractLaneKind::I8x16U => OperandSize::S8,
            ExtractLaneKind::I16x8S | ExtractLaneKind::I16x8U => OperandSize::S16,
            ExtractLaneKind::I32x4 | ExtractLaneKind::F32x4 => OperandSize::S32,
            ExtractLaneKind::I64x2 | ExtractLaneKind::F64x2 => OperandSize::S64,
        }
    }
}

impl From<ExtractLaneKind> for Extend<Signed> {
    fn from(value: ExtractLaneKind) -> Self {
        match value {
            ExtractLaneKind::I8x16S => Extend::I32Extend8,
            ExtractLaneKind::I16x8S => Extend::I32Extend16,
            _ => unimplemented!(),
        }
    }
}

/// Kinds of replace lane supported by WebAssembly.
pub(crate) enum ReplaceLaneKind {
    /// 16 lanes of 8 bit integers.
    I8x16,
    /// 8 lanes of 16 bit integers.
    I16x8,
    /// 4 lanes of 32 bit integers.
    I32x4,
    /// 2 lanes of 64 bit integers.
    I64x2,
    /// 4 lanes of 32 bit floats.
    F32x4,
    /// 2 lanes of 64 bit floats.
    F64x2,
}

impl ReplaceLaneKind {
    /// The lane size to use for different kinds of replace lane kinds.
    pub(crate) fn lane_size(&self) -> OperandSize {
        match self {
            ReplaceLaneKind::I8x16 => OperandSize::S8,
            ReplaceLaneKind::I16x8 => OperandSize::S16,
            ReplaceLaneKind::I32x4 => OperandSize::S32,
            ReplaceLaneKind::I64x2 => OperandSize::S64,
            ReplaceLaneKind::F32x4 => OperandSize::S32,
            ReplaceLaneKind::F64x2 => OperandSize::S64,
        }
    }
}

/// Kinds of behavior supported by Wasm loads.
pub(crate) enum LoadKind {
    /// Load the entire bytes of the operand size without any modifications.
    Operand(OperandSize),
    /// Atomic load, with optional scalar extend.
    Atomic(OperandSize, Option<ExtendKind>),
    /// Duplicate value into vector lanes.
    Splat(SplatLoadKind),
    /// Scalar (non-vector) extend.
    ScalarExtend(ExtendKind),
    /// Vector extend.
    VectorExtend(V128LoadExtendKind),
    /// Load content into select lane.
    VectorLane(LaneSelector),
    /// Load a single element into the lowest bits of a vector and initialize
    /// all other bits to zero.
    VectorZero(OperandSize),
}

impl LoadKind {
    /// Returns the [`OperandSize`] used in the load operation.
    pub(crate) fn derive_operand_size(&self) -> OperandSize {
        match self {
            Self::ScalarExtend(extend) | Self::Atomic(_, Some(extend)) => {
                Self::operand_size_for_scalar(extend)
            }
            Self::VectorExtend(_) => OperandSize::S64,
            Self::Splat(kind) => Self::operand_size_for_splat(kind),
            Self::Operand(size)
            | Self::Atomic(size, None)
            | Self::VectorLane(LaneSelector { size, .. })
            | Self::VectorZero(size) => *size,
        }
    }

    pub fn vector_lane(lane: u8, size: OperandSize) -> Self {
        Self::VectorLane(LaneSelector { lane, size })
    }

    fn operand_size_for_scalar(extend_kind: &ExtendKind) -> OperandSize {
        match extend_kind {
            ExtendKind::Signed(s) => s.from_size(),
            ExtendKind::Unsigned(u) => u.from_size(),
        }
    }

    fn operand_size_for_splat(kind: &SplatLoadKind) -> OperandSize {
        match kind {
            SplatLoadKind::S8 => OperandSize::S8,
            SplatLoadKind::S16 => OperandSize::S16,
            SplatLoadKind::S32 => OperandSize::S32,
            SplatLoadKind::S64 => OperandSize::S64,
        }
    }

    pub(crate) fn is_atomic(&self) -> bool {
        matches!(self, Self::Atomic(_, _))
    }
}

/// Kinds of behavior supported by Wasm loads.
#[derive(Copy, Clone)]
pub enum StoreKind {
    /// Store the entire bytes of the operand size without any modifications.
    Operand(OperandSize),
    /// Store the entire bytes of the operand size without any modifications, atomically.
    Atomic(OperandSize),
    /// Store the content of selected lane.
    VectorLane(LaneSelector),
}

impl StoreKind {
    pub fn vector_lane(lane: u8, size: OperandSize) -> Self {
        Self::VectorLane(LaneSelector { lane, size })
    }
}

#[derive(Copy, Clone)]
pub struct LaneSelector {
    pub lane: u8,
    pub size: OperandSize,
}

/// Types of vector integer to float conversions supported by WebAssembly.
pub(crate) enum V128ConvertKind {
    /// 4 lanes of signed 32-bit integers to 4 lanes of 32-bit floats.
    I32x4S,
    /// 4 lanes of unsigned 32-bit integers to 4 lanes of 32-bit floats.
    I32x4U,
    /// 4 lanes of signed 32-bit integers to low bits of 2 lanes of 64-bit
    /// floats.
    I32x4LowS,
    /// 4 lanes of unsigned 32-bit integers to low bits of 2 lanes of 64-bit
    /// floats.
    I32x4LowU,
}

impl V128ConvertKind {
    pub(crate) fn src_lane_size(&self) -> OperandSize {
        match self {
            V128ConvertKind::I32x4S
            | V128ConvertKind::I32x4U
            | V128ConvertKind::I32x4LowS
            | V128ConvertKind::I32x4LowU => OperandSize::S32,
        }
    }

    pub(crate) fn dst_lane_size(&self) -> OperandSize {
        match self {
            V128ConvertKind::I32x4S | V128ConvertKind::I32x4U => OperandSize::S32,
            V128ConvertKind::I32x4LowS | V128ConvertKind::I32x4LowU => OperandSize::S64,
        }
    }
}

/// Kinds of vector narrowing operations supported by WebAssembly.
pub(crate) enum V128NarrowKind {
    /// Narrow 8 lanes of 16-bit integers to 16 lanes of 8-bit integers using
    /// signed saturation.
    I16x8S,
    /// Narrow 8 lanes of 16-bit integers to 16 lanes of 8-bit integers using
    /// unsigned saturation.
    I16x8U,
    /// Narrow 4 lanes of 32-bit integers to 8 lanes of 16-bit integers using
    /// signed saturation.
    I32x4S,
    /// Narrow 4 lanes of 32-bit integers to 8 lanes of 16-bit integers using
    /// unsigned saturation.
    I32x4U,
}

impl V128NarrowKind {
    /// Return the size of the destination lanes.
    pub(crate) fn dst_lane_size(&self) -> OperandSize {
        match self {
            Self::I16x8S | Self::I16x8U => OperandSize::S8,
            Self::I32x4S | Self::I32x4U => OperandSize::S16,
        }
    }
}

/// Kinds of vector extending operations supported by WebAssembly.
#[derive(Debug, Copy, Clone)]
pub(crate) enum V128ExtendKind {
    /// Low half of i8x16 sign extended.
    LowI8x16S,
    /// High half of i8x16 sign extended.
    HighI8x16S,
    /// Low half of i8x16 zero extended.
    LowI8x16U,
    /// High half of i8x16 zero extended.
    HighI8x16U,
    /// Low half of i16x8 sign extended.
    LowI16x8S,
    /// High half of i16x8 sign extended.
    HighI16x8S,
    /// Low half of i16x8 zero extended.
    LowI16x8U,
    /// High half of i16x8 zero extended.
    HighI16x8U,
    /// Low half of i32x4 sign extended.
    LowI32x4S,
    /// High half of i32x4 sign extended.
    HighI32x4S,
    /// Low half of i32x4 zero extended.
    LowI32x4U,
    /// High half of i32x4 zero extended.
    HighI32x4U,
}

impl V128ExtendKind {
    /// The size of the source's lanes.
    pub(crate) fn src_lane_size(&self) -> OperandSize {
        match self {
            Self::LowI8x16S | Self::LowI8x16U | Self::HighI8x16S | Self::HighI8x16U => {
                OperandSize::S8
            }
            Self::LowI16x8S | Self::LowI16x8U | Self::HighI16x8S | Self::HighI16x8U => {
                OperandSize::S16
            }
            Self::LowI32x4S | Self::LowI32x4U | Self::HighI32x4S | Self::HighI32x4U => {
                OperandSize::S32
            }
        }
    }
}

/// Kinds of vector equalities and non-equalities supported by WebAssembly.
pub(crate) enum VectorEqualityKind {
    /// 16 lanes of 8 bit integers.
    I8x16,
    /// 8 lanes of 16 bit integers.
    I16x8,
    /// 4 lanes of 32 bit integers.
    I32x4,
    /// 2 lanes of 64 bit integers.
    I64x2,
    /// 4 lanes of 32 bit floats.
    F32x4,
    /// 2 lanes of 64 bit floats.
    F64x2,
}

impl VectorEqualityKind {
    /// Get the lane size to use.
    pub(crate) fn lane_size(&self) -> OperandSize {
        match self {
            Self::I8x16 => OperandSize::S8,
            Self::I16x8 => OperandSize::S16,
            Self::I32x4 | Self::F32x4 => OperandSize::S32,
            Self::I64x2 | Self::F64x2 => OperandSize::S64,
        }
    }
}

/// Kinds of vector comparisons supported by WebAssembly.
pub(crate) enum VectorCompareKind {
    /// 16 lanes of signed 8 bit integers.
    I8x16S,
    /// 16 lanes of unsigned 8 bit integers.
    I8x16U,
    /// 8 lanes of signed 16 bit integers.
    I16x8S,
    /// 8 lanes of unsigned 16 bit integers.
    I16x8U,
    /// 4 lanes of signed 32 bit integers.
    I32x4S,
    /// 4 lanes of unsigned 32 bit integers.
    I32x4U,
    /// 2 lanes of signed 64 bit integers.
    I64x2S,
    /// 4 lanes of 32 bit floats.
    F32x4,
    /// 2 lanes of 64 bit floats.
    F64x2,
}

impl VectorCompareKind {
    /// Get the lane size to use.
    pub(crate) fn lane_size(&self) -> OperandSize {
        match self {
            Self::I8x16S | Self::I8x16U => OperandSize::S8,
            Self::I16x8S | Self::I16x8U => OperandSize::S16,
            Self::I32x4S | Self::I32x4U | Self::F32x4 => OperandSize::S32,
            Self::I64x2S | Self::F64x2 => OperandSize::S64,
        }
    }
}

/// Kinds of vector absolute operations supported by WebAssembly.
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub(crate) enum V128AbsKind {
    /// 8 bit integers.
    I8x16,
    /// 16 bit integers.
    I16x8,
    /// 32 bit integers.
    I32x4,
    /// 64 bit integers.
    I64x2,
    /// 32 bit floats.
    F32x4,
    /// 64 bit floats.
    F64x2,
}

impl V128AbsKind {
    /// The lane size to use.
    pub(crate) fn lane_size(&self) -> OperandSize {
        match self {
            Self::I8x16 => OperandSize::S8,
            Self::I16x8 => OperandSize::S16,
            Self::I32x4 | Self::F32x4 => OperandSize::S32,
            Self::I64x2 | Self::F64x2 => OperandSize::S64,
        }
    }
}

/// Kinds of truncation for vectors supported by WebAssembly.
pub(crate) enum V128TruncKind {
    /// Truncates 4 lanes of 32-bit floats to nearest integral value.
    F32x4,
    /// Truncates 2 lanes of 64-bit floats to nearest integral value.
    F64x2,
    /// Integers from signed F32x4.
    I32x4FromF32x4S,
    /// Integers from unsigned F32x4.
    I32x4FromF32x4U,
    /// Integers from signed F64x2.
    I32x4FromF64x2SZero,
    /// Integers from unsigned F64x2.
    I32x4FromF64x2UZero,
}

impl V128TruncKind {
    /// The size of the source lanes.
    pub(crate) fn src_lane_size(&self) -> OperandSize {
        match self {
            V128TruncKind::F32x4
            | V128TruncKind::I32x4FromF32x4S
            | V128TruncKind::I32x4FromF32x4U => OperandSize::S32,
            V128TruncKind::F64x2
            | V128TruncKind::I32x4FromF64x2SZero
            | V128TruncKind::I32x4FromF64x2UZero => OperandSize::S64,
        }
    }

    /// The size of the destination lanes.
    pub(crate) fn dst_lane_size(&self) -> OperandSize {
        if let V128TruncKind::F64x2 = self {
            OperandSize::S64
        } else {
            OperandSize::S32
        }
    }
}

/// Kinds of vector addition supported by WebAssembly.
pub(crate) enum V128AddKind {
    /// 4 lanes of 32-bit floats wrapping.
    F32x4,
    /// 2 lanes of 64-bit floats wrapping.
    F64x2,
    /// 16 lanes of 8-bit integers wrapping.
    I8x16,
    /// 16 lanes of 8-bit integers signed saturating.
    I8x16SatS,
    /// 16 lanes of 8-bit integers unsigned saturating.
    I8x16SatU,
    /// 8 lanes of 16-bit integers wrapping.
    I16x8,
    /// 8 lanes of 16-bit integers signed saturating.
    I16x8SatS,
    /// 8 lanes of 16-bit integers unsigned saturating.
    I16x8SatU,
    /// 4 lanes of 32-bit integers wrapping.
    I32x4,
    /// 2 lanes of 64-bit integers wrapping.
    I64x2,
}

/// Kinds of vector subtraction supported by WebAssembly.
pub(crate) enum V128SubKind {
    /// 4 lanes of 32-bit floats wrapping.
    F32x4,
    /// 2 lanes of 64-bit floats wrapping.
    F64x2,
    /// 16 lanes of 8-bit integers wrapping.
    I8x16,
    /// 16 lanes of 8-bit integers signed saturating.
    I8x16SatS,
    /// 16 lanes of 8-bit integers unsigned saturating.
    I8x16SatU,
    /// 8 lanes of 16-bit integers wrapping.
    I16x8,
    /// 8 lanes of 16-bit integers signed saturating.
    I16x8SatS,
    /// 8 lanes of 16-bit integers unsigned saturating.
    I16x8SatU,
    /// 4 lanes of 32-bit integers wrapping.
    I32x4,
    /// 2 lanes of 64-bit integers wrapping.
    I64x2,
}

impl From<V128NegKind> for V128SubKind {
    fn from(value: V128NegKind) -> Self {
        match value {
            V128NegKind::I8x16 => Self::I8x16,
            V128NegKind::I16x8 => Self::I16x8,
            V128NegKind::I32x4 => Self::I32x4,
            V128NegKind::I64x2 => Self::I64x2,
            V128NegKind::F32x4 | V128NegKind::F64x2 => unimplemented!(),
        }
    }
}

/// Kinds of vector multiplication supported by WebAssembly.
pub(crate) enum V128MulKind {
    /// 4 lanes of 32-bit floats.
    F32x4,
    /// 2 lanes of 64-bit floats.
    F64x2,
    /// 8 lanes of 16-bit integers.
    I16x8,
    /// 4 lanes of 32-bit integers.
    I32x4,
    /// 2 lanes of 64-bit integers.
    I64x2,
}

/// Kinds of vector negation supported by WebAssembly.
#[derive(Copy, Clone)]
pub(crate) enum V128NegKind {
    /// 4 lanes of 32-bit floats.
    F32x4,
    /// 2 lanes of 64-bit floats.
    F64x2,
    /// 16 lanes of 8-bit integers.
    I8x16,
    /// 8 lanes of 16-bit integers.
    I16x8,
    /// 4 lanes of 32-bit integers.
    I32x4,
    /// 2 lanes of 64-bit integers.
    I64x2,
}

impl V128NegKind {
    /// The size of the lanes.
    pub(crate) fn lane_size(&self) -> OperandSize {
        match self {
            Self::F32x4 | Self::I32x4 => OperandSize::S32,
            Self::F64x2 | Self::I64x2 => OperandSize::S64,
            Self::I8x16 => OperandSize::S8,
            Self::I16x8 => OperandSize::S16,
        }
    }
}

/// Kinds of extended pairwise addition supported by WebAssembly.
pub(crate) enum V128ExtAddKind {
    /// 16 lanes of signed 8-bit integers.
    I8x16S,
    /// 16 lanes of unsigned 8-bit integers.
    I8x16U,
    /// 8 lanes of signed 16-bit integers.
    I16x8S,
    /// 8 lanes of unsigned 16-bit integers.
    I16x8U,
}

/// Kinds of vector extended multiplication supported by WebAssembly.
#[derive(Debug, Clone, Copy)]
pub(crate) enum V128ExtMulKind {
    LowI8x16S,
    HighI8x16S,
    LowI8x16U,
    HighI8x16U,
    LowI16x8S,
    HighI16x8S,
    LowI16x8U,
    HighI16x8U,
    LowI32x4S,
    HighI32x4S,
    LowI32x4U,
    HighI32x4U,
}

impl From<V128ExtMulKind> for V128ExtendKind {
    fn from(value: V128ExtMulKind) -> Self {
        match value {
            V128ExtMulKind::LowI8x16S => Self::LowI8x16S,
            V128ExtMulKind::HighI8x16S => Self::HighI8x16S,
            V128ExtMulKind::LowI8x16U => Self::LowI8x16U,
            V128ExtMulKind::HighI8x16U => Self::HighI8x16U,
            V128ExtMulKind::LowI16x8S => Self::LowI16x8S,
            V128ExtMulKind::HighI16x8S => Self::HighI16x8S,
            V128ExtMulKind::LowI16x8U => Self::LowI16x8U,
            V128ExtMulKind::HighI16x8U => Self::HighI16x8U,
            V128ExtMulKind::LowI32x4S => Self::LowI32x4S,
            V128ExtMulKind::HighI32x4S => Self::HighI32x4S,
            V128ExtMulKind::LowI32x4U => Self::LowI32x4U,
            V128ExtMulKind::HighI32x4U => Self::HighI32x4U,
        }
    }
}

impl From<V128ExtMulKind> for V128MulKind {
    fn from(value: V128ExtMulKind) -> Self {
        match value {
            V128ExtMulKind::LowI8x16S
            | V128ExtMulKind::HighI8x16S
            | V128ExtMulKind::LowI8x16U
            | V128ExtMulKind::HighI8x16U => Self::I16x8,
            V128ExtMulKind::LowI16x8S
            | V128ExtMulKind::HighI16x8S
            | V128ExtMulKind::LowI16x8U
            | V128ExtMulKind::HighI16x8U => Self::I32x4,
            V128ExtMulKind::LowI32x4S
            | V128ExtMulKind::HighI32x4S
            | V128ExtMulKind::LowI32x4U
            | V128ExtMulKind::HighI32x4U => Self::I64x2,
        }
    }
}

/// Operand size, in bits.
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub(crate) enum OperandSize {
    /// 8 bits.
    S8,
    /// 16 bits.
    S16,
    /// 32 bits.
    S32,
    /// 64 bits.
    S64,
    /// 128 bits.
    S128,
}

impl OperandSize {
    /// The number of bits in the operand.
    pub fn num_bits(&self) -> u8 {
        match self {
            OperandSize::S8 => 8,
            OperandSize::S16 => 16,
            OperandSize::S32 => 32,
            OperandSize::S64 => 64,
            OperandSize::S128 => 128,
        }
    }

    /// The number of bytes in the operand.
    pub fn bytes(&self) -> u32 {
        match self {
            Self::S8 => 1,
            Self::S16 => 2,
            Self::S32 => 4,
            Self::S64 => 8,
            Self::S128 => 16,
        }
    }

    /// The binary logarithm of the number of bits in the operand.
    pub fn log2(&self) -> u8 {
        match self {
            OperandSize::S8 => 3,
            OperandSize::S16 => 4,
            OperandSize::S32 => 5,
            OperandSize::S64 => 6,
            OperandSize::S128 => 7,
        }
    }

    /// Create an [`OperandSize`]  from the given number of bytes.
    pub fn from_bytes(bytes: u8) -> Self {
        use OperandSize::*;
        match bytes {
            4 => S32,
            8 => S64,
            16 => S128,
            _ => panic!("Invalid bytes {bytes} for OperandSize"),
        }
    }

    pub fn extend_to<T: ExtendType>(&self, to: Self) -> Option<Extend<T>> {
        match to {
            OperandSize::S32 => match self {
                OperandSize::S8 => Some(Extend::I32Extend8),
                OperandSize::S16 => Some(Extend::I32Extend16),
                _ => None,
            },
            OperandSize::S64 => match self {
                OperandSize::S8 => Some(Extend::I64Extend8),
                OperandSize::S16 => Some(Extend::I64Extend16),
                OperandSize::S32 => Some(Extend::I64Extend32),
                _ => None,
            },
            _ => None,
        }
    }

    /// The number of bits in the mantissa.
    ///
    /// Only implemented for floats.
    pub fn mantissa_bits(&self) -> u8 {
        match self {
            Self::S32 => 8,
            Self::S64 => 11,
            _ => unimplemented!(),
        }
    }
}

/// An abstraction over a register or immediate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum RegImm {
    /// A register.
    Reg(Reg),
    /// A tagged immediate argument.
    Imm(Imm),
}

/// An tagged representation of an immediate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Imm {
    /// I32 immediate.
    I32(u32),
    /// I64 immediate.
    I64(u64),
    /// F32 immediate.
    F32(u32),
    /// F64 immediate.
    F64(u64),
    /// V128 immediate.
    V128(i128),
}

impl Imm {
    /// Create a new I64 immediate.
    pub fn i64(val: i64) -> Self {
        Self::I64(val as u64)
    }

    /// Create a new I32 immediate.
    pub fn i32(val: i32) -> Self {
        Self::I32(val as u32)
    }

    /// Create a new F32 immediate.
    pub fn f32(bits: u32) -> Self {
        Self::F32(bits)
    }

    /// Create a new F64 immediate.
    pub fn f64(bits: u64) -> Self {
        Self::F64(bits)
    }

    /// Create a new V128 immediate.
    pub fn v128(bits: i128) -> Self {
        Self::V128(bits)
    }

    /// Convert the immediate to i32, if possible.
    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Self::I32(v) => Some(*v as i32),
            Self::I64(v) => i32::try_from(*v as i64).ok(),
            _ => None,
        }
    }

    /// Unwraps the underlying integer value as u64.
    /// # Panics
    /// This function panics if the underlying value can't be represented
    /// as u64.
    pub fn unwrap_as_u64(&self) -> u64 {
        match self {
            Self::I32(v) => *v as u64,
            Self::I64(v) => *v,
            Self::F32(v) => *v as u64,
            Self::F64(v) => *v,
            _ => unreachable!(),
        }
    }

    /// Get the operand size of the immediate.
    pub fn size(&self) -> OperandSize {
        match self {
            Self::I32(_) | Self::F32(_) => OperandSize::S32,
            Self::I64(_) | Self::F64(_) => OperandSize::S64,
            Self::V128(_) => OperandSize::S128,
        }
    }

    /// Get a little endian representation of the immediate.
    ///
    /// This method heap allocates and is intended to be used when adding
    /// values to the constant pool.
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Imm::I32(n) => n.to_le_bytes().to_vec(),
            Imm::I64(n) => n.to_le_bytes().to_vec(),
            Imm::F32(n) => n.to_le_bytes().to_vec(),
            Imm::F64(n) => n.to_le_bytes().to_vec(),
            Imm::V128(n) => n.to_le_bytes().to_vec(),
        }
    }
}

/// The location of the [VMcontext] used for function calls.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum VMContextLoc {
    /// Dynamic, stored in the given register.
    Reg(Reg),
    /// The pinned [VMContext] register.
    Pinned,
    /// A different VMContext is loaded at the provided offset from the current
    /// VMContext.
    OffsetFromPinned(u32),
}

/// The maximum number of context arguments currently used across the compiler.
pub(crate) const MAX_CONTEXT_ARGS: usize = 2;

/// Out-of-band special purpose arguments used for function call emission.
///
/// We cannot rely on the value stack for these values given that inserting
/// register or memory values at arbitrary locations of the value stack has the
/// potential to break the stack ordering principle, which states that older
/// values must always precede newer values, effectively simulating the order of
/// values in the machine stack.
/// The [ContextArgs] are meant to be resolved at every callsite; in some cases
/// it might be possible to construct it early on, but given that it might
/// contain allocatable registers, it's preferred to construct it in
/// [FnCall::emit].
#[derive(Clone, Debug)]
pub(crate) enum ContextArgs {
    /// A single context argument is required; the current pinned [VMcontext]
    /// register must be passed as the first argument of the function call.
    VMContext([VMContextLoc; 1]),
    /// The callee and caller context arguments are required. In this case, the
    /// callee context argument is usually stored into an allocatable register
    /// and the caller is always the current pinned [VMContext] pointer.
    CalleeAndCallerVMContext([VMContextLoc; MAX_CONTEXT_ARGS]),
}

impl ContextArgs {
    /// Construct a [ContextArgs] declaring the usage of the pinned [VMContext]
    /// register as both the caller and callee context arguments.
    pub fn pinned_callee_and_caller_vmctx() -> Self {
        Self::CalleeAndCallerVMContext([VMContextLoc::Pinned, VMContextLoc::Pinned])
    }

    /// Construct a [ContextArgs] that declares the usage of the pinned
    /// [VMContext] register as the only context argument.
    pub fn pinned_vmctx() -> Self {
        Self::VMContext([VMContextLoc::Pinned])
    }

    /// Construct a [ContextArgs] that declares the usage of a [VMContext] loaded
    /// indirectly from the pinned [VMContext] register as the only context
    /// argument.
    pub fn offset_from_pinned_vmctx(offset: u32) -> Self {
        Self::VMContext([VMContextLoc::OffsetFromPinned(offset)])
    }

    /// Construct a [ContextArgs] that declares a dynamic callee context and the
    /// pinned [VMContext] register as the context arguments.
    pub fn with_callee_and_pinned_caller(callee_vmctx: Reg) -> Self {
        Self::CalleeAndCallerVMContext([VMContextLoc::Reg(callee_vmctx), VMContextLoc::Pinned])
    }

    /// Get the length of the [ContextArgs].
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Get a slice of the context arguments.
    pub fn as_slice(&self) -> &[VMContextLoc] {
        match self {
            Self::VMContext(a) => a.as_slice(),
            Self::CalleeAndCallerVMContext(a) => a.as_slice(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum CalleeKind {
    /// A function call to a raw address.
    Indirect(Reg),
    /// A function call to a local function.
    Direct(UserExternalNameRef),
}

impl CalleeKind {
    /// Creates a callee kind from a register.
    pub fn indirect(reg: Reg) -> Self {
        Self::Indirect(reg)
    }

    /// Creates a direct callee kind from a function name.
    pub fn direct(name: UserExternalNameRef) -> Self {
        Self::Direct(name)
    }
}

impl RegImm {
    /// Register constructor.
    pub fn reg(r: Reg) -> Self {
        RegImm::Reg(r)
    }

    /// I64 immediate constructor.
    pub fn i64(val: i64) -> Self {
        RegImm::Imm(Imm::i64(val))
    }

    /// I32 immediate constructor.
    pub fn i32(val: i32) -> Self {
        RegImm::Imm(Imm::i32(val))
    }

    /// F32 immediate, stored using its bits representation.
    pub fn f32(bits: u32) -> Self {
        RegImm::Imm(Imm::f32(bits))
    }

    /// F64 immediate, stored using its bits representation.
    pub fn f64(bits: u64) -> Self {
        RegImm::Imm(Imm::f64(bits))
    }

    /// V128 immediate.
    pub fn v128(bits: i128) -> Self {
        RegImm::Imm(Imm::v128(bits))
    }
}

impl From<Reg> for RegImm {
    fn from(r: Reg) -> Self {
        Self::Reg(r)
    }
}

#[derive(Debug)]
pub enum RoundingMode {
    Nearest,
    Up,
    Down,
    Zero,
}

/// Memory flags for trusted loads/stores.
pub const TRUSTED_FLAGS: MemFlags = MemFlags::trusted();

/// Flags used for WebAssembly loads / stores.
/// Untrusted by default so we don't set `no_trap`.
/// We also ensure that the endianness is the right one for WebAssembly.
pub const UNTRUSTED_FLAGS: MemFlags = MemFlags::new().with_endianness(Endianness::Little);

/// Generic MacroAssembler interface used by the code generation.
///
/// The MacroAssembler trait aims to expose an interface, high-level enough,
/// so that each ISA can provide its own lowering to machine code. For example,
/// for WebAssembly operators that don't have a direct mapping to a machine
/// a instruction, the interface defines a signature matching the WebAssembly
/// operator, allowing each implementation to lower such operator entirely.
/// This approach attributes more responsibility to the MacroAssembler, but frees
/// the caller from concerning about assembling the right sequence of
/// instructions at the operator callsite.
///
/// The interface defaults to a three-argument form for binary operations;
/// this allows a natural mapping to instructions for RISC architectures,
/// that use three-argument form.
/// This approach allows for a more general interface that can be restricted
/// where needed, in the case of architectures that use a two-argument form.

pub(crate) trait MacroAssembler {
    /// The addressing mode.
    type Address: Copy + Debug;

    /// The pointer representation of the target ISA,
    /// used to access information from [`VMOffsets`].
    type Ptr: PtrSize;

    /// The ABI details of the target.
    type ABI: abi::ABI;

    /// Emit the function prologue.
    fn prologue(&mut self, vmctx: Reg) -> Result<()> {
        self.frame_setup()?;
        self.check_stack(vmctx)
    }

    /// Generate the frame setup sequence.
    fn frame_setup(&mut self) -> Result<()>;

    /// Generate the frame restore sequence.
    fn frame_restore(&mut self) -> Result<()>;

    /// Emit a stack check.
    fn check_stack(&mut self, vmctx: Reg) -> Result<()>;

    /// Emit the function epilogue.
    fn epilogue(&mut self) -> Result<()> {
        self.frame_restore()
    }

    /// Reserve stack space.
    fn reserve_stack(&mut self, bytes: u32) -> Result<()>;

    /// Free stack space.
    fn free_stack(&mut self, bytes: u32) -> Result<()>;

    /// Reset the stack pointer to the given offset;
    ///
    /// Used to reset the stack pointer to a given offset
    /// when dealing with unreachable code.
    fn reset_stack_pointer(&mut self, offset: SPOffset) -> Result<()>;

    /// Get the address of a local slot.
    fn local_address(&mut self, local: &LocalSlot) -> Result<Self::Address>;

    /// Constructs an address with an offset that is relative to the
    /// current position of the stack pointer (e.g. [sp + (sp_offset -
    /// offset)].
    fn address_from_sp(&self, offset: SPOffset) -> Result<Self::Address>;

    /// Constructs an address with an offset that is absolute to the
    /// current position of the stack pointer (e.g. [sp + offset].
    fn address_at_sp(&self, offset: SPOffset) -> Result<Self::Address>;

    /// Alias for [`Self::address_at_reg`] using the VMContext register as
    /// a base. The VMContext register is derived from the ABI type that is
    /// associated to the MacroAssembler.
    fn address_at_vmctx(&self, offset: u32) -> Result<Self::Address>;

    /// Construct an address that is absolute to the current position
    /// of the given register.
    fn address_at_reg(&self, reg: Reg, offset: u32) -> Result<Self::Address>;

    /// Emit a function call to either a local or external function.
    fn call(
        &mut self,
        stack_args_size: u32,
        f: impl FnMut(&mut Self) -> Result<(CalleeKind, CallingConvention)>,
    ) -> Result<u32>;

    /// Acquire a scratch register and execute the given callback.
    fn with_scratch<T: ScratchType, R>(&mut self, f: impl FnOnce(&mut Self, Scratch) -> R) -> R;

    /// Convenience wrapper over [`Self::with_scratch`], derives the register class
    /// for a particular Wasm value type.
    fn with_scratch_for<R>(
        &mut self,
        ty: WasmValType,
        f: impl FnOnce(&mut Self, Scratch) -> R,
    ) -> R {
        match ty {
            WasmValType::I32
            | WasmValType::I64
            | WasmValType::Ref(WasmRefType {
                heap_type: WasmHeapType::Func,
                ..
            }) => self.with_scratch::<IntScratch, _>(f),
            WasmValType::F32 | WasmValType::F64 | WasmValType::V128 => {
                self.with_scratch::<FloatScratch, _>(f)
            }
            _ => unimplemented!(),
        }
    }

    /// Get stack pointer offset.
    fn sp_offset(&self) -> Result<SPOffset>;

    /// Perform a stack store.
    fn store(&mut self, src: RegImm, dst: Self::Address, size: OperandSize) -> Result<()>;

    /// Alias for `MacroAssembler::store` with the operand size corresponding
    /// to the pointer size of the target.
    fn store_ptr(&mut self, src: Reg, dst: Self::Address) -> Result<()>;

    /// Perform a WebAssembly store.
    /// A WebAssembly store introduces several additional invariants compared to
    /// [Self::store], more precisely, it can implicitly trap, in certain
    /// circumstances, even if explicit bounds checks are elided, in that sense,
    /// we consider this type of load as untrusted. It can also differ with
    /// regards to the endianness depending on the target ISA. For this reason,
    /// [Self::wasm_store], should be explicitly used when emitting WebAssembly
    /// stores.
    fn wasm_store(&mut self, src: Reg, dst: Self::Address, store_kind: StoreKind) -> Result<()>;

    /// Perform a zero-extended stack load.
    fn load(&mut self, src: Self::Address, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Perform a WebAssembly load.
    /// A WebAssembly load introduces several additional invariants compared to
    /// [Self::load], more precisely, it can implicitly trap, in certain
    /// circumstances, even if explicit bounds checks are elided, in that sense,
    /// we consider this type of load as untrusted. It can also differ with
    /// regards to the endianness depending on the target ISA. For this reason,
    /// [Self::wasm_load], should be explicitly used when emitting WebAssembly
    /// loads.
    fn wasm_load(&mut self, src: Self::Address, dst: WritableReg, kind: LoadKind) -> Result<()>;

    /// Alias for `MacroAssembler::load` with the operand size corresponding
    /// to the pointer size of the target.
    fn load_ptr(&mut self, src: Self::Address, dst: WritableReg) -> Result<()>;

    /// Computes the effective address and stores the result in the destination
    /// register.
    fn compute_addr(
        &mut self,
        _src: Self::Address,
        _dst: WritableReg,
        _size: OperandSize,
    ) -> Result<()>;

    /// Pop a value from the machine stack into the given register.
    fn pop(&mut self, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Perform a move.
    fn mov(&mut self, dst: WritableReg, src: RegImm, size: OperandSize) -> Result<()>;

    /// Perform a conditional move.
    fn cmov(&mut self, dst: WritableReg, src: Reg, cc: IntCmpKind, size: OperandSize)
    -> Result<()>;

    /// Performs a memory move of bytes from src to dest.
    /// Bytes are moved in blocks of 8 bytes, where possible.
    fn memmove(
        &mut self,
        src: SPOffset,
        dst: SPOffset,
        bytes: u32,
        direction: MemMoveDirection,
    ) -> Result<()> {
        match direction {
            MemMoveDirection::LowToHigh => debug_assert!(dst.as_u32() < src.as_u32()),
            MemMoveDirection::HighToLow => debug_assert!(dst.as_u32() > src.as_u32()),
        }
        // At least 4 byte aligned.
        debug_assert!(bytes % 4 == 0);
        let mut remaining = bytes;
        let word_bytes = <Self::ABI as abi::ABI>::word_bytes();

        let word_bytes = word_bytes as u32;

        let mut dst_offs;
        let mut src_offs;
        match direction {
            MemMoveDirection::LowToHigh => {
                dst_offs = dst.as_u32() - bytes;
                src_offs = src.as_u32() - bytes;
                self.with_scratch::<IntScratch, _>(|masm, scratch| {
                    while remaining >= word_bytes {
                        remaining -= word_bytes;
                        dst_offs += word_bytes;
                        src_offs += word_bytes;

                        masm.load_ptr(
                            masm.address_from_sp(SPOffset::from_u32(src_offs))?,
                            scratch.writable(),
                        )?;
                        masm.store_ptr(
                            scratch.inner(),
                            masm.address_from_sp(SPOffset::from_u32(dst_offs))?,
                        )?;
                    }
                    anyhow::Ok(())
                })?;
            }
            MemMoveDirection::HighToLow => {
                // Go from the end to the beginning to handle overlapping addresses.
                src_offs = src.as_u32();
                dst_offs = dst.as_u32();
                self.with_scratch::<IntScratch, _>(|masm, scratch| {
                    while remaining >= word_bytes {
                        masm.load_ptr(
                            masm.address_from_sp(SPOffset::from_u32(src_offs))?,
                            scratch.writable(),
                        )?;
                        masm.store_ptr(
                            scratch.inner(),
                            masm.address_from_sp(SPOffset::from_u32(dst_offs))?,
                        )?;

                        remaining -= word_bytes;
                        src_offs -= word_bytes;
                        dst_offs -= word_bytes;
                    }
                    anyhow::Ok(())
                })?;
            }
        }

        if remaining > 0 {
            let half_word = word_bytes / 2;
            let ptr_size = OperandSize::from_bytes(half_word as u8);
            debug_assert!(remaining == half_word);
            // Need to move the offsets ahead in the `LowToHigh` case to
            // compensate for the initial subtraction of `bytes`.
            if direction == MemMoveDirection::LowToHigh {
                dst_offs += half_word;
                src_offs += half_word;
            }

            self.with_scratch::<IntScratch, _>(|masm, scratch| {
                masm.load(
                    masm.address_from_sp(SPOffset::from_u32(src_offs))?,
                    scratch.writable(),
                    ptr_size,
                )?;
                masm.store(
                    scratch.inner().into(),
                    masm.address_from_sp(SPOffset::from_u32(dst_offs))?,
                    ptr_size,
                )?;
                anyhow::Ok(())
            })?;
        }
        Ok(())
    }

    /// Perform add operation.
    fn add(&mut self, dst: WritableReg, lhs: Reg, rhs: RegImm, size: OperandSize) -> Result<()>;

    /// Perform add with unsigned extension.
    fn add_uextend(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Reg,
        from_size: OperandSize,
        size: OperandSize,
    ) -> Result<()>;

    /// Perform a checked unsigned integer addition, emitting the provided trap
    /// if the addition overflows.
    ///
    /// Note: This only accepts immediate operands. For register operands with
    /// proper extension, use add_uextend with manual overflow checking.
    fn checked_uadd(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Imm,
        size: OperandSize,
        trap: TrapCode,
    ) -> Result<()>;

    /// Perform subtraction operation.
    fn sub(&mut self, dst: WritableReg, lhs: Reg, rhs: RegImm, size: OperandSize) -> Result<()>;

    /// Perform multiplication operation.
    fn mul(&mut self, dst: WritableReg, lhs: Reg, rhs: RegImm, size: OperandSize) -> Result<()>;

    /// Perform a floating point add operation.
    fn float_add(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) -> Result<()>;

    /// Perform a floating point subtraction operation.
    fn float_sub(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) -> Result<()>;

    /// Perform a floating point multiply operation.
    fn float_mul(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) -> Result<()>;

    /// Perform a floating point divide operation.
    fn float_div(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) -> Result<()>;

    /// Perform a floating point minimum operation. In x86, this will emit
    /// multiple instructions.
    fn float_min(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) -> Result<()>;

    /// Perform a floating point maximum operation. In x86, this will emit
    /// multiple instructions.
    fn float_max(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) -> Result<()>;

    /// Perform a floating point copysign operation. In x86, this will emit
    /// multiple instructions.
    fn float_copysign(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Reg,
        size: OperandSize,
    ) -> Result<()>;

    /// Perform a floating point abs operation.
    fn float_abs(&mut self, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Perform a floating point negation operation.
    fn float_neg(&mut self, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Perform a floating point floor operation.
    fn float_round<
        F: FnMut(&mut FuncEnv<Self::Ptr>, &mut CodeGenContext<Emission>, &mut Self) -> Result<()>,
    >(
        &mut self,
        mode: RoundingMode,
        env: &mut FuncEnv<Self::Ptr>,
        context: &mut CodeGenContext<Emission>,
        size: OperandSize,
        fallback: F,
    ) -> Result<()>;

    /// Perform a floating point square root operation.
    fn float_sqrt(&mut self, dst: WritableReg, src: Reg, size: OperandSize) -> Result<()>;

    /// Perform logical and operation.
    fn and(&mut self, dst: WritableReg, lhs: Reg, rhs: RegImm, size: OperandSize) -> Result<()>;

    /// Perform logical or operation.
    fn or(&mut self, dst: WritableReg, lhs: Reg, rhs: RegImm, size: OperandSize) -> Result<()>;

    /// Perform logical exclusive or operation.
    fn xor(&mut self, dst: WritableReg, lhs: Reg, rhs: RegImm, size: OperandSize) -> Result<()>;

    /// Perform a shift operation between a register and an immediate.
    fn shift_ir(
        &mut self,
        dst: WritableReg,
        imm: Imm,
        lhs: Reg,
        kind: ShiftKind,
        size: OperandSize,
    ) -> Result<()>;

    /// Perform a shift operation between two registers.
    /// This case is special in that some architectures have specific expectations
    /// regarding the location of the instruction arguments. To free the
    /// caller from having to deal with the architecture specific constraints
    /// we give this function access to the code generation context, allowing
    /// each implementation to decide the lowering path.
    fn shift(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        kind: ShiftKind,
        size: OperandSize,
    ) -> Result<()>;

    /// Perform division operation.
    /// Division is special in that some architectures have specific
    /// expectations regarding the location of the instruction
    /// arguments and regarding the location of the quotient /
    /// remainder. To free the caller from having to deal with the
    /// architecture specific constraints we give this function access
    /// to the code generation context, allowing each implementation
    /// to decide the lowering path.  For cases in which division is a
    /// unconstrained binary operation, the caller can decide to use
    /// the `CodeGenContext::i32_binop` or `CodeGenContext::i64_binop`
    /// functions.
    fn div(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        kind: DivKind,
        size: OperandSize,
    ) -> Result<()>;

    /// Calculate remainder.
    fn rem(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        kind: RemKind,
        size: OperandSize,
    ) -> Result<()>;

    /// Compares `src1` against `src2` for the side effect of setting processor
    /// flags.
    ///
    /// Note that `src1` is the left-hand-side of the comparison and `src2` is
    /// the right-hand-side, so if testing `a < b` then `src1 == a` and
    /// `src2 == b`
    fn cmp(&mut self, src1: Reg, src2: RegImm, size: OperandSize) -> Result<()>;

    /// Compare src and dst and put the result in dst.
    /// This function will potentially emit a series of instructions.
    ///
    /// The initial value in `dst` is the left-hand-side of the comparison and
    /// the initial value in `src` is the right-hand-side of the comparison.
    /// That means for `a < b` then `dst == a` and `src == b`.
    fn cmp_with_set(
        &mut self,
        dst: WritableReg,
        src: RegImm,
        kind: IntCmpKind,
        size: OperandSize,
    ) -> Result<()>;

    /// Compare floats in src1 and src2 and put the result in dst.
    /// In x86, this will emit multiple instructions.
    fn float_cmp_with_set(
        &mut self,
        dst: WritableReg,
        src1: Reg,
        src2: Reg,
        kind: FloatCmpKind,
        size: OperandSize,
    ) -> Result<()>;

    /// Count the number of leading zeroes in src and put the result in dst.
    /// In x64, this will emit multiple instructions if the `has_lzcnt` flag is
    /// false.
    fn clz(&mut self, dst: WritableReg, src: Reg, size: OperandSize) -> Result<()>;

    /// Count the number of trailing zeroes in src and put the result in dst.masm
    /// In x64, this will emit multiple instructions if the `has_tzcnt` flag is
    /// false.
    fn ctz(&mut self, dst: WritableReg, src: Reg, size: OperandSize) -> Result<()>;

    /// Push the register to the stack, returning the stack slot metadata.
    // NB
    // The stack alignment should not be assumed after any call to `push`,
    // unless explicitly aligned otherwise.  Typically, stack alignment is
    // maintained at call sites and during the execution of
    // epilogues.
    fn push(&mut self, src: Reg, size: OperandSize) -> Result<StackSlot>;

    /// Finalize the assembly and return the result.
    fn finalize(self, base: Option<SourceLoc>) -> Result<MachBufferFinalized<Final>>;

    /// Zero a particular register.
    fn zero(&mut self, reg: WritableReg) -> Result<()>;

    /// Count the number of 1 bits in src and put the result in dst. In x64,
    /// this will emit multiple instructions if the `has_popcnt` flag is false.
    fn popcnt(&mut self, context: &mut CodeGenContext<Emission>, size: OperandSize) -> Result<()>;

    /// Converts an i64 to an i32 by discarding the high 32 bits.
    fn wrap(&mut self, dst: WritableReg, src: Reg) -> Result<()>;

    /// Extends an integer of a given size to a larger size.
    fn extend(&mut self, dst: WritableReg, src: Reg, kind: ExtendKind) -> Result<()>;

    /// Emits one or more instructions to perform a signed truncation of a
    /// float into an integer.
    fn signed_truncate(
        &mut self,
        dst: WritableReg,
        src: Reg,
        src_size: OperandSize,
        dst_size: OperandSize,
        kind: TruncKind,
    ) -> Result<()>;

    /// Emits one or more instructions to perform an unsigned truncation of a
    /// float into an integer.
    fn unsigned_truncate(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        src_size: OperandSize,
        dst_size: OperandSize,
        kind: TruncKind,
    ) -> Result<()>;

    /// Emits one or more instructions to perform a signed convert of an
    /// integer into a float.
    fn signed_convert(
        &mut self,
        dst: WritableReg,
        src: Reg,
        src_size: OperandSize,
        dst_size: OperandSize,
    ) -> Result<()>;

    /// Emits one or more instructions to perform an unsigned convert of an
    /// integer into a float.
    fn unsigned_convert(
        &mut self,
        dst: WritableReg,
        src: Reg,
        tmp_gpr: Reg,
        src_size: OperandSize,
        dst_size: OperandSize,
    ) -> Result<()>;

    /// Reinterpret a float as an integer.
    fn reinterpret_float_as_int(
        &mut self,
        dst: WritableReg,
        src: Reg,
        size: OperandSize,
    ) -> Result<()>;

    /// Reinterpret an integer as a float.
    fn reinterpret_int_as_float(
        &mut self,
        dst: WritableReg,
        src: Reg,
        size: OperandSize,
    ) -> Result<()>;

    /// Demote an f64 to an f32.
    fn demote(&mut self, dst: WritableReg, src: Reg) -> Result<()>;

    /// Promote an f32 to an f64.
    fn promote(&mut self, dst: WritableReg, src: Reg) -> Result<()>;

    /// Zero a given memory range.
    ///
    /// The default implementation divides the given memory range
    /// into word-sized slots. Then it unrolls a series of store
    /// instructions, effectively assigning zero to each slot.
    fn zero_mem_range(&mut self, mem: &Range<u32>) -> Result<()> {
        let word_size = <Self::ABI as abi::ABI>::word_bytes() as u32;
        if mem.is_empty() {
            return Ok(());
        }

        let start = if mem.start % word_size == 0 {
            mem.start
        } else {
            // Ensure that the start of the range is at least 4-byte aligned.
            assert!(mem.start % 4 == 0);
            let start = align_to(mem.start, word_size);
            let addr: Self::Address = self.local_address(&LocalSlot::i32(start))?;
            self.store(RegImm::i32(0), addr, OperandSize::S32)?;
            // Ensure that the new start of the range, is word-size aligned.
            assert!(start % word_size == 0);
            start
        };

        let end = align_to(mem.end, word_size);
        let slots = (end - start) / word_size;

        if slots == 1 {
            let slot = LocalSlot::i64(start + word_size);
            let addr: Self::Address = self.local_address(&slot)?;
            self.store(RegImm::i64(0), addr, OperandSize::S64)?;
        } else {
            // TODO
            // Add an upper bound to this generation;
            // given a considerably large amount of slots
            // this will be inefficient.
            self.with_scratch::<IntScratch, _>(|masm, scratch| {
                masm.zero(scratch.writable())?;
                let zero = RegImm::reg(scratch.inner());

                for step in (start..end).step_by(word_size as usize) {
                    let slot = LocalSlot::i64(step + word_size);
                    let addr: Self::Address = masm.local_address(&slot)?;
                    masm.store(zero, addr, OperandSize::S64)?;
                }
                anyhow::Ok(())
            })?;
        }

        Ok(())
    }

    /// Generate a label.
    fn get_label(&mut self) -> Result<MachLabel>;

    /// Bind the given label at the current code offset.
    fn bind(&mut self, label: MachLabel) -> Result<()>;

    /// Conditional branch.
    ///
    /// Performs a comparison between the two operands,
    /// and immediately after emits a jump to the given
    /// label destination if the condition is met.
    fn branch(
        &mut self,
        kind: IntCmpKind,
        lhs: Reg,
        rhs: RegImm,
        taken: MachLabel,
        size: OperandSize,
    ) -> Result<()>;

    /// Emits and unconditional jump to the given label.
    fn jmp(&mut self, target: MachLabel) -> Result<()>;

    /// Emits a jump table sequence. The default label is specified as
    /// the last element of the targets slice.
    fn jmp_table(&mut self, targets: &[MachLabel], index: Reg, tmp: Reg) -> Result<()>;

    /// Emit an unreachable code trap.
    fn unreachable(&mut self) -> Result<()>;

    /// Emit an unconditional trap.
    fn trap(&mut self, code: TrapCode) -> Result<()>;

    /// Traps if the condition code is met.
    fn trapif(&mut self, cc: IntCmpKind, code: TrapCode) -> Result<()>;

    /// Trap if the source register is zero.
    fn trapz(&mut self, src: Reg, code: TrapCode) -> Result<()>;

    /// Ensures that the stack pointer is correctly positioned before an unconditional
    /// jump according to the requirements of the destination target.
    fn ensure_sp_for_jump(&mut self, target: SPOffset) -> Result<()> {
        let bytes = self
            .sp_offset()?
            .as_u32()
            .checked_sub(target.as_u32())
            .unwrap_or(0);

        if bytes > 0 {
            self.free_stack(bytes)?;
        }

        Ok(())
    }

    /// Mark the start of a source location returning the machine code offset
    /// and the relative source code location.
    fn start_source_loc(&mut self, loc: RelSourceLoc) -> Result<(CodeOffset, RelSourceLoc)>;

    /// Mark the end of a source location.
    fn end_source_loc(&mut self) -> Result<()>;

    /// The current offset, in bytes from the beginning of the function.
    fn current_code_offset(&self) -> Result<CodeOffset>;

    /// Performs a 128-bit addition
    fn add128(
        &mut self,
        dst_lo: WritableReg,
        dst_hi: WritableReg,
        lhs_lo: Reg,
        lhs_hi: Reg,
        rhs_lo: Reg,
        rhs_hi: Reg,
    ) -> Result<()>;

    /// Performs a 128-bit subtraction
    fn sub128(
        &mut self,
        dst_lo: WritableReg,
        dst_hi: WritableReg,
        lhs_lo: Reg,
        lhs_hi: Reg,
        rhs_lo: Reg,
        rhs_hi: Reg,
    ) -> Result<()>;

    /// Performs a widening multiplication from two 64-bit operands into a
    /// 128-bit result.
    ///
    /// Note that some platforms require special handling of registers in this
    /// instruction (e.g. x64) so full access to `CodeGenContext` is provided.
    fn mul_wide(&mut self, context: &mut CodeGenContext<Emission>, kind: MulWideKind)
    -> Result<()>;

    /// Takes the value in a src operand and replicates it across lanes of
    /// `size` in a destination result.
    fn splat(&mut self, context: &mut CodeGenContext<Emission>, size: SplatKind) -> Result<()>;

    /// Performs a shuffle between two 128-bit vectors into a 128-bit result
    /// using lanes as a mask to select which indexes to copy.
    fn shuffle(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, lanes: [u8; 16]) -> Result<()>;

    /// Performs a swizzle between two 128-bit vectors into a 128-bit result.
    fn swizzle(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg) -> Result<()>;

    /// Performs the RMW `op` operation on the passed `addr`.
    ///
    /// The value *before* the operation was performed is written back to the `operand` register.
    fn atomic_rmw(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        addr: Self::Address,
        size: OperandSize,
        op: RmwOp,
        flags: MemFlags,
        extend: Option<Extend<Zero>>,
    ) -> Result<()>;

    /// Extracts the scalar value from `src` in `lane` to `dst`.
    fn extract_lane(
        &mut self,
        src: Reg,
        dst: WritableReg,
        lane: u8,
        kind: ExtractLaneKind,
    ) -> Result<()>;

    /// Replaces the value in `lane` in `dst` with the value in `src`.
    fn replace_lane(
        &mut self,
        src: RegImm,
        dst: WritableReg,
        lane: u8,
        kind: ReplaceLaneKind,
    ) -> Result<()>;

    /// Perform an atomic CAS (compare-and-swap) operation with the value at `addr`, and `expected`
    /// and `replacement` (at the top of the context's stack).
    ///
    /// This method takes the `CodeGenContext` as an arguments to accommodate architectures that
    /// expect parameters in specific registers. The context stack contains the `replacement`,
    /// and `expected` values in that order. The implementer is expected to push the value at
    /// `addr` before the update to the context's stack before returning.
    fn atomic_cas(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        addr: Self::Address,
        size: OperandSize,
        flags: MemFlags,
        extend: Option<Extend<Zero>>,
    ) -> Result<()>;

    /// Compares vector registers `lhs` and `rhs` for equality and puts the
    /// vector of results in `dst`.
    fn v128_eq(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Reg,
        kind: VectorEqualityKind,
    ) -> Result<()>;

    /// Compares vector registers `lhs` and `rhs` for inequality and puts the
    /// vector of results in `dst`.
    fn v128_ne(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Reg,
        kind: VectorEqualityKind,
    ) -> Result<()>;

    /// Performs a less than comparison with vector registers `lhs` and `rhs`
    /// and puts the vector of results in `dst`.
    fn v128_lt(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Reg,
        kind: VectorCompareKind,
    ) -> Result<()>;

    /// Performs a less than or equal comparison with vector registers `lhs`
    /// and `rhs` and puts the vector of results in `dst`.
    fn v128_le(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Reg,
        kind: VectorCompareKind,
    ) -> Result<()>;

    /// Performs a greater than comparison with vector registers `lhs` and
    /// `rhs` and puts the vector of results in `dst`.
    fn v128_gt(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Reg,
        kind: VectorCompareKind,
    ) -> Result<()>;

    /// Performs a greater than or equal comparison with vector registers `lhs`
    /// and `rhs` and puts the vector of results in `dst`.
    fn v128_ge(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Reg,
        kind: VectorCompareKind,
    ) -> Result<()>;

    /// Emit a memory fence.
    fn fence(&mut self) -> Result<()>;

    /// Perform a logical `not` operation on the 128bits vector value in `dst`.
    fn v128_not(&mut self, dst: WritableReg) -> Result<()>;

    /// Perform a logical `and` operation on `src1` and `src1`, both 128bits vector values, writing
    /// the result to `dst`.
    fn v128_and(&mut self, src1: Reg, src2: Reg, dst: WritableReg) -> Result<()>;

    /// Perform a logical `and_not` operation on `src1` and `src1`, both 128bits vector values, writing
    /// the result to `dst`.
    ///
    /// `and_not` is not commutative: dst = !src1 & src2.
    fn v128_and_not(&mut self, src1: Reg, src2: Reg, dst: WritableReg) -> Result<()>;

    /// Perform a logical `or` operation on `src1` and `src1`, both 128bits vector values, writing
    /// the result to `dst`.
    fn v128_or(&mut self, src1: Reg, src2: Reg, dst: WritableReg) -> Result<()>;

    /// Perform a logical `xor` operation on `src1` and `src1`, both 128bits vector values, writing
    /// the result to `dst`.
    fn v128_xor(&mut self, src1: Reg, src2: Reg, dst: WritableReg) -> Result<()>;

    /// Given two 128bits vectors `src1` and `src2`, and a 128bits bitmask `mask`, selects bits
    /// from `src1` when mask is 1, and from `src2` when mask is 0.
    ///
    /// This is equivalent to: `v128.or(v128.and(src1, mask), v128.and(src2, v128.not(mask)))`.
    fn v128_bitselect(&mut self, src1: Reg, src2: Reg, mask: Reg, dst: WritableReg) -> Result<()>;

    /// If any bit in `src` is 1, set `dst` to 1, or 0 otherwise.
    fn v128_any_true(&mut self, src: Reg, dst: WritableReg) -> Result<()>;

    /// Convert vector of integers to vector of floating points.
    fn v128_convert(&mut self, src: Reg, dst: WritableReg, kind: V128ConvertKind) -> Result<()>;

    /// Convert two input vectors into a smaller lane vector by narrowing each
    /// lane.
    fn v128_narrow(
        &mut self,
        src1: Reg,
        src2: Reg,
        dst: WritableReg,
        kind: V128NarrowKind,
    ) -> Result<()>;

    /// Converts a vector containing two 64-bit floating point lanes to two
    /// 32-bit floating point lanes and setting the two higher lanes to 0.
    fn v128_demote(&mut self, src: Reg, dst: WritableReg) -> Result<()>;

    /// Converts a vector containing four 32-bit floating point lanes to two
    /// 64-bit floating point lanes. Only the two lower lanes are converted.
    fn v128_promote(&mut self, src: Reg, dst: WritableReg) -> Result<()>;

    /// Converts low or high half of the smaller lane vector to a larger lane
    /// vector.
    fn v128_extend(&mut self, src: Reg, dst: WritableReg, kind: V128ExtendKind) -> Result<()>;

    /// Perform a vector add between `lsh` and `rhs`, placing the result in
    /// `dst`.
    fn v128_add(&mut self, lhs: Reg, rhs: Reg, dst: WritableReg, kind: V128AddKind) -> Result<()>;

    /// Perform a vector sub between `lhs` and `rhs`, placing the result in `dst`.
    fn v128_sub(&mut self, lhs: Reg, rhs: Reg, dst: WritableReg, kind: V128SubKind) -> Result<()>;

    /// Perform a vector lane-wise mul between `lhs` and `rhs`, placing the result in `dst`.
    fn v128_mul(&mut self, context: &mut CodeGenContext<Emission>, kind: V128MulKind)
    -> Result<()>;

    /// Perform an absolute operation on a vector.
    fn v128_abs(&mut self, src: Reg, dst: WritableReg, kind: V128AbsKind) -> Result<()>;

    /// Vectorized negate of the content of `op`.
    fn v128_neg(&mut self, op: WritableReg, kind: V128NegKind) -> Result<()>;

    /// Perform the shift operation specified by `kind`, by the shift amount specified by the 32-bit
    /// integer at the top of the stack, on the 128-bit vector specified by the second value
    /// from the top of the stack, interpreted as packed integers of size `lane_width`.
    ///
    /// The shift amount is taken modulo `lane_width`.
    fn v128_shift(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        lane_width: OperandSize,
        kind: ShiftKind,
    ) -> Result<()>;

    /// Perform a saturating integer q-format rounding multiplication.
    fn v128_q15mulr_sat_s(
        &mut self,
        lhs: Reg,
        rhs: Reg,
        dst: WritableReg,
        size: OperandSize,
    ) -> Result<()>;

    /// Sets `dst` to 1 if all lanes in `src` are non-zero, sets `dst` to 0
    /// otherwise.
    fn v128_all_true(&mut self, src: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Extracts the high bit of each lane in `src` and produces a scalar mask
    /// with all bits concatenated in `dst`.
    fn v128_bitmask(&mut self, src: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Lanewise truncation operation.
    ///
    /// If using an integer kind of truncation, then this performs a lane-wise
    /// saturating conversion from float to integer using the IEEE
    /// `convertToIntegerTowardZero` function. If any input lane is NaN, the
    /// resulting lane is 0. If the rounded integer value of a lane is outside
    /// the range of the destination type, the result is saturated to the
    /// nearest representable integer value.
    fn v128_trunc(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        kind: V128TruncKind,
    ) -> Result<()>;

    /// Perform a lane-wise `min` operation between `src1` and `src2`.
    fn v128_min(&mut self, src1: Reg, src2: Reg, dst: WritableReg, kind: V128MinKind)
    -> Result<()>;

    /// Perform a lane-wise `max` operation between `src1` and `src2`.
    fn v128_max(&mut self, src1: Reg, src2: Reg, dst: WritableReg, kind: V128MaxKind)
    -> Result<()>;

    /// Perform the lane-wise integer extended multiplication producing twice wider result than the
    /// inputs. This is equivalent to an extend followed by a multiply.
    ///
    /// The extension to be performed is inferred from the `lane_width` and the `kind` of extmul,
    /// e.g, if `lane_width` is `S16`, and `kind` is `LowSigned`, then we sign-extend the lower
    /// 8bits of the 16bits lanes.
    fn v128_extmul(
        &mut self,
        context: &mut CodeGenContext<Emission>,
        kind: V128ExtMulKind,
    ) -> Result<()>;

    /// Perform the lane-wise integer extended pairwise addition producing extended results (twice
    /// wider results than the inputs).
    fn v128_extadd_pairwise(
        &mut self,
        src: Reg,
        dst: WritableReg,
        kind: V128ExtAddKind,
    ) -> Result<()>;

    /// Lane-wise multiply signed 16-bit integers in `lhs` and `rhs` and add
    /// adjacent pairs of the 32-bit results.
    fn v128_dot(&mut self, lhs: Reg, rhs: Reg, dst: WritableReg) -> Result<()>;

    /// Count the number of bits set in each lane.
    fn v128_popcnt(&mut self, context: &mut CodeGenContext<Emission>) -> Result<()>;

    /// Lane-wise rounding average of vectors of integers in `lhs` and `rhs`
    /// and put the results in `dst`.
    fn v128_avgr(&mut self, lhs: Reg, rhs: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Lane-wise IEEE division on vectors of floats.
    fn v128_div(&mut self, lhs: Reg, rhs: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Lane-wise IEEE square root of vector of floats.
    fn v128_sqrt(&mut self, src: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Lane-wise ceiling of vector of floats.
    fn v128_ceil(&mut self, src: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Lane-wise flooring of vector of floats.
    fn v128_floor(&mut self, src: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Lane-wise rounding to nearest integer for vector of floats.
    fn v128_nearest(&mut self, src: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Lane-wise minimum value defined as `rhs < lhs ? rhs : lhs`.
    fn v128_pmin(&mut self, lhs: Reg, rhs: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;

    /// Lane-wise maximum value defined as `lhs < rhs ? rhs : lhs`.
    fn v128_pmax(&mut self, lhs: Reg, rhs: Reg, dst: WritableReg, size: OperandSize) -> Result<()>;
}
