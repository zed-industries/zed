//! Pulley instruction arguments.

use super::*;
use crate::ir::ExternalName;
use crate::machinst::abi::StackAMode;
use pulley_interpreter::encode;
use pulley_interpreter::regs::Reg as _;
use std::fmt;

/// A macro for defining a newtype of `Reg` that enforces some invariant about
/// the wrapped `Reg` (such as that it is of a particular register class).
macro_rules! newtype_of_reg {
    (
        $newtype_reg:ident,
        $newtype_writable_reg:ident,
        $class:expr
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

        impl TryFrom<Reg> for $newtype_reg {
            type Error = ();
            fn try_from(r: Reg) -> Result<Self, Self::Error> {
                Self::new(r).ok_or(())
            }
        }

        impl $newtype_reg {
            /// Create this newtype from the given register, or return `None` if the register
            /// is not a valid instance of this newtype.
            pub fn new(reg: Reg) -> Option<Self> {
                if reg.class() == $class {
                    Some(Self(reg))
                } else {
                    None
                }
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

        /// If you know what you're doing, you can explicitly mutably borrow the
        /// underlying `Reg`. Don't make it point to the wrong type of register
        /// please.
        impl AsMut<Reg> for $newtype_reg {
            fn as_mut(&mut self) -> &mut Reg {
                &mut self.0
            }
        }

        /// Writable Reg.
        pub type $newtype_writable_reg = Writable<$newtype_reg>;

        impl From<pulley_interpreter::regs::$newtype_reg> for $newtype_reg {
            fn from(r: pulley_interpreter::regs::$newtype_reg) -> Self {
                Self::new(regalloc2::PReg::new(usize::from(r as u8), $class).into()).unwrap()
            }
        }
        impl From<$newtype_reg> for pulley_interpreter::regs::$newtype_reg {
            fn from(r: $newtype_reg) -> Self {
                Self::new(r.to_real_reg().unwrap().hw_enc()).unwrap()
            }
        }
        impl<'a> From<&'a $newtype_reg> for pulley_interpreter::regs::$newtype_reg {
            fn from(r: &'a $newtype_reg) -> Self {
                Self::new(r.to_real_reg().unwrap().hw_enc()).unwrap()
            }
        }
        impl From<$newtype_writable_reg> for pulley_interpreter::regs::$newtype_reg {
            fn from(r: $newtype_writable_reg) -> Self {
                Self::new(r.to_reg().to_real_reg().unwrap().hw_enc()).unwrap()
            }
        }
        impl<'a> From<&'a $newtype_writable_reg> for pulley_interpreter::regs::$newtype_reg {
            fn from(r: &'a $newtype_writable_reg) -> Self {
                Self::new(r.to_reg().to_real_reg().unwrap().hw_enc()).unwrap()
            }
        }

        impl TryFrom<Writable<Reg>> for $newtype_writable_reg {
            type Error = ();
            fn try_from(r: Writable<Reg>) -> Result<Self, Self::Error> {
                let r = r.to_reg();
                match $newtype_reg::new(r) {
                    Some(r) => Ok(Writable::from_reg(r)),
                    None => Err(()),
                }
            }
        }
    };
}

// Newtypes for registers classes.
newtype_of_reg!(XReg, WritableXReg, RegClass::Int);
newtype_of_reg!(FReg, WritableFReg, RegClass::Float);
newtype_of_reg!(VReg, WritableVReg, RegClass::Vector);

impl XReg {
    /// Index of the first "special" register, or the end of which registers
    /// regalloc is allowed to use.
    pub const SPECIAL_START: u8 = pulley_interpreter::regs::XReg::SPECIAL_START;

    /// Returns whether this is a "special" physical register for pulley.
    pub fn is_special(&self) -> bool {
        match self.as_pulley() {
            Some(reg) => reg.is_special(),
            None => false,
        }
    }

    /// Returns the pulley-typed register, if this is a physical register.
    pub fn as_pulley(&self) -> Option<pulley_interpreter::XReg> {
        let enc = self.to_real_reg()?.hw_enc();
        Some(pulley_interpreter::XReg::new(enc).unwrap())
    }
}

pub use super::super::lower::isle::generated_code::Amode;

impl Amode {
    /// Add the registers referenced by this Amode to `collector`.
    pub(crate) fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            Amode::RegOffset { base, offset: _ } => collector.reg_use(base),
            // Registers used in these modes aren't allocatable.
            Amode::SpOffset { .. } | Amode::Stack { .. } => {}
        }
    }

    pub(crate) fn get_base_register(&self) -> Option<XReg> {
        match self {
            Amode::RegOffset { base, offset: _ } => Some(*base),
            Amode::SpOffset { .. } | Amode::Stack { .. } => Some(XReg::new(stack_reg()).unwrap()),
        }
    }

    pub(crate) fn get_offset_with_state<P>(&self, state: &EmitState<P>) -> i32
    where
        P: PulleyTargetKind,
    {
        match self {
            Amode::RegOffset { base: _, offset } | Amode::SpOffset { offset } => *offset,
            Amode::Stack { amode } => {
                let offset64 = match amode {
                    StackAMode::IncomingArg(offset, stack_args_size) => {
                        let offset = i64::from(*stack_args_size) - *offset;
                        let frame_layout = state.frame_layout();
                        let sp_offset = frame_layout.tail_args_size
                            + frame_layout.setup_area_size
                            + frame_layout.clobber_size
                            + frame_layout.fixed_frame_storage_size
                            + frame_layout.outgoing_args_size;
                        i64::from(sp_offset) - offset
                    }
                    StackAMode::Slot(offset) => {
                        offset + i64::from(state.frame_layout().outgoing_args_size)
                    }
                    StackAMode::OutgoingArg(offset) => *offset,
                };
                i32::try_from(offset64).unwrap()
            }
        }
    }
}

impl core::fmt::Display for Amode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Amode::SpOffset { offset } => {
                if *offset >= 0 {
                    write!(f, "sp+{offset}")
                } else {
                    write!(f, "sp{offset}")
                }
            }
            Amode::RegOffset { base, offset } => {
                let name = reg_name(**base);
                if *offset >= 0 {
                    write!(f, "{name}+{offset}")
                } else {
                    write!(f, "{name}{offset}")
                }
            }
            Amode::Stack { amode } => core::fmt::Debug::fmt(amode, f),
        }
    }
}

impl From<StackAMode> for Amode {
    fn from(amode: StackAMode) -> Self {
        Amode::Stack { amode }
    }
}

/// The size of an operand or operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperandSize {
    /// 32 bits.
    Size32,
    /// 64 bits.
    Size64,
}

pub use crate::isa::pulley_shared::lower::isle::generated_code::Cond;

impl Cond {
    /// Collect register operands within `collector` for register allocation.
    pub fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            Cond::If32 { reg } | Cond::IfNot32 { reg } => collector.reg_use(reg),

            Cond::IfXeq32 { src1, src2 }
            | Cond::IfXneq32 { src1, src2 }
            | Cond::IfXslt32 { src1, src2 }
            | Cond::IfXslteq32 { src1, src2 }
            | Cond::IfXult32 { src1, src2 }
            | Cond::IfXulteq32 { src1, src2 }
            | Cond::IfXeq64 { src1, src2 }
            | Cond::IfXneq64 { src1, src2 }
            | Cond::IfXslt64 { src1, src2 }
            | Cond::IfXslteq64 { src1, src2 }
            | Cond::IfXult64 { src1, src2 }
            | Cond::IfXulteq64 { src1, src2 } => {
                collector.reg_use(src1);
                collector.reg_use(src2);
            }

            Cond::IfXeq32I32 { src1, src2 }
            | Cond::IfXneq32I32 { src1, src2 }
            | Cond::IfXslt32I32 { src1, src2 }
            | Cond::IfXslteq32I32 { src1, src2 }
            | Cond::IfXsgt32I32 { src1, src2 }
            | Cond::IfXsgteq32I32 { src1, src2 }
            | Cond::IfXeq64I32 { src1, src2 }
            | Cond::IfXneq64I32 { src1, src2 }
            | Cond::IfXslt64I32 { src1, src2 }
            | Cond::IfXslteq64I32 { src1, src2 }
            | Cond::IfXsgt64I32 { src1, src2 }
            | Cond::IfXsgteq64I32 { src1, src2 } => {
                collector.reg_use(src1);
                let _: &mut i32 = src2;
            }

            Cond::IfXult32I32 { src1, src2 }
            | Cond::IfXulteq32I32 { src1, src2 }
            | Cond::IfXugt32I32 { src1, src2 }
            | Cond::IfXugteq32I32 { src1, src2 }
            | Cond::IfXult64I32 { src1, src2 }
            | Cond::IfXulteq64I32 { src1, src2 }
            | Cond::IfXugt64I32 { src1, src2 }
            | Cond::IfXugteq64I32 { src1, src2 } => {
                collector.reg_use(src1);
                let _: &mut u32 = src2;
            }
        }
    }

    /// Encode this condition as a branch into `sink`.
    ///
    /// Note that the offset encoded to jump by is filled in as 0 and it's
    /// assumed `MachBuffer` will come back and clean it up.
    pub fn encode(&self, sink: &mut impl Extend<u8>) {
        match *self {
            Cond::If32 { reg } => encode::br_if32(sink, reg, 0),
            Cond::IfNot32 { reg } => encode::br_if_not32(sink, reg, 0),
            Cond::IfXeq32 { src1, src2 } => encode::br_if_xeq32(sink, src1, src2, 0),
            Cond::IfXneq32 { src1, src2 } => encode::br_if_xneq32(sink, src1, src2, 0),
            Cond::IfXslt32 { src1, src2 } => encode::br_if_xslt32(sink, src1, src2, 0),
            Cond::IfXslteq32 { src1, src2 } => encode::br_if_xslteq32(sink, src1, src2, 0),
            Cond::IfXult32 { src1, src2 } => encode::br_if_xult32(sink, src1, src2, 0),
            Cond::IfXulteq32 { src1, src2 } => encode::br_if_xulteq32(sink, src1, src2, 0),
            Cond::IfXeq64 { src1, src2 } => encode::br_if_xeq64(sink, src1, src2, 0),
            Cond::IfXneq64 { src1, src2 } => encode::br_if_xneq64(sink, src1, src2, 0),
            Cond::IfXslt64 { src1, src2 } => encode::br_if_xslt64(sink, src1, src2, 0),
            Cond::IfXslteq64 { src1, src2 } => encode::br_if_xslteq64(sink, src1, src2, 0),
            Cond::IfXult64 { src1, src2 } => encode::br_if_xult64(sink, src1, src2, 0),
            Cond::IfXulteq64 { src1, src2 } => encode::br_if_xulteq64(sink, src1, src2, 0),

            Cond::IfXeq32I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xeq32_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xeq32_i32(sink, src1, src2, 0),
            },
            Cond::IfXneq32I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xneq32_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xneq32_i32(sink, src1, src2, 0),
            },
            Cond::IfXslt32I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xslt32_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xslt32_i32(sink, src1, src2, 0),
            },
            Cond::IfXslteq32I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xslteq32_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xslteq32_i32(sink, src1, src2, 0),
            },
            Cond::IfXsgt32I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xsgt32_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xsgt32_i32(sink, src1, src2, 0),
            },
            Cond::IfXsgteq32I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xsgteq32_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xsgteq32_i32(sink, src1, src2, 0),
            },
            Cond::IfXult32I32 { src1, src2 } => match u8::try_from(src2) {
                Ok(src2) => encode::br_if_xult32_u8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xult32_u32(sink, src1, src2, 0),
            },
            Cond::IfXulteq32I32 { src1, src2 } => match u8::try_from(src2) {
                Ok(src2) => encode::br_if_xulteq32_u8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xulteq32_u32(sink, src1, src2, 0),
            },
            Cond::IfXugt32I32 { src1, src2 } => match u8::try_from(src2) {
                Ok(src2) => encode::br_if_xugt32_u8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xugt32_u32(sink, src1, src2, 0),
            },
            Cond::IfXugteq32I32 { src1, src2 } => match u8::try_from(src2) {
                Ok(src2) => encode::br_if_xugteq32_u8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xugteq32_u32(sink, src1, src2, 0),
            },

            Cond::IfXeq64I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xeq64_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xeq64_i32(sink, src1, src2, 0),
            },
            Cond::IfXneq64I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xneq64_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xneq64_i32(sink, src1, src2, 0),
            },
            Cond::IfXslt64I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xslt64_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xslt64_i32(sink, src1, src2, 0),
            },
            Cond::IfXslteq64I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xslteq64_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xslteq64_i32(sink, src1, src2, 0),
            },
            Cond::IfXsgt64I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xsgt64_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xsgt64_i32(sink, src1, src2, 0),
            },
            Cond::IfXsgteq64I32 { src1, src2 } => match i8::try_from(src2) {
                Ok(src2) => encode::br_if_xsgteq64_i8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xsgteq64_i32(sink, src1, src2, 0),
            },
            Cond::IfXult64I32 { src1, src2 } => match u8::try_from(src2) {
                Ok(src2) => encode::br_if_xult64_u8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xult64_u32(sink, src1, src2, 0),
            },
            Cond::IfXulteq64I32 { src1, src2 } => match u8::try_from(src2) {
                Ok(src2) => encode::br_if_xulteq64_u8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xulteq64_u32(sink, src1, src2, 0),
            },
            Cond::IfXugt64I32 { src1, src2 } => match u8::try_from(src2) {
                Ok(src2) => encode::br_if_xugt64_u8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xugt64_u32(sink, src1, src2, 0),
            },
            Cond::IfXugteq64I32 { src1, src2 } => match u8::try_from(src2) {
                Ok(src2) => encode::br_if_xugteq64_u8(sink, src1, src2, 0),
                Err(_) => encode::br_if_xugteq64_u32(sink, src1, src2, 0),
            },
        }
    }

    /// Inverts this conditional.
    pub fn invert(&self) -> Cond {
        match *self {
            Cond::If32 { reg } => Cond::IfNot32 { reg },
            Cond::IfNot32 { reg } => Cond::If32 { reg },
            Cond::IfXeq32 { src1, src2 } => Cond::IfXneq32 { src1, src2 },
            Cond::IfXneq32 { src1, src2 } => Cond::IfXeq32 { src1, src2 },
            Cond::IfXeq64 { src1, src2 } => Cond::IfXneq64 { src1, src2 },
            Cond::IfXneq64 { src1, src2 } => Cond::IfXeq64 { src1, src2 },

            // Note that for below the condition changes but the operands are
            // also swapped.
            Cond::IfXslt32 { src1, src2 } => Cond::IfXslteq32 {
                src1: src2,
                src2: src1,
            },
            Cond::IfXslteq32 { src1, src2 } => Cond::IfXslt32 {
                src1: src2,
                src2: src1,
            },
            Cond::IfXult32 { src1, src2 } => Cond::IfXulteq32 {
                src1: src2,
                src2: src1,
            },
            Cond::IfXulteq32 { src1, src2 } => Cond::IfXult32 {
                src1: src2,
                src2: src1,
            },
            Cond::IfXslt64 { src1, src2 } => Cond::IfXslteq64 {
                src1: src2,
                src2: src1,
            },
            Cond::IfXslteq64 { src1, src2 } => Cond::IfXslt64 {
                src1: src2,
                src2: src1,
            },
            Cond::IfXult64 { src1, src2 } => Cond::IfXulteq64 {
                src1: src2,
                src2: src1,
            },
            Cond::IfXulteq64 { src1, src2 } => Cond::IfXult64 {
                src1: src2,
                src2: src1,
            },

            Cond::IfXeq32I32 { src1, src2 } => Cond::IfXneq32I32 { src1, src2 },
            Cond::IfXneq32I32 { src1, src2 } => Cond::IfXeq32I32 { src1, src2 },
            Cond::IfXslt32I32 { src1, src2 } => Cond::IfXsgteq32I32 { src1, src2 },
            Cond::IfXslteq32I32 { src1, src2 } => Cond::IfXsgt32I32 { src1, src2 },
            Cond::IfXult32I32 { src1, src2 } => Cond::IfXugteq32I32 { src1, src2 },
            Cond::IfXulteq32I32 { src1, src2 } => Cond::IfXugt32I32 { src1, src2 },
            Cond::IfXsgt32I32 { src1, src2 } => Cond::IfXslteq32I32 { src1, src2 },
            Cond::IfXsgteq32I32 { src1, src2 } => Cond::IfXslt32I32 { src1, src2 },
            Cond::IfXugt32I32 { src1, src2 } => Cond::IfXulteq32I32 { src1, src2 },
            Cond::IfXugteq32I32 { src1, src2 } => Cond::IfXult32I32 { src1, src2 },

            Cond::IfXeq64I32 { src1, src2 } => Cond::IfXneq64I32 { src1, src2 },
            Cond::IfXneq64I32 { src1, src2 } => Cond::IfXeq64I32 { src1, src2 },
            Cond::IfXslt64I32 { src1, src2 } => Cond::IfXsgteq64I32 { src1, src2 },
            Cond::IfXslteq64I32 { src1, src2 } => Cond::IfXsgt64I32 { src1, src2 },
            Cond::IfXult64I32 { src1, src2 } => Cond::IfXugteq64I32 { src1, src2 },
            Cond::IfXulteq64I32 { src1, src2 } => Cond::IfXugt64I32 { src1, src2 },
            Cond::IfXsgt64I32 { src1, src2 } => Cond::IfXslteq64I32 { src1, src2 },
            Cond::IfXsgteq64I32 { src1, src2 } => Cond::IfXslt64I32 { src1, src2 },
            Cond::IfXugt64I32 { src1, src2 } => Cond::IfXulteq64I32 { src1, src2 },
            Cond::IfXugteq64I32 { src1, src2 } => Cond::IfXult64I32 { src1, src2 },
        }
    }
}

impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cond::If32 { reg } => write!(f, "if32 {}", reg_name(**reg)),
            Cond::IfNot32 { reg } => write!(f, "if_not32 {}", reg_name(**reg)),
            Cond::IfXeq32 { src1, src2 } => {
                write!(f, "if_xeq32 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXneq32 { src1, src2 } => {
                write!(f, "if_xneq32 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXslt32 { src1, src2 } => {
                write!(f, "if_xslt32 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXslteq32 { src1, src2 } => {
                write!(f, "if_xslteq32 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXult32 { src1, src2 } => {
                write!(f, "if_xult32 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXulteq32 { src1, src2 } => {
                write!(f, "if_xulteq32 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXeq64 { src1, src2 } => {
                write!(f, "if_xeq64 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXneq64 { src1, src2 } => {
                write!(f, "if_xneq64 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXslt64 { src1, src2 } => {
                write!(f, "if_xslt64 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXslteq64 { src1, src2 } => {
                write!(f, "if_xslteq64 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXult64 { src1, src2 } => {
                write!(f, "if_xult64 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXulteq64 { src1, src2 } => {
                write!(f, "if_xulteq64 {}, {}", reg_name(**src1), reg_name(**src2))
            }
            Cond::IfXeq32I32 { src1, src2 } => {
                write!(f, "if_xeq32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXneq32I32 { src1, src2 } => {
                write!(f, "if_xneq32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXslt32I32 { src1, src2 } => {
                write!(f, "if_xslt32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXslteq32I32 { src1, src2 } => {
                write!(f, "if_xslteq32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXsgt32I32 { src1, src2 } => {
                write!(f, "if_xsgt32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXsgteq32I32 { src1, src2 } => {
                write!(f, "if_xsgteq32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXult32I32 { src1, src2 } => {
                write!(f, "if_xult32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXulteq32I32 { src1, src2 } => {
                write!(f, "if_xulteq32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXugt32I32 { src1, src2 } => {
                write!(f, "if_xugt32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXugteq32I32 { src1, src2 } => {
                write!(f, "if_xugteq32_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXeq64I32 { src1, src2 } => {
                write!(f, "if_xeq64_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXneq64I32 { src1, src2 } => {
                write!(f, "if_xneq64_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXslt64I32 { src1, src2 } => {
                write!(f, "if_xslt64_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXslteq64I32 { src1, src2 } => {
                write!(f, "if_xslteq64_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXsgt64I32 { src1, src2 } => {
                write!(f, "if_xsgt64_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXsgteq64I32 { src1, src2 } => {
                write!(f, "if_xsgteq64_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXult64I32 { src1, src2 } => {
                write!(f, "if_xult64_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXulteq64I32 { src1, src2 } => {
                write!(f, "if_xulteq64_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXugt64I32 { src1, src2 } => {
                write!(f, "if_xugt64_i32 {}, {src2}", reg_name(**src1))
            }
            Cond::IfXugteq64I32 { src1, src2 } => {
                write!(f, "if_xugteq64_i32 {}, {src2}", reg_name(**src1))
            }
        }
    }
}

/// Payload of `CallInfo` for call instructions
#[derive(Clone, Debug)]
pub struct PulleyCall {
    /// The external name that's being called, or the Cranelift-generated
    /// function that's being invoked.
    pub name: ExternalName,
    /// Arguments tracked in this call invocation which aren't assigned fixed
    /// registers. This tracks up to 4 registers and all remaining registers
    /// will be present and tracked in `CallInfo<T>` fields.
    pub args: SmallVec<[XReg; 4]>,
}

pub use super::super::lower::isle::generated_code::AddrO32;

impl Copy for AddrO32 {}

impl AddrO32 {
    /// Implementation of regalloc for this addressing mode.
    pub fn collect_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            AddrO32::Base { addr, offset: _ } => {
                collector.reg_use(addr);
            }
        }
    }
}

impl From<AddrO32> for pulley_interpreter::AddrO32 {
    fn from(addr: AddrO32) -> Self {
        match addr {
            AddrO32::Base { addr, offset } => Self {
                addr: addr.into(),
                offset,
            },
        }
    }
}

impl fmt::Display for AddrO32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddrO32::Base { addr, offset } => {
                let addr = reg_name(**addr);
                write!(f, "{addr}, {offset}")
            }
        }
    }
}

pub use super::super::lower::isle::generated_code::AddrZ;

impl Copy for AddrZ {}

impl AddrZ {
    /// Implementation of regalloc for this addressing mode.
    pub fn collect_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            AddrZ::Base { addr, offset: _ } => {
                collector.reg_use(addr);
            }
        }
    }
}

impl From<AddrZ> for pulley_interpreter::AddrZ {
    fn from(addr: AddrZ) -> Self {
        match addr {
            AddrZ::Base { addr, offset } => Self {
                addr: addr.into(),
                offset,
            },
        }
    }
}

impl fmt::Display for AddrZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddrZ::Base { addr, offset } => {
                let addr = reg_name(**addr);
                write!(f, "{addr}, {offset}")
            }
        }
    }
}

pub use super::super::lower::isle::generated_code::AddrG32;

impl Copy for AddrG32 {}

impl AddrG32 {
    /// Implementation of regalloc for this addressing mode.
    pub fn collect_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            AddrG32::RegisterBound {
                host_heap_base,
                host_heap_bound,
                wasm_addr,
                offset: _,
            } => {
                collector.reg_use(host_heap_base);
                collector.reg_use(host_heap_bound);
                collector.reg_use(wasm_addr);
            }
        }
    }
}

impl From<AddrG32> for pulley_interpreter::AddrG32 {
    fn from(addr: AddrG32) -> Self {
        match addr {
            AddrG32::RegisterBound {
                host_heap_base,
                host_heap_bound,
                wasm_addr,
                offset,
            } => Self {
                host_heap_base: host_heap_base.into(),
                host_heap_bound: host_heap_bound.into(),
                wasm_addr: wasm_addr.into(),
                offset,
            },
        }
    }
}

impl fmt::Display for AddrG32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddrG32::RegisterBound {
                host_heap_base,
                host_heap_bound,
                wasm_addr,
                offset,
            } => {
                let host_heap_base = reg_name(**host_heap_base);
                let host_heap_bound = reg_name(**host_heap_bound);
                let wasm_addr = reg_name(**wasm_addr);
                write!(
                    f,
                    "{host_heap_base}, {host_heap_bound}, {wasm_addr}, {offset}",
                )
            }
        }
    }
}

pub use super::super::lower::isle::generated_code::AddrG32Bne;

impl Copy for AddrG32Bne {}

impl AddrG32Bne {
    /// Implementation of regalloc for this addressing mode.
    pub fn collect_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            AddrG32Bne::BoundNe {
                host_heap_base,
                host_heap_bound_addr,
                host_heap_bound_offset: _,
                wasm_addr,
                offset: _,
            } => {
                collector.reg_use(host_heap_base);
                collector.reg_use(host_heap_bound_addr);
                collector.reg_use(wasm_addr);
            }
        }
    }
}

impl From<AddrG32Bne> for pulley_interpreter::AddrG32Bne {
    fn from(addr: AddrG32Bne) -> Self {
        match addr {
            AddrG32Bne::BoundNe {
                host_heap_base,
                host_heap_bound_addr,
                host_heap_bound_offset,
                wasm_addr,
                offset,
            } => Self {
                host_heap_base: host_heap_base.into(),
                host_heap_bound_addr: host_heap_bound_addr.into(),
                host_heap_bound_offset,
                wasm_addr: wasm_addr.into(),
                offset,
            },
        }
    }
}

impl fmt::Display for AddrG32Bne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddrG32Bne::BoundNe {
                host_heap_base,
                host_heap_bound_addr,
                host_heap_bound_offset,
                wasm_addr,
                offset,
            } => {
                let host_heap_base = reg_name(**host_heap_base);
                let host_heap_bound_addr = reg_name(**host_heap_bound_addr);
                let wasm_addr = reg_name(**wasm_addr);
                write!(
                    f,
                    "{host_heap_base}, \
                     *[{host_heap_bound_addr} + {host_heap_bound_offset}], \
                     {wasm_addr}, \
                     {offset}",
                )
            }
        }
    }
}
