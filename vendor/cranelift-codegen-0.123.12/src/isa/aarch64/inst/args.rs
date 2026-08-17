//! AArch64 ISA definitions: instruction arguments.

use crate::ir::types::*;
use crate::isa::aarch64::inst::*;

//=============================================================================
// Instruction sub-components: shift and extend descriptors

/// A shift operator for a register or immediate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ShiftOp {
    /// Logical shift left.
    LSL = 0b00,
    /// Logical shift right.
    LSR = 0b01,
    /// Arithmetic shift right.
    ASR = 0b10,
    /// Rotate right.
    ROR = 0b11,
}

impl ShiftOp {
    /// Get the encoding of this shift op.
    pub fn bits(self) -> u8 {
        self as u8
    }
}

/// A shift operator amount.
#[derive(Clone, Copy, Debug)]
pub struct ShiftOpShiftImm(u8);

impl ShiftOpShiftImm {
    /// Maximum shift for shifted-register operands.
    pub const MAX_SHIFT: u64 = 63;

    /// Create a new shiftop shift amount, if possible.
    pub fn maybe_from_shift(shift: u64) -> Option<ShiftOpShiftImm> {
        if shift <= Self::MAX_SHIFT {
            Some(ShiftOpShiftImm(shift as u8))
        } else {
            None
        }
    }

    /// Return the shift amount.
    pub fn value(self) -> u8 {
        self.0
    }

    /// Mask down to a given number of bits.
    pub fn mask(self, bits: u8) -> ShiftOpShiftImm {
        ShiftOpShiftImm(self.0 & (bits - 1))
    }
}

/// A shift operator with an amount, guaranteed to be within range.
#[derive(Copy, Clone, Debug)]
pub struct ShiftOpAndAmt {
    /// The shift operator.
    op: ShiftOp,
    /// The shift operator amount.
    shift: ShiftOpShiftImm,
}

impl ShiftOpAndAmt {
    /// Create a new shift operator with an amount.
    pub fn new(op: ShiftOp, shift: ShiftOpShiftImm) -> ShiftOpAndAmt {
        ShiftOpAndAmt { op, shift }
    }

    /// Get the shift op.
    pub fn op(&self) -> ShiftOp {
        self.op
    }

    /// Get the shift amount.
    pub fn amt(&self) -> ShiftOpShiftImm {
        self.shift
    }
}

/// An extend operator for a register.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ExtendOp {
    /// Unsigned extend byte.
    UXTB = 0b000,
    /// Unsigned extend halfword.
    UXTH = 0b001,
    /// Unsigned extend word.
    UXTW = 0b010,
    /// Unsigned extend doubleword.
    UXTX = 0b011,
    /// Signed extend byte.
    SXTB = 0b100,
    /// Signed extend halfword.
    SXTH = 0b101,
    /// Signed extend word.
    SXTW = 0b110,
    /// Signed extend doubleword.
    SXTX = 0b111,
}

impl ExtendOp {
    /// Encoding of this op.
    pub fn bits(self) -> u8 {
        self as u8
    }
}

//=============================================================================
// Instruction sub-components (memory addresses): definitions

/// A reference to some memory address.
#[derive(Clone, Debug)]
pub enum MemLabel {
    /// An address in the code, a constant pool or jumptable, with relative
    /// offset from this instruction. This form must be used at emission time;
    /// see `memlabel_finalize()` for how other forms are lowered to this one.
    PCRel(i32),
    /// An address that refers to a label within a `MachBuffer`, for example a
    /// constant that lives in the pool at the end of the function.
    Mach(MachLabel),
}

impl AMode {
    /// Memory reference using an address in a register.
    pub fn reg(reg: Reg) -> AMode {
        // Use UnsignedOffset rather than Unscaled to use ldr rather than ldur.
        // This also does not use PostIndexed / PreIndexed as they update the register.
        AMode::UnsignedOffset {
            rn: reg,
            uimm12: UImm12Scaled::zero(I64),
        }
    }

    /// Memory reference using `reg1 + sizeof(ty) * reg2` as an address, with `reg2` sign- or
    /// zero-extended as per `op`.
    pub fn reg_plus_reg_scaled_extended(reg1: Reg, reg2: Reg, op: ExtendOp) -> AMode {
        AMode::RegScaledExtended {
            rn: reg1,
            rm: reg2,
            extendop: op,
        }
    }
}

pub use crate::isa::aarch64::lower::isle::generated_code::PairAMode;

//=============================================================================
// Instruction sub-components (conditions, branches and branch targets):
// definitions

/// Condition for conditional branches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cond {
    /// Equal.
    Eq = 0,
    /// Not equal.
    Ne = 1,
    /// Unsigned greater than or equal to.
    Hs = 2,
    /// Unsigned less than.
    Lo = 3,
    /// Minus, negative.
    Mi = 4,
    /// Positive or zero.
    Pl = 5,
    /// Signed overflow.
    Vs = 6,
    /// No signed overflow.
    Vc = 7,
    /// Unsigned greater than.
    Hi = 8,
    /// Unsigned less than or equal to.
    Ls = 9,
    /// Signed greater or equal to.
    Ge = 10,
    /// Signed less than.
    Lt = 11,
    /// Signed greater than.
    Gt = 12,
    /// Signed less than or equal.
    Le = 13,
    /// Always executed.
    Al = 14,
    /// Always executed.
    Nv = 15,
}

impl Cond {
    /// Return the inverted condition.
    pub fn invert(self) -> Cond {
        match self {
            Cond::Eq => Cond::Ne,
            Cond::Ne => Cond::Eq,

            Cond::Hs => Cond::Lo,
            Cond::Lo => Cond::Hs,

            Cond::Mi => Cond::Pl,
            Cond::Pl => Cond::Mi,

            Cond::Vs => Cond::Vc,
            Cond::Vc => Cond::Vs,

            Cond::Hi => Cond::Ls,
            Cond::Ls => Cond::Hi,

            Cond::Ge => Cond::Lt,
            Cond::Lt => Cond::Ge,

            Cond::Gt => Cond::Le,
            Cond::Le => Cond::Gt,

            Cond::Al => Cond::Nv,
            Cond::Nv => Cond::Al,
        }
    }

    /// Return the machine encoding of this condition.
    pub fn bits(self) -> u32 {
        self as u32
    }
}

/// The kind of conditional branch: the common-case-optimized "reg-is-zero" /
/// "reg-is-nonzero" variants, or the generic one that tests the machine
/// condition codes.
#[derive(Clone, Copy, Debug)]
pub enum CondBrKind {
    /// Condition: given register is zero.
    Zero(Reg, OperandSize),
    /// Condition: given register is nonzero.
    NotZero(Reg, OperandSize),
    /// Condition: the given condition-code test is true.
    Cond(Cond),
}

impl CondBrKind {
    /// Return the inverted branch condition.
    pub fn invert(self) -> CondBrKind {
        match self {
            CondBrKind::Zero(reg, size) => CondBrKind::NotZero(reg, size),
            CondBrKind::NotZero(reg, size) => CondBrKind::Zero(reg, size),
            CondBrKind::Cond(c) => CondBrKind::Cond(c.invert()),
        }
    }
}

/// A branch target. Either unresolved (basic-block index) or resolved (offset
/// from end of current instruction).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchTarget {
    /// An unresolved reference to a Label, as passed into
    /// `lower_branch_group()`.
    Label(MachLabel),
    /// A fixed PC offset.
    ResolvedOffset(i32),
}

impl BranchTarget {
    /// Return the target's label, if it is a label-based target.
    pub fn as_label(self) -> Option<MachLabel> {
        match self {
            BranchTarget::Label(l) => Some(l),
            _ => None,
        }
    }

    /// Return the target's offset, if specified, or zero if label-based.
    pub fn as_offset14_or_zero(self) -> u32 {
        self.as_offset_bounded(14)
    }

    /// Return the target's offset, if specified, or zero if label-based.
    pub fn as_offset19_or_zero(self) -> u32 {
        self.as_offset_bounded(19)
    }

    /// Return the target's offset, if specified, or zero if label-based.
    pub fn as_offset26_or_zero(self) -> u32 {
        self.as_offset_bounded(26)
    }

    fn as_offset_bounded(self, bits: u32) -> u32 {
        let off = match self {
            BranchTarget::ResolvedOffset(off) => off >> 2,
            _ => 0,
        };
        let hi = (1 << (bits - 1)) - 1;
        let lo = -(1 << bits - 1);
        assert!(off <= hi);
        assert!(off >= lo);
        (off as u32) & ((1 << bits) - 1)
    }
}

impl PrettyPrint for ShiftOpAndAmt {
    fn pretty_print(&self, _: u8) -> String {
        format!("{:?} {}", self.op(), self.amt().value())
    }
}

impl PrettyPrint for ExtendOp {
    fn pretty_print(&self, _: u8) -> String {
        format!("{self:?}")
    }
}

impl PrettyPrint for MemLabel {
    fn pretty_print(&self, _: u8) -> String {
        match self {
            MemLabel::PCRel(off) => format!("pc+{off}"),
            MemLabel::Mach(off) => format!("label({})", off.as_u32()),
        }
    }
}

fn shift_for_type(size_bytes: u8) -> usize {
    match size_bytes {
        1 => 0,
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        _ => panic!("unknown type size: {size_bytes}"),
    }
}

impl PrettyPrint for AMode {
    fn pretty_print(&self, size_bytes: u8) -> String {
        debug_assert!(size_bytes != 0);
        match self {
            &AMode::Unscaled { rn, simm9 } => {
                let reg = pretty_print_reg(rn);
                if simm9.value != 0 {
                    let simm9 = simm9.pretty_print(8);
                    format!("[{reg}, {simm9}]")
                } else {
                    format!("[{reg}]")
                }
            }
            &AMode::UnsignedOffset { rn, uimm12 } => {
                let reg = pretty_print_reg(rn);
                if uimm12.value() != 0 {
                    let uimm12 = uimm12.pretty_print(8);
                    format!("[{reg}, {uimm12}]")
                } else {
                    format!("[{reg}]")
                }
            }
            &AMode::RegReg { rn, rm } => {
                let r1 = pretty_print_reg(rn);
                let r2 = pretty_print_reg(rm);
                format!("[{r1}, {r2}]")
            }
            &AMode::RegScaled { rn, rm } => {
                let r1 = pretty_print_reg(rn);
                let r2 = pretty_print_reg(rm);
                let shift = shift_for_type(size_bytes);
                format!("[{r1}, {r2}, LSL #{shift}]")
            }
            &AMode::RegScaledExtended { rn, rm, extendop } => {
                let shift = shift_for_type(size_bytes);
                let size = match extendop {
                    ExtendOp::SXTW | ExtendOp::UXTW => OperandSize::Size32,
                    _ => OperandSize::Size64,
                };
                let r1 = pretty_print_reg(rn);
                let r2 = pretty_print_ireg(rm, size);
                let op = extendop.pretty_print(0);
                format!("[{r1}, {r2}, {op} #{shift}]")
            }
            &AMode::RegExtended { rn, rm, extendop } => {
                let size = match extendop {
                    ExtendOp::SXTW | ExtendOp::UXTW => OperandSize::Size32,
                    _ => OperandSize::Size64,
                };
                let r1 = pretty_print_reg(rn);
                let r2 = pretty_print_ireg(rm, size);
                let op = extendop.pretty_print(0);
                format!("[{r1}, {r2}, {op}]")
            }
            &AMode::Label { ref label } => label.pretty_print(0),
            &AMode::SPPreIndexed { simm9 } => {
                let simm9 = simm9.pretty_print(8);
                format!("[sp, {simm9}]!")
            }
            &AMode::SPPostIndexed { simm9 } => {
                let simm9 = simm9.pretty_print(8);
                format!("[sp], {simm9}")
            }
            AMode::Const { addr } => format!("[const({})]", addr.as_u32()),

            // Eliminated by `mem_finalize()`.
            &AMode::SPOffset { .. }
            | &AMode::FPOffset { .. }
            | &AMode::IncomingArg { .. }
            | &AMode::SlotOffset { .. }
            | &AMode::RegOffset { .. } => {
                panic!("Unexpected pseudo mem-arg mode: {self:?}")
            }
        }
    }
}

impl PrettyPrint for PairAMode {
    fn pretty_print(&self, _: u8) -> String {
        match self {
            &PairAMode::SignedOffset { reg, simm7 } => {
                let reg = pretty_print_reg(reg);
                if simm7.value != 0 {
                    let simm7 = simm7.pretty_print(8);
                    format!("[{reg}, {simm7}]")
                } else {
                    format!("[{reg}]")
                }
            }
            &PairAMode::SPPreIndexed { simm7 } => {
                let simm7 = simm7.pretty_print(8);
                format!("[sp, {simm7}]!")
            }
            &PairAMode::SPPostIndexed { simm7 } => {
                let simm7 = simm7.pretty_print(8);
                format!("[sp], {simm7}")
            }
        }
    }
}

impl PrettyPrint for Cond {
    fn pretty_print(&self, _: u8) -> String {
        let mut s = format!("{self:?}");
        s.make_ascii_lowercase();
        s
    }
}

impl PrettyPrint for BranchTarget {
    fn pretty_print(&self, _: u8) -> String {
        match self {
            &BranchTarget::Label(label) => format!("label{:?}", label.as_u32()),
            &BranchTarget::ResolvedOffset(off) => format!("{off}"),
        }
    }
}

/// Type used to communicate the operand size of a machine instruction, as AArch64 has 32- and
/// 64-bit variants of many instructions (and integer registers).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperandSize {
    /// 32-bit.
    Size32,
    /// 64-bit.
    Size64,
}

impl OperandSize {
    /// 32-bit case?
    pub fn is32(self) -> bool {
        self == OperandSize::Size32
    }

    /// 64-bit case?
    pub fn is64(self) -> bool {
        self == OperandSize::Size64
    }

    /// Convert from a needed width to the smallest size that fits.
    pub fn from_bits<I: Into<usize>>(bits: I) -> OperandSize {
        let bits: usize = bits.into();
        assert!(bits <= 64);
        if bits <= 32 {
            OperandSize::Size32
        } else {
            OperandSize::Size64
        }
    }

    /// Return the operand size in bits.
    pub fn bits(&self) -> u8 {
        match self {
            OperandSize::Size32 => 32,
            OperandSize::Size64 => 64,
        }
    }

    /// Convert from an integer type into the smallest size that fits.
    pub fn from_ty(ty: Type) -> OperandSize {
        debug_assert!(!ty.is_vector());

        Self::from_bits(ty_bits(ty))
    }

    /// Convert to I32, I64, or I128.
    pub fn to_ty(self) -> Type {
        match self {
            OperandSize::Size32 => I32,
            OperandSize::Size64 => I64,
        }
    }

    /// Register interpretation bit.
    /// When 0, the register is interpreted as the 32-bit version.
    /// When 1, the register is interpreted as the 64-bit version.
    pub fn sf_bit(&self) -> u32 {
        match self {
            OperandSize::Size32 => 0,
            OperandSize::Size64 => 1,
        }
    }

    /// The maximum unsigned value representable in a value of this size.
    pub fn max_value(&self) -> u64 {
        match self {
            OperandSize::Size32 => u32::MAX as u64,
            OperandSize::Size64 => u64::MAX,
        }
    }
}

/// Type used to communicate the size of a scalar SIMD & FP operand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalarSize {
    /// 8-bit.
    Size8,
    /// 16-bit.
    Size16,
    /// 32-bit.
    Size32,
    /// 64-bit.
    Size64,
    /// 128-bit.
    Size128,
}

impl ScalarSize {
    /// Convert to an integer operand size.
    pub fn operand_size(&self) -> OperandSize {
        match self {
            ScalarSize::Size8 | ScalarSize::Size16 | ScalarSize::Size32 => OperandSize::Size32,
            ScalarSize::Size64 => OperandSize::Size64,
            _ => panic!("Unexpected operand_size request for: {self:?}"),
        }
    }

    /// Return the encoding bits that are used by some scalar FP instructions
    /// for a particular operand size.
    pub fn ftype(&self) -> u32 {
        match self {
            ScalarSize::Size16 => 0b11,
            ScalarSize::Size32 => 0b00,
            ScalarSize::Size64 => 0b01,
            _ => panic!("Unexpected scalar FP operand size: {self:?}"),
        }
    }

    /// Return the widened version of the scalar size.
    pub fn widen(&self) -> ScalarSize {
        match self {
            ScalarSize::Size8 => ScalarSize::Size16,
            ScalarSize::Size16 => ScalarSize::Size32,
            ScalarSize::Size32 => ScalarSize::Size64,
            ScalarSize::Size64 => ScalarSize::Size128,
            ScalarSize::Size128 => panic!("can't widen 128-bits"),
        }
    }

    /// Return the narrowed version of the scalar size.
    pub fn narrow(&self) -> ScalarSize {
        match self {
            ScalarSize::Size8 => panic!("can't narrow 8-bits"),
            ScalarSize::Size16 => ScalarSize::Size8,
            ScalarSize::Size32 => ScalarSize::Size16,
            ScalarSize::Size64 => ScalarSize::Size32,
            ScalarSize::Size128 => ScalarSize::Size64,
        }
    }

    /// Return a type with the same size as this scalar.
    pub fn ty(&self) -> Type {
        match self {
            ScalarSize::Size8 => I8,
            ScalarSize::Size16 => I16,
            ScalarSize::Size32 => I32,
            ScalarSize::Size64 => I64,
            ScalarSize::Size128 => I128,
        }
    }
}

/// Type used to communicate the size of a vector operand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorSize {
    /// 8-bit, 8 lanes.
    Size8x8,
    /// 8 bit, 16 lanes.
    Size8x16,
    /// 16-bit, 4 lanes.
    Size16x4,
    /// 16-bit, 8 lanes.
    Size16x8,
    /// 32-bit, 2 lanes.
    Size32x2,
    /// 32-bit, 4 lanes.
    Size32x4,
    /// 64-bit, 2 lanes.
    Size64x2,
}

impl VectorSize {
    /// Get the vector operand size with the given scalar size as lane size.
    pub fn from_lane_size(size: ScalarSize, is_128bit: bool) -> VectorSize {
        match (size, is_128bit) {
            (ScalarSize::Size8, false) => VectorSize::Size8x8,
            (ScalarSize::Size8, true) => VectorSize::Size8x16,
            (ScalarSize::Size16, false) => VectorSize::Size16x4,
            (ScalarSize::Size16, true) => VectorSize::Size16x8,
            (ScalarSize::Size32, false) => VectorSize::Size32x2,
            (ScalarSize::Size32, true) => VectorSize::Size32x4,
            (ScalarSize::Size64, true) => VectorSize::Size64x2,
            _ => panic!("Unexpected scalar FP operand size: {size:?}"),
        }
    }

    /// Get the integer operand size that corresponds to a lane of a vector with a certain size.
    pub fn operand_size(&self) -> OperandSize {
        match self {
            VectorSize::Size64x2 => OperandSize::Size64,
            _ => OperandSize::Size32,
        }
    }

    /// Get the scalar operand size that corresponds to a lane of a vector with a certain size.
    pub fn lane_size(&self) -> ScalarSize {
        match self {
            VectorSize::Size8x8 | VectorSize::Size8x16 => ScalarSize::Size8,
            VectorSize::Size16x4 | VectorSize::Size16x8 => ScalarSize::Size16,
            VectorSize::Size32x2 | VectorSize::Size32x4 => ScalarSize::Size32,
            VectorSize::Size64x2 => ScalarSize::Size64,
        }
    }

    /// Returns true if the VectorSize is 128-bits.
    pub fn is_128bits(&self) -> bool {
        match self {
            VectorSize::Size8x8 => false,
            VectorSize::Size8x16 => true,
            VectorSize::Size16x4 => false,
            VectorSize::Size16x8 => true,
            VectorSize::Size32x2 => false,
            VectorSize::Size32x4 => true,
            VectorSize::Size64x2 => true,
        }
    }

    /// Return the encoding bits that are used by some SIMD instructions
    /// for a particular operand size.
    pub fn enc_size(&self) -> (u32, u32) {
        let q = self.is_128bits() as u32;
        let size = match self.lane_size() {
            ScalarSize::Size8 => 0b00,
            ScalarSize::Size16 => 0b01,
            ScalarSize::Size32 => 0b10,
            ScalarSize::Size64 => 0b11,
            _ => unreachable!(),
        };

        (q, size)
    }

    /// Return the encoding bit that is used by some floating-point SIMD
    /// instructions for a particular operand size.
    pub fn enc_float_size(&self) -> u32 {
        match self.lane_size() {
            ScalarSize::Size32 => 0b0,
            ScalarSize::Size64 => 0b1,
            size => panic!("Unsupported floating-point size for vector op: {size:?}"),
        }
    }
}

impl APIKey {
    /// Returns the encoding of the `auti{key}` instruction used to decrypt the
    /// `lr` register.
    pub fn enc_auti_hint(&self) -> u32 {
        let (crm, op2) = match self {
            APIKey::AZ => (0b0011, 0b100),
            APIKey::ASP => (0b0011, 0b101),
            APIKey::BZ => (0b0011, 0b110),
            APIKey::BSP => (0b0011, 0b111),
        };
        0xd503201f | (crm << 8) | (op2 << 5)
    }
}

pub use crate::isa::aarch64::lower::isle::generated_code::TestBitAndBranchKind;

impl TestBitAndBranchKind {
    /// Complements this branch condition to act on the opposite result.
    pub fn complement(&self) -> TestBitAndBranchKind {
        match self {
            TestBitAndBranchKind::Z => TestBitAndBranchKind::NZ,
            TestBitAndBranchKind::NZ => TestBitAndBranchKind::Z,
        }
    }
}
