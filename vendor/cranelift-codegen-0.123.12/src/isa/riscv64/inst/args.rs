//! Riscv64 ISA definitions: instruction arguments.

use super::*;
use crate::ir::condcodes::CondCode;

use crate::isa::riscv64::lower::isle::generated_code::{
    COpcodeSpace, CaOp, CbOp, CiOp, CiwOp, ClOp, CrOp, CsOp, CssOp, CsznOp, FpuOPWidth, ZcbMemOp,
};
use crate::machinst::isle::WritableReg;

use std::fmt::Result;

/// A macro for defining a newtype of `Reg` that enforces some invariant about
/// the wrapped `Reg` (such as that it is of a particular register class).
macro_rules! newtype_of_reg {
    (
        $newtype_reg:ident,
        $newtype_writable_reg:ident,
        |$check_reg:ident| $check:expr
    ) => {
        /// A newtype wrapper around `Reg`.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $newtype_reg(Reg);

        impl PartialEq<Reg> for $newtype_reg {
            fn eq(&self, other: &Reg) -> bool {
                self.0 == *other
            }
        }

        impl From<$newtype_reg> for Reg {
            fn from(r: $newtype_reg) -> Self {
                r.0
            }
        }

        impl $newtype_reg {
            /// Create this newtype from the given register, or return `None` if the register
            /// is not a valid instance of this newtype.
            pub fn new($check_reg: Reg) -> Option<Self> {
                if $check { Some(Self($check_reg)) } else { None }
            }

            /// Get this newtype's underlying `Reg`.
            pub fn to_reg(self) -> Reg {
                self.0
            }
        }

        // Convenience impl so that people working with this newtype can use it
        // "just like" a plain `Reg`.
        //
        // NB: We cannot implement `DerefMut` because that would let people do
        // nasty stuff like `*my_xreg.deref_mut() = some_freg`, breaking the
        // invariants that `XReg` provides.
        impl std::ops::Deref for $newtype_reg {
            type Target = Reg;

            fn deref(&self) -> &Reg {
                &self.0
            }
        }

        /// Writable Reg.
        pub type $newtype_writable_reg = Writable<$newtype_reg>;
    };
}

// Newtypes for registers classes.
newtype_of_reg!(XReg, WritableXReg, |reg| reg.class() == RegClass::Int);
newtype_of_reg!(FReg, WritableFReg, |reg| reg.class() == RegClass::Float);
newtype_of_reg!(VReg, WritableVReg, |reg| reg.class() == RegClass::Vector);

/// An addressing mode specified for a load/store operation.
#[derive(Clone, Debug, Copy)]
pub enum AMode {
    /// Arbitrary offset from a register. Converted to generation of large
    /// offsets with multiple instructions as necessary during code emission.
    RegOffset(Reg, i64),
    /// Offset from the stack pointer.
    SPOffset(i64),

    /// Offset from the frame pointer.
    FPOffset(i64),

    /// Offset into the slot area of the stack, which lies just above the
    /// outgoing argument area that's setup by the function prologue.
    /// At emission time, this is converted to `SPOffset` with a fixup added to
    /// the offset constant. The fixup is a running value that is tracked as
    /// emission iterates through instructions in linear order, and can be
    /// adjusted up and down with [Inst::VirtualSPOffsetAdj].
    ///
    /// The standard ABI is in charge of handling this (by emitting the
    /// adjustment meta-instructions). See the diagram in the documentation
    /// for [crate::isa::aarch64::abi](the ABI module) for more details.
    SlotOffset(i64),

    /// Offset into the argument area.
    IncomingArg(i64),

    /// A reference to a constant which is placed outside of the function's
    /// body, typically at the end.
    Const(VCodeConstant),

    /// A reference to a label.
    Label(MachLabel),
}

impl AMode {
    /// Add the registers referenced by this AMode to `collector`.
    pub(crate) fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            AMode::RegOffset(reg, ..) => collector.reg_use(reg),
            // Registers used in these modes aren't allocatable.
            AMode::SPOffset(..)
            | AMode::FPOffset(..)
            | AMode::SlotOffset(..)
            | AMode::IncomingArg(..)
            | AMode::Const(..)
            | AMode::Label(..) => {}
        }
    }

    pub(crate) fn get_base_register(&self) -> Option<Reg> {
        match self {
            &AMode::RegOffset(reg, ..) => Some(reg),
            &AMode::SPOffset(..) => Some(stack_reg()),
            &AMode::FPOffset(..) => Some(fp_reg()),
            &AMode::SlotOffset(..) => Some(stack_reg()),
            &AMode::IncomingArg(..) => Some(stack_reg()),
            &AMode::Const(..) | AMode::Label(..) => None,
        }
    }

    pub(crate) fn get_offset_with_state(&self, state: &EmitState) -> i64 {
        match self {
            &AMode::SlotOffset(offset) => {
                offset + i64::from(state.frame_layout().outgoing_args_size)
            }

            // Compute the offset into the incoming argument area relative to SP
            &AMode::IncomingArg(offset) => {
                let frame_layout = state.frame_layout();
                let sp_offset = frame_layout.tail_args_size
                    + frame_layout.setup_area_size
                    + frame_layout.clobber_size
                    + frame_layout.fixed_frame_storage_size
                    + frame_layout.outgoing_args_size;
                i64::from(sp_offset) - offset
            }

            &AMode::RegOffset(_, offset) => offset,
            &AMode::SPOffset(offset) => offset,
            &AMode::FPOffset(offset) => offset,
            &AMode::Const(_) | &AMode::Label(_) => 0,
        }
    }

    /// Retrieve a MachLabel that corresponds to this addressing mode, if it exists.
    pub(crate) fn get_label_with_sink(&self, sink: &mut MachBuffer<Inst>) -> Option<MachLabel> {
        match self {
            &AMode::Const(addr) => Some(sink.get_label_for_constant(addr)),
            &AMode::Label(label) => Some(label),
            &AMode::RegOffset(..)
            | &AMode::SPOffset(..)
            | &AMode::FPOffset(..)
            | &AMode::IncomingArg(..)
            | &AMode::SlotOffset(..) => None,
        }
    }
}

impl Display for AMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            &AMode::RegOffset(r, offset, ..) => {
                write!(f, "{}({})", offset, reg_name(r))
            }
            &AMode::SPOffset(offset, ..) => {
                write!(f, "{offset}(sp)")
            }
            &AMode::SlotOffset(offset, ..) => {
                write!(f, "{offset}(slot)")
            }
            &AMode::IncomingArg(offset) => {
                write!(f, "-{offset}(incoming_arg)")
            }
            &AMode::FPOffset(offset, ..) => {
                write!(f, "{offset}(fp)")
            }
            &AMode::Const(addr, ..) => {
                write!(f, "[const({})]", addr.as_u32())
            }
            &AMode::Label(label) => {
                write!(f, "[label{}]", label.as_u32())
            }
        }
    }
}

impl From<StackAMode> for AMode {
    fn from(stack: StackAMode) -> AMode {
        match stack {
            StackAMode::IncomingArg(offset, stack_args_size) => {
                AMode::IncomingArg(i64::from(stack_args_size) - offset)
            }
            StackAMode::OutgoingArg(offset) => AMode::SPOffset(offset),
            StackAMode::Slot(offset) => AMode::SlotOffset(offset),
        }
    }
}

/// risc-v always take two register to compare
#[derive(Clone, Copy, Debug)]
pub struct IntegerCompare {
    pub(crate) kind: IntCC,
    pub(crate) rs1: Reg,
    pub(crate) rs2: Reg,
}

pub(crate) enum BranchFunct3 {
    // ==
    Eq,
    // !=
    Ne,
    // signed <
    Lt,
    // signed >=
    Ge,
    // unsigned <
    Ltu,
    // unsigned >=
    Geu,
}

impl BranchFunct3 {
    pub(crate) fn funct3(self) -> u32 {
        match self {
            BranchFunct3::Eq => 0b000,
            BranchFunct3::Ne => 0b001,
            BranchFunct3::Lt => 0b100,
            BranchFunct3::Ge => 0b101,
            BranchFunct3::Ltu => 0b110,
            BranchFunct3::Geu => 0b111,
        }
    }
}

impl IntegerCompare {
    pub(crate) fn op_code(self) -> u32 {
        0b1100011
    }

    // funct3 and if need inverse the register
    pub(crate) fn funct3(&self) -> (BranchFunct3, bool) {
        match self.kind {
            IntCC::Equal => (BranchFunct3::Eq, false),
            IntCC::NotEqual => (BranchFunct3::Ne, false),
            IntCC::SignedLessThan => (BranchFunct3::Lt, false),
            IntCC::SignedGreaterThanOrEqual => (BranchFunct3::Ge, false),

            IntCC::SignedGreaterThan => (BranchFunct3::Lt, true),
            IntCC::SignedLessThanOrEqual => (BranchFunct3::Ge, true),

            IntCC::UnsignedLessThan => (BranchFunct3::Ltu, false),
            IntCC::UnsignedGreaterThanOrEqual => (BranchFunct3::Geu, false),

            IntCC::UnsignedGreaterThan => (BranchFunct3::Ltu, true),
            IntCC::UnsignedLessThanOrEqual => (BranchFunct3::Geu, true),
        }
    }

    #[inline]
    pub(crate) fn op_name(&self) -> &'static str {
        match self.kind {
            IntCC::Equal => "beq",
            IntCC::NotEqual => "bne",
            IntCC::SignedLessThan => "blt",
            IntCC::SignedGreaterThanOrEqual => "bge",
            IntCC::SignedGreaterThan => "bgt",
            IntCC::SignedLessThanOrEqual => "ble",
            IntCC::UnsignedLessThan => "bltu",
            IntCC::UnsignedGreaterThanOrEqual => "bgeu",
            IntCC::UnsignedGreaterThan => "bgtu",
            IntCC::UnsignedLessThanOrEqual => "bleu",
        }
    }

    pub(crate) fn emit(self) -> u32 {
        let (funct3, reverse) = self.funct3();
        let (rs1, rs2) = if reverse {
            (self.rs2, self.rs1)
        } else {
            (self.rs1, self.rs2)
        };

        self.op_code()
            | funct3.funct3() << 12
            | reg_to_gpr_num(rs1) << 15
            | reg_to_gpr_num(rs2) << 20
    }

    pub(crate) fn inverse(self) -> Self {
        Self {
            kind: self.kind.complement(),
            ..self
        }
    }

    pub(crate) fn regs(&self) -> [Reg; 2] {
        [self.rs1, self.rs2]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FliConstant(u8);

impl FliConstant {
    pub(crate) fn new(value: u8) -> Self {
        debug_assert!(value <= 31, "Invalid FliConstant: {value}");
        Self(value)
    }

    pub(crate) fn maybe_from_u64(ty: Type, imm: u64) -> Option<Self> {
        // Convert the value into an F64, this allows us to represent
        // values from both f32 and f64 in the same value.
        let value = match ty {
            F16 => {
                // FIXME(#8312): Use `f16` once it has been stabilised.
                // Handle special/non-normal values first.
                match imm {
                    // `f16::MIN_POSITIVE`
                    0x0400 => return Some(Self::new(1)),
                    // 2 pow -16
                    0x0100 => return Some(Self::new(2)),
                    // 2 pow -15
                    0x0200 => return Some(Self::new(3)),
                    // `f16::INFINITY`
                    0x7c00 => return Some(Self::new(30)),
                    // Canonical NaN
                    0x7e00 => return Some(Self::new(31)),
                    _ => {
                        let exponent_bits = imm & 0x7c00;
                        if exponent_bits == 0 || exponent_bits == 0x7c00 {
                            // All non-normal values are handled above.
                            return None;
                        }
                        let sign = (imm & 0x8000) << 48;
                        // Adjust the exponent for the difference between the `f16` exponent bias
                        // and the `f64` exponent bias.
                        let exponent = (exponent_bits + ((1023 - 15) << 10)) << 42;
                        let significand = (imm & 0x3ff) << 42;
                        f64::from_bits(sign | exponent | significand)
                    }
                }
            }
            F32 => f32::from_bits(imm as u32) as f64,
            F64 => f64::from_bits(imm),
            _ => unimplemented!(),
        };

        Some(match (ty, value) {
            (_, f) if f == -1.0 => Self::new(0),

            // Since f64 can represent all f32 values, f32::min_positive won't be
            // the same as f64::min_positive, so we need to check for both indepenendtly
            (F32, f) if f == (f32::MIN_POSITIVE as f64) => Self::new(1),
            (F64, f) if f == f64::MIN_POSITIVE => Self::new(1),

            (_, f) if f == 2.0f64.powi(-16) => Self::new(2),
            (_, f) if f == 2.0f64.powi(-15) => Self::new(3),
            (_, f) if f == 2.0f64.powi(-8) => Self::new(4),
            (_, f) if f == 2.0f64.powi(-7) => Self::new(5),
            (_, f) if f == 0.0625 => Self::new(6),
            (_, f) if f == 0.125 => Self::new(7),
            (_, f) if f == 0.25 => Self::new(8),
            (_, f) if f == 0.3125 => Self::new(9),
            (_, f) if f == 0.375 => Self::new(10),
            (_, f) if f == 0.4375 => Self::new(11),
            (_, f) if f == 0.5 => Self::new(12),
            (_, f) if f == 0.625 => Self::new(13),
            (_, f) if f == 0.75 => Self::new(14),
            (_, f) if f == 0.875 => Self::new(15),
            (_, f) if f == 1.0 => Self::new(16),
            (_, f) if f == 1.25 => Self::new(17),
            (_, f) if f == 1.5 => Self::new(18),
            (_, f) if f == 1.75 => Self::new(19),
            (_, f) if f == 2.0 => Self::new(20),
            (_, f) if f == 2.5 => Self::new(21),
            (_, f) if f == 3.0 => Self::new(22),
            (_, f) if f == 4.0 => Self::new(23),
            (_, f) if f == 8.0 => Self::new(24),
            (_, f) if f == 16.0 => Self::new(25),
            (_, f) if f == 128.0 => Self::new(26),
            (_, f) if f == 256.0 => Self::new(27),
            (_, f) if f == 32768.0 => Self::new(28),
            (_, f) if f == 65536.0 => Self::new(29),
            (_, f) if f == f64::INFINITY => Self::new(30),

            // NaN's are not guaranteed to preserve the sign / payload bits, so we need to check
            // the original bits directly.
            (F32, f) if f.is_nan() && imm == 0x7fc0_0000 => Self::new(31), // Canonical NaN
            (F64, f) if f.is_nan() && imm == 0x7ff8_0000_0000_0000 => Self::new(31), // Canonical NaN
            _ => return None,
        })
    }

    pub(crate) fn format(self) -> &'static str {
        // The preferred assembly syntax for entries 1, 30, and 31 is min, inf, and nan, respectively.
        // For entries 0 through 29 (including entry 1), the assembler will accept decimal constants
        // in C-like syntax.
        match self.0 {
            0 => "-1.0",
            1 => "min",
            2 => "2^-16",
            3 => "2^-15",
            4 => "2^-8",
            5 => "2^-7",
            6 => "0.0625",
            7 => "0.125",
            8 => "0.25",
            9 => "0.3125",
            10 => "0.375",
            11 => "0.4375",
            12 => "0.5",
            13 => "0.625",
            14 => "0.75",
            15 => "0.875",
            16 => "1.0",
            17 => "1.25",
            18 => "1.5",
            19 => "1.75",
            20 => "2.0",
            21 => "2.5",
            22 => "3.0",
            23 => "4.0",
            24 => "8.0",
            25 => "16.0",
            26 => "128.0",
            27 => "256.0",
            28 => "32768.0",
            29 => "65536.0",
            30 => "inf",
            31 => "nan",
            _ => panic!("Invalid FliConstant"),
        }
    }

    pub(crate) fn bits(self) -> u8 {
        self.0
    }
}

impl FpuOPRRRR {
    pub(crate) fn op_name(self, width: FpuOPWidth) -> String {
        match self {
            Self::Fmadd => format!("fmadd.{width}"),
            Self::Fmsub => format!("fmsub.{width}"),
            Self::Fnmsub => format!("fnmsub.{width}"),
            Self::Fnmadd => format!("fnmadd.{width}"),
        }
    }

    pub(crate) fn opcode(self) -> u32 {
        match self {
            Self::Fmadd => 0b1000011,
            Self::Fmsub => 0b1000111,
            Self::Fnmsub => 0b1001011,
            Self::Fnmadd => 0b1001111,
        }
    }
}

impl FpuOPRR {
    pub(crate) fn op_name(self, width: FpuOPWidth) -> String {
        let fmv_width = match width {
            FpuOPWidth::H => "h",
            FpuOPWidth::S => "w",
            FpuOPWidth::D => "d",
            FpuOPWidth::Q => "q",
        };
        match self {
            Self::Fsqrt => format!("fsqrt.{width}"),
            Self::Fround => format!("fround.{width}"),
            Self::Fclass => format!("fclass.{width}"),
            Self::FcvtWFmt => format!("fcvt.w.{width}"),
            Self::FcvtWuFmt => format!("fcvt.wu.{width}"),
            Self::FcvtLFmt => format!("fcvt.l.{width}"),
            Self::FcvtLuFmt => format!("fcvt.lu.{width}"),
            Self::FcvtFmtW => format!("fcvt.{width}.w"),
            Self::FcvtFmtWu => format!("fcvt.{width}.wu"),
            Self::FcvtFmtL => format!("fcvt.{width}.l"),
            Self::FcvtFmtLu => format!("fcvt.{width}.lu"),

            // fmv instructions deviate from the normal encoding and instead
            // encode the width as "w" instead of "s". The ISA manual gives this rationale:
            //
            // Instructions FMV.S.X and FMV.X.S were renamed to FMV.W.X and FMV.X.W respectively
            // to be more consistent with their semantics, which did not change. The old names will continue
            // to be supported in the tools.
            Self::FmvXFmt => format!("fmv.x.{fmv_width}"),
            Self::FmvFmtX => format!("fmv.{fmv_width}.x"),

            Self::FcvtSD => "fcvt.s.d".to_string(),
            Self::FcvtDS => "fcvt.d.s".to_string(),
        }
    }

    pub(crate) fn is_convert_to_int(self) -> bool {
        match self {
            Self::FcvtWFmt | Self::FcvtWuFmt | Self::FcvtLFmt | Self::FcvtLuFmt => true,
            _ => false,
        }
    }

    pub(crate) fn has_frm(self) -> bool {
        match self {
            FpuOPRR::FmvXFmt | FpuOPRR::FmvFmtX | FpuOPRR::Fclass => false,
            _ => true,
        }
    }

    pub(crate) fn opcode(self) -> u32 {
        // OP-FP Major opcode
        0b1010011
    }

    pub(crate) fn rs2(self) -> u32 {
        match self {
            Self::Fsqrt => 0b00000,
            Self::Fround => 0b00100,
            Self::Fclass => 0b00000,
            Self::FcvtWFmt => 0b00000,
            Self::FcvtWuFmt => 0b00001,
            Self::FcvtLFmt => 0b00010,
            Self::FcvtLuFmt => 0b00011,
            Self::FcvtFmtW => 0b00000,
            Self::FcvtFmtWu => 0b00001,
            Self::FcvtFmtL => 0b00010,
            Self::FcvtFmtLu => 0b00011,
            Self::FmvXFmt => 0b00000,
            Self::FmvFmtX => 0b00000,
            Self::FcvtSD => 0b00001,
            Self::FcvtDS => 0b00000,
        }
    }

    pub(crate) fn funct5(self) -> u32 {
        match self {
            Self::Fsqrt => 0b01011,
            Self::Fround => 0b01000,
            Self::Fclass => 0b11100,
            Self::FcvtWFmt => 0b11000,
            Self::FcvtWuFmt => 0b11000,
            Self::FcvtLFmt => 0b11000,
            Self::FcvtLuFmt => 0b11000,
            Self::FcvtFmtW => 0b11010,
            Self::FcvtFmtWu => 0b11010,
            Self::FcvtFmtL => 0b11010,
            Self::FcvtFmtLu => 0b11010,
            Self::FmvXFmt => 0b11100,
            Self::FmvFmtX => 0b11110,
            Self::FcvtSD => 0b01000,
            Self::FcvtDS => 0b01000,
        }
    }

    pub(crate) fn funct7(self, width: FpuOPWidth) -> u32 {
        (self.funct5() << 2) | width.as_u32()
    }
}

impl FpuOPRRR {
    pub(crate) fn op_name(self, width: FpuOPWidth) -> String {
        match self {
            Self::Fadd => format!("fadd.{width}"),
            Self::Fsub => format!("fsub.{width}"),
            Self::Fmul => format!("fmul.{width}"),
            Self::Fdiv => format!("fdiv.{width}"),
            Self::Fsgnj => format!("fsgnj.{width}"),
            Self::Fsgnjn => format!("fsgnjn.{width}"),
            Self::Fsgnjx => format!("fsgnjx.{width}"),
            Self::Fmin => format!("fmin.{width}"),
            Self::Fmax => format!("fmax.{width}"),
            Self::Feq => format!("feq.{width}"),
            Self::Flt => format!("flt.{width}"),
            Self::Fle => format!("fle.{width}"),
            Self::Fminm => format!("fminm.{width}"),
            Self::Fmaxm => format!("fmaxm.{width}"),
        }
    }

    pub(crate) fn opcode(self) -> u32 {
        // OP-FP Major opcode
        0b1010011
    }

    pub(crate) const fn funct5(self) -> u32 {
        match self {
            Self::Fadd => 0b00000,
            Self::Fsub => 0b00001,
            Self::Fmul => 0b00010,
            Self::Fdiv => 0b00011,
            Self::Fsgnj => 0b00100,
            Self::Fsgnjn => 0b00100,
            Self::Fsgnjx => 0b00100,
            Self::Fmin => 0b00101,
            Self::Fmax => 0b00101,
            Self::Feq => 0b10100,
            Self::Flt => 0b10100,
            Self::Fle => 0b10100,
            Self::Fminm => 0b00101,
            Self::Fmaxm => 0b00101,
        }
    }

    pub(crate) fn funct7(self, width: FpuOPWidth) -> u32 {
        (self.funct5() << 2) | width.as_u32()
    }

    pub(crate) fn has_frm(self) -> bool {
        match self {
            FpuOPRRR::Fsgnj
            | FpuOPRRR::Fsgnjn
            | FpuOPRRR::Fsgnjx
            | FpuOPRRR::Fmin
            | FpuOPRRR::Fmax
            | FpuOPRRR::Feq
            | FpuOPRRR::Flt
            | FpuOPRRR::Fle => false,
            _ => true,
        }
    }
}

impl Display for FpuOPWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                FpuOPWidth::H => "h",
                FpuOPWidth::S => "s",
                FpuOPWidth::D => "d",
                FpuOPWidth::Q => "q",
            }
        )
    }
}

impl TryFrom<Type> for FpuOPWidth {
    type Error = &'static str;

    fn try_from(value: Type) -> std::result::Result<Self, Self::Error> {
        match value {
            F16 => Ok(FpuOPWidth::H),
            F32 => Ok(FpuOPWidth::S),
            F64 => Ok(FpuOPWidth::D),
            F128 => Ok(FpuOPWidth::Q),
            _ => Err("Invalid type for FpuOPWidth"),
        }
    }
}

impl FpuOPWidth {
    pub(crate) fn as_u32(&self) -> u32 {
        match self {
            FpuOPWidth::S => 0b00,
            FpuOPWidth::D => 0b01,
            FpuOPWidth::H => 0b10,
            FpuOPWidth::Q => 0b11,
        }
    }
}

impl AluOPRRR {
    pub(crate) const fn op_name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Sll => "sll",
            Self::Slt => "slt",
            Self::Sgt => "sgt",
            Self::SltU => "sltu",
            Self::Sgtu => "sgtu",
            Self::Xor => "xor",
            Self::Srl => "srl",
            Self::Sra => "sra",
            Self::Or => "or",
            Self::And => "and",
            Self::Addw => "addw",
            Self::Subw => "subw",
            Self::Sllw => "sllw",
            Self::Srlw => "srlw",
            Self::Sraw => "sraw",
            Self::Mul => "mul",
            Self::Mulh => "mulh",
            Self::Mulhsu => "mulhsu",
            Self::Mulhu => "mulhu",
            Self::Div => "div",
            Self::DivU => "divu",
            Self::Rem => "rem",
            Self::RemU => "remu",
            Self::Mulw => "mulw",
            Self::Divw => "divw",
            Self::Divuw => "divuw",
            Self::Remw => "remw",
            Self::Remuw => "remuw",
            Self::Adduw => "add.uw",
            Self::Andn => "andn",
            Self::Bclr => "bclr",
            Self::Bext => "bext",
            Self::Binv => "binv",
            Self::Bset => "bset",
            Self::Clmul => "clmul",
            Self::Clmulh => "clmulh",
            Self::Clmulr => "clmulr",
            Self::Max => "max",
            Self::Maxu => "maxu",
            Self::Min => "min",
            Self::Minu => "minu",
            Self::Orn => "orn",
            Self::Rol => "rol",
            Self::Rolw => "rolw",
            Self::Ror => "ror",
            Self::Rorw => "rorw",
            Self::Sh1add => "sh1add",
            Self::Sh1adduw => "sh1add.uw",
            Self::Sh2add => "sh2add",
            Self::Sh2adduw => "sh2add.uw",
            Self::Sh3add => "sh3add",
            Self::Sh3adduw => "sh3add.uw",
            Self::Xnor => "xnor",
            Self::Pack => "pack",
            Self::Packw => "packw",
            Self::Packh => "packh",
            Self::CzeroEqz => "czero.eqz",
            Self::CzeroNez => "czero.nez",
        }
    }

    pub fn funct3(self) -> u32 {
        match self {
            AluOPRRR::Add => 0b000,
            AluOPRRR::Sll => 0b001,
            AluOPRRR::Slt => 0b010,
            AluOPRRR::Sgt => 0b010,
            AluOPRRR::SltU => 0b011,
            AluOPRRR::Sgtu => 0b011,
            AluOPRRR::Xor => 0b100,
            AluOPRRR::Srl => 0b101,
            AluOPRRR::Sra => 0b101,
            AluOPRRR::Or => 0b110,
            AluOPRRR::And => 0b111,
            AluOPRRR::Sub => 0b000,

            AluOPRRR::Addw => 0b000,
            AluOPRRR::Subw => 0b000,
            AluOPRRR::Sllw => 0b001,
            AluOPRRR::Srlw => 0b101,
            AluOPRRR::Sraw => 0b101,

            AluOPRRR::Mul => 0b000,
            AluOPRRR::Mulh => 0b001,
            AluOPRRR::Mulhsu => 0b010,
            AluOPRRR::Mulhu => 0b011,
            AluOPRRR::Div => 0b100,
            AluOPRRR::DivU => 0b101,
            AluOPRRR::Rem => 0b110,
            AluOPRRR::RemU => 0b111,

            AluOPRRR::Mulw => 0b000,
            AluOPRRR::Divw => 0b100,
            AluOPRRR::Divuw => 0b101,
            AluOPRRR::Remw => 0b110,
            AluOPRRR::Remuw => 0b111,

            // Zbb
            AluOPRRR::Adduw => 0b000,
            AluOPRRR::Andn => 0b111,
            AluOPRRR::Bclr => 0b001,
            AluOPRRR::Bext => 0b101,
            AluOPRRR::Binv => 0b001,
            AluOPRRR::Bset => 0b001,
            AluOPRRR::Clmul => 0b001,
            AluOPRRR::Clmulh => 0b011,
            AluOPRRR::Clmulr => 0b010,
            AluOPRRR::Max => 0b110,
            AluOPRRR::Maxu => 0b111,
            AluOPRRR::Min => 0b100,
            AluOPRRR::Minu => 0b101,
            AluOPRRR::Orn => 0b110,
            AluOPRRR::Rol => 0b001,
            AluOPRRR::Rolw => 0b001,
            AluOPRRR::Ror => 0b101,
            AluOPRRR::Rorw => 0b101,
            AluOPRRR::Sh1add => 0b010,
            AluOPRRR::Sh1adduw => 0b010,
            AluOPRRR::Sh2add => 0b100,
            AluOPRRR::Sh2adduw => 0b100,
            AluOPRRR::Sh3add => 0b110,
            AluOPRRR::Sh3adduw => 0b110,
            AluOPRRR::Xnor => 0b100,

            // Zbkb
            AluOPRRR::Pack => 0b100,
            AluOPRRR::Packw => 0b100,
            AluOPRRR::Packh => 0b111,

            // ZiCond
            AluOPRRR::CzeroEqz => 0b101,
            AluOPRRR::CzeroNez => 0b111,
        }
    }

    pub fn op_code(self) -> u32 {
        match self {
            AluOPRRR::Add
            | AluOPRRR::Sub
            | AluOPRRR::Sll
            | AluOPRRR::Slt
            | AluOPRRR::Sgt
            | AluOPRRR::SltU
            | AluOPRRR::Sgtu
            | AluOPRRR::Xor
            | AluOPRRR::Srl
            | AluOPRRR::Sra
            | AluOPRRR::Or
            | AluOPRRR::And
            | AluOPRRR::Pack
            | AluOPRRR::Packh => 0b0110011,

            AluOPRRR::Addw
            | AluOPRRR::Subw
            | AluOPRRR::Sllw
            | AluOPRRR::Srlw
            | AluOPRRR::Sraw
            | AluOPRRR::Packw => 0b0111011,

            AluOPRRR::Mul
            | AluOPRRR::Mulh
            | AluOPRRR::Mulhsu
            | AluOPRRR::Mulhu
            | AluOPRRR::Div
            | AluOPRRR::DivU
            | AluOPRRR::Rem
            | AluOPRRR::RemU => 0b0110011,

            AluOPRRR::Mulw
            | AluOPRRR::Divw
            | AluOPRRR::Divuw
            | AluOPRRR::Remw
            | AluOPRRR::Remuw => 0b0111011,

            AluOPRRR::Adduw => 0b0111011,
            AluOPRRR::Andn
            | AluOPRRR::Bclr
            | AluOPRRR::Bext
            | AluOPRRR::Binv
            | AluOPRRR::Bset
            | AluOPRRR::Clmul
            | AluOPRRR::Clmulh
            | AluOPRRR::Clmulr
            | AluOPRRR::Max
            | AluOPRRR::Maxu
            | AluOPRRR::Min
            | AluOPRRR::Minu
            | AluOPRRR::Orn
            | AluOPRRR::Rol
            | AluOPRRR::Ror
            | AluOPRRR::Sh1add
            | AluOPRRR::Sh2add
            | AluOPRRR::Sh3add
            | AluOPRRR::Xnor
            | AluOPRRR::CzeroEqz
            | AluOPRRR::CzeroNez => 0b0110011,

            AluOPRRR::Rolw
            | AluOPRRR::Rorw
            | AluOPRRR::Sh2adduw
            | AluOPRRR::Sh3adduw
            | AluOPRRR::Sh1adduw => 0b0111011,
        }
    }

    pub const fn funct7(self) -> u32 {
        match self {
            AluOPRRR::Add => 0b0000000,
            AluOPRRR::Sub => 0b0100000,
            AluOPRRR::Sll => 0b0000000,
            AluOPRRR::Slt => 0b0000000,
            AluOPRRR::Sgt => 0b0000000,
            AluOPRRR::SltU => 0b0000000,
            AluOPRRR::Sgtu => 0b0000000,

            AluOPRRR::Xor => 0b0000000,
            AluOPRRR::Srl => 0b0000000,
            AluOPRRR::Sra => 0b0100000,
            AluOPRRR::Or => 0b0000000,
            AluOPRRR::And => 0b0000000,

            AluOPRRR::Addw => 0b0000000,
            AluOPRRR::Subw => 0b0100000,
            AluOPRRR::Sllw => 0b0000000,
            AluOPRRR::Srlw => 0b0000000,
            AluOPRRR::Sraw => 0b0100000,

            AluOPRRR::Mul => 0b0000001,
            AluOPRRR::Mulh => 0b0000001,
            AluOPRRR::Mulhsu => 0b0000001,
            AluOPRRR::Mulhu => 0b0000001,
            AluOPRRR::Div => 0b0000001,
            AluOPRRR::DivU => 0b0000001,
            AluOPRRR::Rem => 0b0000001,
            AluOPRRR::RemU => 0b0000001,

            AluOPRRR::Mulw => 0b0000001,
            AluOPRRR::Divw => 0b0000001,
            AluOPRRR::Divuw => 0b0000001,
            AluOPRRR::Remw => 0b0000001,
            AluOPRRR::Remuw => 0b0000001,
            AluOPRRR::Adduw => 0b0000100,
            AluOPRRR::Andn => 0b0100000,
            AluOPRRR::Bclr => 0b0100100,
            AluOPRRR::Bext => 0b0100100,
            AluOPRRR::Binv => 0b0110100,
            AluOPRRR::Bset => 0b0010100,
            AluOPRRR::Clmul => 0b0000101,
            AluOPRRR::Clmulh => 0b0000101,
            AluOPRRR::Clmulr => 0b0000101,
            AluOPRRR::Max => 0b0000101,
            AluOPRRR::Maxu => 0b0000101,
            AluOPRRR::Min => 0b0000101,
            AluOPRRR::Minu => 0b0000101,
            AluOPRRR::Orn => 0b0100000,
            AluOPRRR::Rol => 0b0110000,
            AluOPRRR::Rolw => 0b0110000,
            AluOPRRR::Ror => 0b0110000,
            AluOPRRR::Rorw => 0b0110000,
            AluOPRRR::Sh1add => 0b0010000,
            AluOPRRR::Sh1adduw => 0b0010000,
            AluOPRRR::Sh2add => 0b0010000,
            AluOPRRR::Sh2adduw => 0b0010000,
            AluOPRRR::Sh3add => 0b0010000,
            AluOPRRR::Sh3adduw => 0b0010000,
            AluOPRRR::Xnor => 0b0100000,

            // Zbkb
            AluOPRRR::Pack => 0b0000100,
            AluOPRRR::Packw => 0b0000100,
            AluOPRRR::Packh => 0b0000100,

            // ZiCond
            AluOPRRR::CzeroEqz => 0b0000111,
            AluOPRRR::CzeroNez => 0b0000111,
        }
    }

    pub(crate) fn reverse_rs(self) -> bool {
        // special case.
        // sgt and sgtu is not defined in isa.
        // emit should reverse rs1 and rs2.
        self == AluOPRRR::Sgt || self == AluOPRRR::Sgtu
    }
}

impl AluOPRRI {
    pub(crate) fn option_funct6(self) -> Option<u32> {
        let x: Option<u32> = match self {
            Self::Slli => Some(0b00_0000),
            Self::Srli => Some(0b00_0000),
            Self::Srai => Some(0b01_0000),
            Self::Bclri => Some(0b010010),
            Self::Bexti => Some(0b010010),
            Self::Binvi => Some(0b011010),
            Self::Bseti => Some(0b001010),
            Self::Rori => Some(0b011000),
            Self::SlliUw => Some(0b000010),
            _ => None,
        };
        x
    }

    pub(crate) fn option_funct7(self) -> Option<u32> {
        let x = match self {
            Self::Slliw => Some(0b000_0000),
            Self::SrliW => Some(0b000_0000),
            Self::Sraiw => Some(0b010_0000),
            Self::Roriw => Some(0b0110000),
            _ => None,
        };
        x
    }

    pub(crate) fn imm12(self, imm12: Imm12) -> u32 {
        let x = imm12.bits();
        if let Some(func) = self.option_funct6() {
            func << 6 | (x & 0b11_1111)
        } else if let Some(func) = self.option_funct7() {
            func << 5 | (x & 0b1_1111)
        } else if let Some(func) = self.option_funct12() {
            func
        } else {
            x
        }
    }

    pub(crate) fn option_funct12(self) -> Option<u32> {
        match self {
            Self::Clz => Some(0b011000000000),
            Self::Clzw => Some(0b011000000000),
            Self::Cpop => Some(0b011000000010),
            Self::Cpopw => Some(0b011000000010),
            Self::Ctz => Some(0b011000000001),
            Self::Ctzw => Some(0b011000000001),
            Self::Rev8 => Some(0b011010111000),
            Self::Sextb => Some(0b011000000100),
            Self::Sexth => Some(0b011000000101),
            Self::Zexth => Some(0b000010000000),
            Self::Orcb => Some(0b001010000111),
            Self::Brev8 => Some(0b0110_1000_0111),
            _ => None,
        }
    }

    pub(crate) fn op_name(self) -> &'static str {
        match self {
            Self::Addi => "addi",
            Self::Slti => "slti",
            Self::SltiU => "sltiu",
            Self::Xori => "xori",
            Self::Ori => "ori",
            Self::Andi => "andi",
            Self::Slli => "slli",
            Self::Srli => "srli",
            Self::Srai => "srai",
            Self::Addiw => "addiw",
            Self::Slliw => "slliw",
            Self::SrliW => "srliw",
            Self::Sraiw => "sraiw",
            Self::Bclri => "bclri",
            Self::Bexti => "bexti",
            Self::Binvi => "binvi",
            Self::Bseti => "bseti",
            Self::Rori => "rori",
            Self::Roriw => "roriw",
            Self::SlliUw => "slli.uw",
            Self::Clz => "clz",
            Self::Clzw => "clzw",
            Self::Cpop => "cpop",
            Self::Cpopw => "cpopw",
            Self::Ctz => "ctz",
            Self::Ctzw => "ctzw",
            Self::Rev8 => "rev8",
            Self::Sextb => "sext.b",
            Self::Sexth => "sext.h",
            Self::Zexth => "zext.h",
            Self::Orcb => "orc.b",
            Self::Brev8 => "brev8",
        }
    }

    pub fn funct3(self) -> u32 {
        match self {
            AluOPRRI::Addi => 0b000,
            AluOPRRI::Slti => 0b010,
            AluOPRRI::SltiU => 0b011,
            AluOPRRI::Xori => 0b100,
            AluOPRRI::Ori => 0b110,
            AluOPRRI::Andi => 0b111,
            AluOPRRI::Slli => 0b001,
            AluOPRRI::Srli => 0b101,
            AluOPRRI::Srai => 0b101,
            AluOPRRI::Addiw => 0b000,
            AluOPRRI::Slliw => 0b001,
            AluOPRRI::SrliW => 0b101,
            AluOPRRI::Sraiw => 0b101,
            AluOPRRI::Bclri => 0b001,
            AluOPRRI::Bexti => 0b101,
            AluOPRRI::Binvi => 0b001,
            AluOPRRI::Bseti => 0b001,
            AluOPRRI::Rori => 0b101,
            AluOPRRI::Roriw => 0b101,
            AluOPRRI::SlliUw => 0b001,
            AluOPRRI::Clz => 0b001,
            AluOPRRI::Clzw => 0b001,
            AluOPRRI::Cpop => 0b001,
            AluOPRRI::Cpopw => 0b001,
            AluOPRRI::Ctz => 0b001,
            AluOPRRI::Ctzw => 0b001,
            AluOPRRI::Rev8 => 0b101,
            AluOPRRI::Sextb => 0b001,
            AluOPRRI::Sexth => 0b001,
            AluOPRRI::Zexth => 0b100,
            AluOPRRI::Orcb => 0b101,
            AluOPRRI::Brev8 => 0b101,
        }
    }

    pub fn op_code(self) -> u32 {
        match self {
            AluOPRRI::Addi
            | AluOPRRI::Slti
            | AluOPRRI::SltiU
            | AluOPRRI::Xori
            | AluOPRRI::Ori
            | AluOPRRI::Andi
            | AluOPRRI::Slli
            | AluOPRRI::Srli
            | AluOPRRI::Srai
            | AluOPRRI::Bclri
            | AluOPRRI::Bexti
            | AluOPRRI::Binvi
            | AluOPRRI::Bseti
            | AluOPRRI::Rori
            | AluOPRRI::Clz
            | AluOPRRI::Cpop
            | AluOPRRI::Ctz
            | AluOPRRI::Rev8
            | AluOPRRI::Sextb
            | AluOPRRI::Sexth
            | AluOPRRI::Orcb
            | AluOPRRI::Brev8 => 0b0010011,

            AluOPRRI::Addiw
            | AluOPRRI::Slliw
            | AluOPRRI::SrliW
            | AluOPRRI::Sraiw
            | AluOPRRI::Roriw
            | AluOPRRI::SlliUw
            | AluOPRRI::Clzw
            | AluOPRRI::Cpopw
            | AluOPRRI::Ctzw => 0b0011011,
            AluOPRRI::Zexth => 0b0111011,
        }
    }
}

impl Default for FRM {
    fn default() -> Self {
        Self::Fcsr
    }
}

/// float rounding mode.
impl FRM {
    pub(crate) fn to_static_str(self) -> &'static str {
        match self {
            FRM::RNE => "rne",
            FRM::RTZ => "rtz",
            FRM::RDN => "rdn",
            FRM::RUP => "rup",
            FRM::RMM => "rmm",
            FRM::Fcsr => "fcsr",
        }
    }

    #[inline]
    pub(crate) fn bits(self) -> u8 {
        match self {
            FRM::RNE => 0b000,
            FRM::RTZ => 0b001,
            FRM::RDN => 0b010,
            FRM::RUP => 0b011,
            FRM::RMM => 0b100,
            FRM::Fcsr => 0b111,
        }
    }
    pub(crate) fn as_u32(self) -> u32 {
        self.bits() as u32
    }
}

impl FFlagsException {
    #[inline]
    #[expect(dead_code, reason = "here for future use")]
    pub(crate) fn mask(self) -> u32 {
        match self {
            FFlagsException::NV => 1 << 4,
            FFlagsException::DZ => 1 << 3,
            FFlagsException::OF => 1 << 2,
            FFlagsException::UF => 1 << 1,
            FFlagsException::NX => 1 << 0,
        }
    }
}

impl LoadOP {
    pub(crate) fn op_name(self) -> &'static str {
        match self {
            Self::Lb => "lb",
            Self::Lh => "lh",
            Self::Lw => "lw",
            Self::Lbu => "lbu",
            Self::Lhu => "lhu",
            Self::Lwu => "lwu",
            Self::Ld => "ld",
            Self::Flh => "flh",
            Self::Flw => "flw",
            Self::Fld => "fld",
        }
    }

    pub(crate) fn from_type(ty: Type) -> Self {
        match ty {
            F16 => Self::Flh,
            F32 => Self::Flw,
            F64 => Self::Fld,
            I8 => Self::Lb,
            I16 => Self::Lh,
            I32 => Self::Lw,
            I64 => Self::Ld,
            _ => unreachable!(),
        }
    }

    pub(crate) fn size(&self) -> i64 {
        match self {
            Self::Lb | Self::Lbu => 1,
            Self::Lh | Self::Lhu | Self::Flh => 2,
            Self::Lw | Self::Lwu | Self::Flw => 4,
            Self::Ld | Self::Fld => 8,
        }
    }

    pub(crate) fn op_code(self) -> u32 {
        match self {
            Self::Lb | Self::Lh | Self::Lw | Self::Lbu | Self::Lhu | Self::Lwu | Self::Ld => {
                0b0000011
            }
            Self::Flh | Self::Flw | Self::Fld => 0b0000111,
        }
    }
    pub(crate) fn funct3(self) -> u32 {
        match self {
            Self::Lb => 0b000,
            Self::Lh => 0b001,
            Self::Lw => 0b010,
            Self::Lwu => 0b110,
            Self::Lbu => 0b100,
            Self::Lhu => 0b101,
            Self::Ld => 0b011,
            Self::Flh => 0b001,
            Self::Flw => 0b010,
            Self::Fld => 0b011,
        }
    }
}

impl StoreOP {
    pub(crate) fn op_name(self) -> &'static str {
        match self {
            Self::Sb => "sb",
            Self::Sh => "sh",
            Self::Sw => "sw",
            Self::Sd => "sd",
            Self::Fsh => "fsh",
            Self::Fsw => "fsw",
            Self::Fsd => "fsd",
        }
    }
    pub(crate) fn from_type(ty: Type) -> Self {
        match ty {
            F16 => Self::Fsh,
            F32 => Self::Fsw,
            F64 => Self::Fsd,
            I8 => Self::Sb,
            I16 => Self::Sh,
            I32 => Self::Sw,
            I64 => Self::Sd,
            _ => unreachable!(),
        }
    }

    pub(crate) fn size(&self) -> i64 {
        match self {
            Self::Sb => 1,
            Self::Sh | Self::Fsh => 2,
            Self::Sw | Self::Fsw => 4,
            Self::Sd | Self::Fsd => 8,
        }
    }

    pub(crate) fn op_code(self) -> u32 {
        match self {
            Self::Sb | Self::Sh | Self::Sw | Self::Sd => 0b0100011,
            Self::Fsh | Self::Fsw | Self::Fsd => 0b0100111,
        }
    }
    pub(crate) fn funct3(self) -> u32 {
        match self {
            Self::Sb => 0b000,
            Self::Sh => 0b001,
            Self::Sw => 0b010,
            Self::Sd => 0b011,
            Self::Fsh => 0b001,
            Self::Fsw => 0b010,
            Self::Fsd => 0b011,
        }
    }
}

impl FClassResult {
    pub(crate) const fn bit(self) -> u32 {
        match self {
            FClassResult::NegInfinite => 1 << 0,
            FClassResult::NegNormal => 1 << 1,
            FClassResult::NegSubNormal => 1 << 2,
            FClassResult::NegZero => 1 << 3,
            FClassResult::PosZero => 1 << 4,
            FClassResult::PosSubNormal => 1 << 5,
            FClassResult::PosNormal => 1 << 6,
            FClassResult::PosInfinite => 1 << 7,
            FClassResult::SNaN => 1 << 8,
            FClassResult::QNaN => 1 << 9,
        }
    }

    #[inline]
    #[expect(dead_code, reason = "here for future use")]
    pub(crate) const fn is_nan_bits() -> u32 {
        Self::SNaN.bit() | Self::QNaN.bit()
    }
    #[inline]
    #[expect(dead_code, reason = "here for future use")]
    pub(crate) fn is_zero_bits() -> u32 {
        Self::NegZero.bit() | Self::PosZero.bit()
    }

    #[inline]
    #[expect(dead_code, reason = "here for future use")]
    pub(crate) fn is_infinite_bits() -> u32 {
        Self::PosInfinite.bit() | Self::NegInfinite.bit()
    }
}

impl AtomicOP {
    #[inline]
    pub(crate) fn is_load(self) -> bool {
        match self {
            Self::LrW | Self::LrD => true,
            _ => false,
        }
    }

    #[inline]
    pub(crate) fn op_name(self, amo: AMO) -> String {
        let s = match self {
            Self::LrW => "lr.w",
            Self::ScW => "sc.w",

            Self::AmoswapW => "amoswap.w",
            Self::AmoaddW => "amoadd.w",
            Self::AmoxorW => "amoxor.w",
            Self::AmoandW => "amoand.w",
            Self::AmoorW => "amoor.w",
            Self::AmominW => "amomin.w",
            Self::AmomaxW => "amomax.w",
            Self::AmominuW => "amominu.w",
            Self::AmomaxuW => "amomaxu.w",
            Self::LrD => "lr.d",
            Self::ScD => "sc.d",
            Self::AmoswapD => "amoswap.d",
            Self::AmoaddD => "amoadd.d",
            Self::AmoxorD => "amoxor.d",
            Self::AmoandD => "amoand.d",
            Self::AmoorD => "amoor.d",
            Self::AmominD => "amomin.d",
            Self::AmomaxD => "amomax.d",
            Self::AmominuD => "amominu.d",
            Self::AmomaxuD => "amomaxu.d",
        };
        format!("{}{}", s, amo.to_static_str())
    }
    #[inline]
    pub(crate) fn op_code(self) -> u32 {
        0b0101111
    }

    #[inline]
    pub(crate) fn funct7(self, amo: AMO) -> u32 {
        self.funct5() << 2 | amo.as_u32() & 0b11
    }

    pub(crate) fn funct3(self) -> u32 {
        match self {
            AtomicOP::LrW
            | AtomicOP::ScW
            | AtomicOP::AmoswapW
            | AtomicOP::AmoaddW
            | AtomicOP::AmoxorW
            | AtomicOP::AmoandW
            | AtomicOP::AmoorW
            | AtomicOP::AmominW
            | AtomicOP::AmomaxW
            | AtomicOP::AmominuW
            | AtomicOP::AmomaxuW => 0b010,
            AtomicOP::LrD
            | AtomicOP::ScD
            | AtomicOP::AmoswapD
            | AtomicOP::AmoaddD
            | AtomicOP::AmoxorD
            | AtomicOP::AmoandD
            | AtomicOP::AmoorD
            | AtomicOP::AmominD
            | AtomicOP::AmomaxD
            | AtomicOP::AmominuD
            | AtomicOP::AmomaxuD => 0b011,
        }
    }
    pub(crate) fn funct5(self) -> u32 {
        match self {
            AtomicOP::LrW => 0b00010,
            AtomicOP::ScW => 0b00011,
            AtomicOP::AmoswapW => 0b00001,
            AtomicOP::AmoaddW => 0b00000,
            AtomicOP::AmoxorW => 0b00100,
            AtomicOP::AmoandW => 0b01100,
            AtomicOP::AmoorW => 0b01000,
            AtomicOP::AmominW => 0b10000,
            AtomicOP::AmomaxW => 0b10100,
            AtomicOP::AmominuW => 0b11000,
            AtomicOP::AmomaxuW => 0b11100,
            AtomicOP::LrD => 0b00010,
            AtomicOP::ScD => 0b00011,
            AtomicOP::AmoswapD => 0b00001,
            AtomicOP::AmoaddD => 0b00000,
            AtomicOP::AmoxorD => 0b00100,
            AtomicOP::AmoandD => 0b01100,
            AtomicOP::AmoorD => 0b01000,
            AtomicOP::AmominD => 0b10000,
            AtomicOP::AmomaxD => 0b10100,
            AtomicOP::AmominuD => 0b11000,
            AtomicOP::AmomaxuD => 0b11100,
        }
    }

    pub(crate) fn load_op(t: Type) -> Self {
        if t.bits() <= 32 { Self::LrW } else { Self::LrD }
    }
    pub(crate) fn store_op(t: Type) -> Self {
        if t.bits() <= 32 { Self::ScW } else { Self::ScD }
    }

    /// extract
    pub(crate) fn extract(rd: WritableReg, offset: Reg, rs: Reg, ty: Type) -> SmallInstVec<Inst> {
        let mut insts = SmallInstVec::new();
        insts.push(Inst::AluRRR {
            alu_op: AluOPRRR::Srl,
            rd,
            rs1: rs,
            rs2: offset,
        });
        //
        insts.push(Inst::Extend {
            rd,
            rn: rd.to_reg(),
            signed: false,
            from_bits: ty.bits() as u8,
            to_bits: 64,
        });
        insts
    }

    /// like extract but sign extend the value.
    /// suitable for smax,etc.
    pub(crate) fn extract_sext(
        rd: WritableReg,
        offset: Reg,
        rs: Reg,
        ty: Type,
    ) -> SmallInstVec<Inst> {
        let mut insts = SmallInstVec::new();
        insts.push(Inst::AluRRR {
            alu_op: AluOPRRR::Srl,
            rd,
            rs1: rs,
            rs2: offset,
        });
        //
        insts.push(Inst::Extend {
            rd,
            rn: rd.to_reg(),
            signed: true,
            from_bits: ty.bits() as u8,
            to_bits: 64,
        });
        insts
    }

    pub(crate) fn unset(
        rd: WritableReg,
        tmp: WritableReg,
        offset: Reg,
        ty: Type,
    ) -> SmallInstVec<Inst> {
        assert!(rd != tmp);
        let mut insts = SmallInstVec::new();
        insts.extend(Inst::load_int_mask(tmp, ty));
        insts.push(Inst::AluRRR {
            alu_op: AluOPRRR::Sll,
            rd: tmp,
            rs1: tmp.to_reg(),
            rs2: offset,
        });
        insts.push(Inst::construct_bit_not(tmp, tmp.to_reg()));
        insts.push(Inst::AluRRR {
            alu_op: AluOPRRR::And,
            rd,
            rs1: rd.to_reg(),
            rs2: tmp.to_reg(),
        });
        insts
    }

    pub(crate) fn set(
        rd: WritableReg,
        tmp: WritableReg,
        offset: Reg,
        rs: Reg,
        ty: Type,
    ) -> SmallInstVec<Inst> {
        assert!(rd != tmp);
        let mut insts = SmallInstVec::new();
        // make rs into tmp.
        insts.push(Inst::Extend {
            rd: tmp,
            rn: rs,
            signed: false,
            from_bits: ty.bits() as u8,
            to_bits: 64,
        });
        insts.push(Inst::AluRRR {
            alu_op: AluOPRRR::Sll,
            rd: tmp,
            rs1: tmp.to_reg(),
            rs2: offset,
        });
        insts.push(Inst::AluRRR {
            alu_op: AluOPRRR::Or,
            rd,
            rs1: rd.to_reg(),
            rs2: tmp.to_reg(),
        });
        insts
    }

    /// Merge reset part of rs into rd.
    /// Call this function must make sure that other part of value is already in rd.
    pub(crate) fn merge(
        rd: WritableReg,
        tmp: WritableReg,
        offset: Reg,
        rs: Reg,
        ty: Type,
    ) -> SmallInstVec<Inst> {
        let mut insts = Self::unset(rd, tmp, offset, ty);
        insts.extend(Self::set(rd, tmp, offset, rs, ty));
        insts
    }
}

///Atomic Memory ordering.
#[derive(Copy, Clone, Debug)]
pub enum AMO {
    Relax = 0b00,
    Release = 0b01,
    Acquire = 0b10,
    SeqCst = 0b11,
}

impl AMO {
    pub(crate) fn to_static_str(self) -> &'static str {
        match self {
            AMO::Relax => "",
            AMO::Release => ".rl",
            AMO::Acquire => ".aq",
            AMO::SeqCst => ".aqrl",
        }
    }
    pub(crate) fn as_u32(self) -> u32 {
        self as u32
    }
}

impl Inst {
    /// fence request bits.
    pub(crate) const FENCE_REQ_I: u8 = 1 << 3;
    pub(crate) const FENCE_REQ_O: u8 = 1 << 2;
    pub(crate) const FENCE_REQ_R: u8 = 1 << 1;
    pub(crate) const FENCE_REQ_W: u8 = 1 << 0;
    pub(crate) fn fence_req_to_string(x: u8) -> String {
        let mut s = String::default();
        if x & Self::FENCE_REQ_I != 0 {
            s.push_str("i");
        }
        if x & Self::FENCE_REQ_O != 0 {
            s.push_str("o");
        }
        if x & Self::FENCE_REQ_R != 0 {
            s.push_str("r");
        }
        if x & Self::FENCE_REQ_W != 0 {
            s.push_str("w");
        }
        s
    }
}

impl CsrRegOP {
    pub(crate) fn funct3(self) -> u32 {
        match self {
            CsrRegOP::CsrRW => 0b001,
            CsrRegOP::CsrRS => 0b010,
            CsrRegOP::CsrRC => 0b011,
        }
    }

    pub(crate) fn opcode(self) -> u32 {
        0b1110011
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            CsrRegOP::CsrRW => "csrrw",
            CsrRegOP::CsrRS => "csrrs",
            CsrRegOP::CsrRC => "csrrc",
        }
    }
}

impl Display for CsrRegOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name())
    }
}

impl CsrImmOP {
    pub(crate) fn funct3(self) -> u32 {
        match self {
            CsrImmOP::CsrRWI => 0b101,
            CsrImmOP::CsrRSI => 0b110,
            CsrImmOP::CsrRCI => 0b111,
        }
    }

    pub(crate) fn opcode(self) -> u32 {
        0b1110011
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            CsrImmOP::CsrRWI => "csrrwi",
            CsrImmOP::CsrRSI => "csrrsi",
            CsrImmOP::CsrRCI => "csrrci",
        }
    }
}

impl Display for CsrImmOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name())
    }
}

impl CSR {
    pub(crate) fn bits(self) -> Imm12 {
        Imm12::from_i16(match self {
            CSR::Frm => 0x0002,
        })
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            CSR::Frm => "frm",
        }
    }
}

impl Display for CSR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name())
    }
}

impl COpcodeSpace {
    pub fn bits(&self) -> u32 {
        match self {
            COpcodeSpace::C0 => 0b00,
            COpcodeSpace::C1 => 0b01,
            COpcodeSpace::C2 => 0b10,
        }
    }
}

impl CrOp {
    pub fn funct4(&self) -> u32 {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            // `c.jr` has the same op/funct4 as C.MV, but RS2 is 0, which is illegal for mv.
            CrOp::CMv | CrOp::CJr => 0b1000,
            CrOp::CAdd | CrOp::CJalr | CrOp::CEbreak => 0b1001,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            CrOp::CMv | CrOp::CAdd | CrOp::CJr | CrOp::CJalr | CrOp::CEbreak => COpcodeSpace::C2,
        }
    }
}

impl CaOp {
    pub fn funct2(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CaOp::CAnd => 0b11,
            CaOp::COr => 0b10,
            CaOp::CXor => 0b01,
            CaOp::CSub => 0b00,
            CaOp::CAddw => 0b01,
            CaOp::CSubw => 0b00,
            CaOp::CMul => 0b10,
        }
    }

    pub fn funct6(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CaOp::CAnd | CaOp::COr | CaOp::CXor | CaOp::CSub => 0b100_011,
            CaOp::CSubw | CaOp::CAddw | CaOp::CMul => 0b100_111,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            CaOp::CAnd
            | CaOp::COr
            | CaOp::CXor
            | CaOp::CSub
            | CaOp::CAddw
            | CaOp::CSubw
            | CaOp::CMul => COpcodeSpace::C1,
        }
    }
}

impl CjOp {
    pub fn funct3(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CjOp::CJ => 0b101,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            CjOp::CJ => COpcodeSpace::C1,
        }
    }
}

impl CiOp {
    pub fn funct3(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CiOp::CAddi | CiOp::CSlli => 0b000,
            CiOp::CAddiw | CiOp::CFldsp => 0b001,
            CiOp::CLi | CiOp::CLwsp => 0b010,
            CiOp::CAddi16sp | CiOp::CLui | CiOp::CLdsp => 0b011,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            CiOp::CAddi | CiOp::CAddiw | CiOp::CAddi16sp | CiOp::CLi | CiOp::CLui => {
                COpcodeSpace::C1
            }
            CiOp::CSlli | CiOp::CLwsp | CiOp::CLdsp | CiOp::CFldsp => COpcodeSpace::C2,
        }
    }
}

impl CiwOp {
    pub fn funct3(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CiwOp::CAddi4spn => 0b000,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            CiwOp::CAddi4spn => COpcodeSpace::C0,
        }
    }
}

impl CbOp {
    pub fn funct3(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CbOp::CSrli | CbOp::CSrai | CbOp::CAndi => 0b100,
        }
    }

    pub fn funct2(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CbOp::CSrli => 0b00,
            CbOp::CSrai => 0b01,
            CbOp::CAndi => 0b10,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            CbOp::CSrli | CbOp::CSrai | CbOp::CAndi => COpcodeSpace::C1,
        }
    }
}

impl CssOp {
    pub fn funct3(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CssOp::CFsdsp => 0b101,
            CssOp::CSwsp => 0b110,
            CssOp::CSdsp => 0b111,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            CssOp::CSwsp | CssOp::CSdsp | CssOp::CFsdsp => COpcodeSpace::C2,
        }
    }
}

impl CsOp {
    pub fn funct3(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CsOp::CFsd => 0b101,
            CsOp::CSw => 0b110,
            CsOp::CSd => 0b111,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            CsOp::CSw | CsOp::CSd | CsOp::CFsd => COpcodeSpace::C0,
        }
    }
}

impl ClOp {
    pub fn funct3(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            ClOp::CFld => 0b001,
            ClOp::CLw => 0b010,
            ClOp::CLd => 0b011,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            ClOp::CLw | ClOp::CLd | ClOp::CFld => COpcodeSpace::C0,
        }
    }
}

impl CsznOp {
    pub fn funct6(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CsznOp::CNot
            | CsznOp::CZextw
            | CsznOp::CZextb
            | CsznOp::CZexth
            | CsznOp::CSextb
            | CsznOp::CSexth => 0b100_111,
        }
    }

    pub fn funct5(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            CsznOp::CNot => 0b11_101,
            CsznOp::CZextb => 0b11_000,
            CsznOp::CZexth => 0b11_010,
            CsznOp::CZextw => 0b11_100,
            CsznOp::CSextb => 0b11_001,
            CsznOp::CSexth => 0b11_011,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            CsznOp::CNot
            | CsznOp::CZextb
            | CsznOp::CZexth
            | CsznOp::CZextw
            | CsznOp::CSextb
            | CsznOp::CSexth => COpcodeSpace::C1,
        }
    }
}

impl ZcbMemOp {
    pub fn funct6(&self) -> u32 {
        // https://github.com/michaeljclark/riscv-meta/blob/master/opcodes
        match self {
            ZcbMemOp::CLbu => 0b100_000,
            // These two opcodes are differentiated in the imm field of the instruction.
            ZcbMemOp::CLhu | ZcbMemOp::CLh => 0b100_001,
            ZcbMemOp::CSb => 0b100_010,
            ZcbMemOp::CSh => 0b100_011,
        }
    }

    pub fn imm_bits(&self) -> u8 {
        match self {
            ZcbMemOp::CLhu | ZcbMemOp::CLh | ZcbMemOp::CSh => 1,
            ZcbMemOp::CLbu | ZcbMemOp::CSb => 2,
        }
    }

    pub fn op(&self) -> COpcodeSpace {
        // https://five-embeddev.com/riscv-isa-manual/latest/rvc-opcode-map.html#rvcopcodemap
        match self {
            ZcbMemOp::CLbu | ZcbMemOp::CLhu | ZcbMemOp::CLh | ZcbMemOp::CSb | ZcbMemOp::CSh => {
                COpcodeSpace::C0
            }
        }
    }
}
