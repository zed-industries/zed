//! Instruction operand sub-components (aka "parts"): definitions and printing.

use super::regs::{self};
use crate::ir::MemFlags;
use crate::ir::condcodes::{FloatCC, IntCC};
use crate::ir::types::*;
use crate::isa::x64::inst::Inst;
use crate::isa::x64::inst::regs::pretty_print_reg;
use crate::machinst::*;
use std::fmt;
use std::string::String;

/// An extension trait for converting `Writable{Xmm,Gpr}` to `Writable<Reg>`.
pub trait ToWritableReg {
    /// Convert `Writable{Xmm,Gpr}` to `Writable<Reg>`.
    fn to_writable_reg(&self) -> Writable<Reg>;
}

/// An extension trait for converting `Writable<Reg>` to `Writable{Xmm,Gpr}`.
pub trait FromWritableReg: Sized {
    /// Convert `Writable<Reg>` to `Writable{Xmm,Gpr}`.
    fn from_writable_reg(w: Writable<Reg>) -> Option<Self>;
}

/// A macro for defining a newtype of `Reg` that enforces some invariant about
/// the wrapped `Reg` (such as that it is of a particular register class).
macro_rules! newtype_of_reg {
    (
        $newtype_reg:ident,
        $newtype_writable_reg:ident,
        $newtype_option_writable_reg:ident,
        reg_mem: ($($newtype_reg_mem:ident $(aligned:$aligned:ident)?),*),
        reg_mem_imm: ($($newtype_reg_mem_imm:ident $(aligned:$aligned_imm:ident)?),*),
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
                if $check {
                    Some(Self($check_reg))
                } else {
                    None
                }
            }

            /// Like `Self::new(r).unwrap()` but with a better panic message on
            /// failure.
            pub fn unwrap_new($check_reg: Reg) -> Self {
                if $check {
                    Self($check_reg)
                } else {
                    panic!(
                        "cannot construct {} from register {:?} with register class {:?}",
                        stringify!($newtype_reg),
                        $check_reg,
                        $check_reg.class(),
                    )
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
        // nasty stuff like `*my_gpr.deref_mut() = some_xmm_reg`, breaking the
        // invariants that `Gpr` provides.
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

        /// Writable Gpr.
        pub type $newtype_writable_reg = Writable<$newtype_reg>;

        #[allow(dead_code, reason = "Used by some newtypes and not others")]
        /// Optional writable Gpr.
        pub type $newtype_option_writable_reg = Option<Writable<$newtype_reg>>;

        impl ToWritableReg for $newtype_writable_reg {
            fn to_writable_reg(&self) -> Writable<Reg> {
                Writable::from_reg(self.to_reg().to_reg())
            }
        }

        impl FromWritableReg for $newtype_writable_reg {
            fn from_writable_reg(w: Writable<Reg>) -> Option<Self> {
                Some(Writable::from_reg($newtype_reg::new(w.to_reg())?))
            }
        }

        $(
            /// A newtype wrapper around `RegMem` for general-purpose registers.
            #[derive(Clone, Debug)]
            pub struct $newtype_reg_mem(RegMem);

            impl From<$newtype_reg_mem> for RegMem {
                fn from(rm: $newtype_reg_mem) -> Self {
                    rm.0
                }
            }
            impl<'a> From<&'a $newtype_reg_mem> for &'a RegMem {
                fn from(rm: &'a $newtype_reg_mem) -> &'a RegMem {
                    &rm.0
                }
            }

            impl From<$newtype_reg> for $newtype_reg_mem {
                fn from(r: $newtype_reg) -> Self {
                    $newtype_reg_mem(RegMem::reg(r.into()))
                }
            }

            impl $newtype_reg_mem {
                /// Construct a `RegMem` newtype from the given `RegMem`, or return
                /// `None` if the `RegMem` is not a valid instance of this `RegMem`
                /// newtype.
                pub fn new(rm: RegMem) -> Option<Self> {
                    match rm {
                        RegMem::Mem { addr } => {
                            let mut _allow = true;
                            $(
                                if $aligned {
                                    _allow = addr.aligned();
                                }
                            )?
                            if _allow {
                                Some(Self(RegMem::Mem { addr }))
                            } else {
                                None
                            }
                        }
                        RegMem::Reg { reg } => Some($newtype_reg::new(reg)?.into()),
                    }
                }

                /// Like `Self::new(rm).unwrap()` but with better panic messages
                /// in case of failure.
                pub fn unwrap_new(rm: RegMem) -> Self {
                    match rm {
                        RegMem::Mem { addr } => {
                            $(
                                if $aligned && !addr.aligned() {
                                    panic!(
                                        "cannot create {} from an unaligned memory address: {addr:?}",
                                        stringify!($newtype_reg_mem),
                                    );
                                }
                            )?
                            Self(RegMem::Mem { addr })
                        }
                        RegMem::Reg { reg } => $newtype_reg::unwrap_new(reg).into(),
                    }
                }

                /// Convert this newtype into its underlying `RegMem`.
                pub fn to_reg_mem(self) -> RegMem {
                    self.0
                }

                #[allow(dead_code, reason = "Used by some newtypes and not others")]
                pub(crate) fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
                    self.0.get_operands(collector);
                }
            }
            impl PrettyPrint for $newtype_reg_mem {
                fn pretty_print(&self, size: u8) -> String {
                    self.0.pretty_print(size)
                }
            }
        )*

        $(
            /// A newtype wrapper around `RegMemImm`.
            #[derive(Clone, Debug)]
            pub struct $newtype_reg_mem_imm(RegMemImm);

            impl From<$newtype_reg_mem_imm> for RegMemImm {
                fn from(rmi: $newtype_reg_mem_imm) -> RegMemImm {
                    rmi.0
                }
            }
            impl<'a> From<&'a $newtype_reg_mem_imm> for &'a RegMemImm {
                fn from(rmi: &'a $newtype_reg_mem_imm) -> &'a RegMemImm {
                    &rmi.0
                }
            }

            impl From<$newtype_reg> for $newtype_reg_mem_imm {
                fn from(r: $newtype_reg) -> Self {
                    $newtype_reg_mem_imm(RegMemImm::reg(r.into()))
                }
            }

            impl $newtype_reg_mem_imm {
                /// Construct this newtype from the given `RegMemImm`, or return
                /// `None` if the `RegMemImm` is not a valid instance of this
                /// newtype.
                pub fn new(rmi: RegMemImm) -> Option<Self> {
                    match rmi {
                        RegMemImm::Imm { .. } => Some(Self(rmi)),
                        RegMemImm::Mem { addr } => {
                            let mut _allow = true;
                            $(
                                if $aligned_imm {
                                    _allow = addr.aligned();
                                }
                            )?
                            if _allow {
                                Some(Self(RegMemImm::Mem { addr }))
                            } else {
                                None
                            }
                        }
                        RegMemImm::Reg { reg } => Some($newtype_reg::new(reg)?.into()),
                    }
                }

                /// Like `Self::new(rmi).unwrap()` but with better panic
                /// messages in case of failure.
                pub fn unwrap_new(rmi: RegMemImm) -> Self {
                    match rmi {
                        RegMemImm::Imm { .. } => Self(rmi),
                        RegMemImm::Mem { addr } => {
                            $(
                                if $aligned_imm && !addr.aligned() {
                                    panic!(
                                        "cannot construct {} from unaligned memory address: {:?}",
                                        stringify!($newtype_reg_mem_imm),
                                        addr,
                                    );
                                }
                            )?
                            Self(RegMemImm::Mem { addr })

                        }
                        RegMemImm::Reg { reg } => $newtype_reg::unwrap_new(reg).into(),
                    }
                }

                /// Convert this newtype into its underlying `RegMemImm`.
                #[allow(dead_code, reason = "Used by some newtypes and not others")]
                pub fn to_reg_mem_imm(self) -> RegMemImm {
                    self.0
                }

                #[allow(dead_code, reason = "Used by some newtypes and not others")]
                pub(crate) fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
                    self.0.get_operands(collector);
                }
            }

            impl PrettyPrint for $newtype_reg_mem_imm {
                fn pretty_print(&self, size: u8) -> String {
                    self.0.pretty_print(size)
                }
            }
        )*
    };
}

// Define a newtype of `Reg` for general-purpose registers.
newtype_of_reg!(
    Gpr,
    WritableGpr,
    OptionWritableGpr,
    reg_mem: (GprMem),
    reg_mem_imm: (GprMemImm),
    |reg| reg.class() == RegClass::Int
);

#[expect(missing_docs, reason = "self-describing fields")]
impl Gpr {
    pub const RAX: Gpr = Gpr(regs::rax());
    pub const RBX: Gpr = Gpr(regs::rbx());
    pub const RCX: Gpr = Gpr(regs::rcx());
    pub const RDX: Gpr = Gpr(regs::rdx());
    pub const RSI: Gpr = Gpr(regs::rsi());
    pub const RDI: Gpr = Gpr(regs::rdi());
    pub const RSP: Gpr = Gpr(regs::rsp());
    pub const RBP: Gpr = Gpr(regs::rbp());
    pub const R8: Gpr = Gpr(regs::r8());
    pub const R9: Gpr = Gpr(regs::r9());
    pub const R10: Gpr = Gpr(regs::r10());
    pub const R11: Gpr = Gpr(regs::r11());
    pub const R12: Gpr = Gpr(regs::r12());
    pub const R13: Gpr = Gpr(regs::r13());
    pub const R14: Gpr = Gpr(regs::r14());
    pub const R15: Gpr = Gpr(regs::r15());
}

// Define a newtype of `Reg` for XMM registers.
newtype_of_reg!(
    Xmm,
    WritableXmm,
    OptionWritableXmm,
    reg_mem: (XmmMem, XmmMemAligned aligned:true),
    reg_mem_imm: (XmmMemImm, XmmMemAlignedImm aligned:true),
    |reg| reg.class() == RegClass::Float
);

// N.B.: `Amode` is defined in `inst.isle`. We add some convenience
// constructors here.

// Re-export the type from the ISLE generated code.
pub use crate::isa::x64::lower::isle::generated_code::Amode;

impl Amode {
    /// Create an immediate sign-extended and register addressing mode.
    pub fn imm_reg(simm32: i32, base: Reg) -> Self {
        debug_assert!(base.class() == RegClass::Int);
        Self::ImmReg {
            simm32,
            base,
            flags: MemFlags::trusted(),
        }
    }

    /// Create a sign-extended-32-to-64 with register and shift addressing mode.
    pub fn imm_reg_reg_shift(simm32: i32, base: Gpr, index: Gpr, shift: u8) -> Self {
        debug_assert!(base.class() == RegClass::Int);
        debug_assert!(index.class() == RegClass::Int);
        debug_assert!(shift <= 3);
        Self::ImmRegRegShift {
            simm32,
            base,
            index,
            shift,
            flags: MemFlags::trusted(),
        }
    }

    pub(crate) fn rip_relative(target: MachLabel) -> Self {
        Self::RipRelative { target }
    }

    /// Set the specified [MemFlags] to the [Amode].
    pub fn with_flags(&self, flags: MemFlags) -> Self {
        match self {
            &Self::ImmReg { simm32, base, .. } => Self::ImmReg {
                simm32,
                base,
                flags,
            },
            &Self::ImmRegRegShift {
                simm32,
                base,
                index,
                shift,
                ..
            } => Self::ImmRegRegShift {
                simm32,
                base,
                index,
                shift,
                flags,
            },
            _ => panic!("Amode {self:?} cannot take memflags"),
        }
    }

    /// Add the registers mentioned by `self` to `collector`.
    pub(crate) fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            Amode::ImmReg { base, .. } => {
                if *base != regs::rbp() && *base != regs::rsp() {
                    collector.reg_use(base);
                }
            }
            Amode::ImmRegRegShift { base, index, .. } => {
                debug_assert_ne!(base.to_reg(), regs::rbp());
                debug_assert_ne!(base.to_reg(), regs::rsp());
                collector.reg_use(base);
                debug_assert_ne!(index.to_reg(), regs::rbp());
                debug_assert_ne!(index.to_reg(), regs::rsp());
                collector.reg_use(index);
            }
            Amode::RipRelative { .. } => {
                // RIP isn't involved in regalloc.
            }
        }
    }

    /// Same as `get_operands`, but add the registers in the "late" phase.
    pub(crate) fn get_operands_late(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            Amode::ImmReg { base, .. } => {
                collector.reg_late_use(base);
            }
            Amode::ImmRegRegShift { base, index, .. } => {
                collector.reg_late_use(base);
                collector.reg_late_use(index);
            }
            Amode::RipRelative { .. } => {
                // RIP isn't involved in regalloc.
            }
        }
    }

    pub(crate) fn get_flags(&self) -> MemFlags {
        match self {
            Amode::ImmReg { flags, .. } | Amode::ImmRegRegShift { flags, .. } => *flags,
            Amode::RipRelative { .. } => MemFlags::trusted(),
        }
    }

    /// Offset the amode by a fixed offset.
    pub(crate) fn offset(&self, offset: i32) -> Self {
        let mut ret = self.clone();
        match &mut ret {
            &mut Amode::ImmReg { ref mut simm32, .. } => *simm32 += offset,
            &mut Amode::ImmRegRegShift { ref mut simm32, .. } => *simm32 += offset,
            _ => panic!("Cannot offset amode: {self:?}"),
        }
        ret
    }

    pub(crate) fn aligned(&self) -> bool {
        self.get_flags().aligned()
    }
}

impl PrettyPrint for Amode {
    fn pretty_print(&self, _size: u8) -> String {
        match self {
            Amode::ImmReg { simm32, base, .. } => {
                // Note: size is always 8; the address is 64 bits,
                // even if the addressed operand is smaller.
                format!("{}({})", *simm32, pretty_print_reg(*base, 8))
            }
            Amode::ImmRegRegShift {
                simm32,
                base,
                index,
                shift,
                ..
            } => format!(
                "{}({},{},{})",
                *simm32,
                pretty_print_reg(base.to_reg(), 8),
                pretty_print_reg(index.to_reg(), 8),
                1 << shift
            ),
            Amode::RipRelative { target } => format!("label{}(%rip)", target.as_u32()),
        }
    }
}

/// A Memory Address. These denote a 64-bit value only.
/// Used for usual addressing modes as well as addressing modes used during compilation, when the
/// moving SP offset is not known.
#[derive(Clone, Debug)]
pub enum SyntheticAmode {
    /// A real amode.
    Real(Amode),

    /// A (virtual) offset into the incoming argument area.
    IncomingArg {
        /// The downward offset from the start of the incoming argument area.
        offset: u32,
    },

    /// A (virtual) offset to the slot area of the function frame, which lies just above the
    /// outgoing arguments.
    SlotOffset {
        /// The offset into the slot area.
        simm32: i32,
    },

    /// A virtual offset to a constant that will be emitted in the constant section of the buffer.
    ConstantOffset(VCodeConstant),
}

impl SyntheticAmode {
    /// Create a real addressing mode.
    pub fn real(amode: Amode) -> Self {
        Self::Real(amode)
    }

    pub(crate) fn slot_offset(simm32: i32) -> Self {
        SyntheticAmode::SlotOffset { simm32 }
    }

    /// Add the registers mentioned by `self` to `collector`.
    pub(crate) fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            SyntheticAmode::Real(addr) => addr.get_operands(collector),
            SyntheticAmode::IncomingArg { .. } => {
                // Nothing to do; the base is known and isn't involved in regalloc.
            }
            SyntheticAmode::SlotOffset { .. } => {
                // Nothing to do; the base is SP and isn't involved in regalloc.
            }
            SyntheticAmode::ConstantOffset(_) => {}
        }
    }

    /// Same as `get_operands`, but add the register in the "late" phase.
    pub(crate) fn get_operands_late(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            SyntheticAmode::Real(addr) => addr.get_operands_late(collector),
            SyntheticAmode::IncomingArg { .. } => {
                // Nothing to do; the base is known and isn't involved in regalloc.
            }
            SyntheticAmode::SlotOffset { .. } => {
                // Nothing to do; the base is SP and isn't involved in regalloc.
            }
            SyntheticAmode::ConstantOffset(_) => {}
        }
    }

    pub(crate) fn finalize(&self, frame: &FrameLayout, buffer: &mut MachBuffer<Inst>) -> Amode {
        match self {
            SyntheticAmode::Real(addr) => addr.clone(),
            SyntheticAmode::IncomingArg { offset } => {
                // NOTE: this could be made relative to RSP by adding additional
                // offsets from the frame_layout.
                let args_max_fp_offset = frame.tail_args_size + frame.setup_area_size;
                Amode::imm_reg(
                    i32::try_from(args_max_fp_offset - offset).unwrap(),
                    regs::rbp(),
                )
            }
            SyntheticAmode::SlotOffset { simm32 } => {
                let off = *simm32 as i64 + i64::from(frame.outgoing_args_size);
                Amode::imm_reg(off.try_into().expect("invalid sp offset"), regs::rsp())
            }
            SyntheticAmode::ConstantOffset(c) => {
                Amode::rip_relative(buffer.get_label_for_constant(*c))
            }
        }
    }

    pub(crate) fn aligned(&self) -> bool {
        match self {
            SyntheticAmode::Real(addr) => addr.aligned(),
            &SyntheticAmode::IncomingArg { .. }
            | SyntheticAmode::SlotOffset { .. }
            | SyntheticAmode::ConstantOffset { .. } => true,
        }
    }
}

impl From<Amode> for SyntheticAmode {
    fn from(amode: Amode) -> SyntheticAmode {
        SyntheticAmode::Real(amode)
    }
}

impl From<VCodeConstant> for SyntheticAmode {
    fn from(c: VCodeConstant) -> SyntheticAmode {
        SyntheticAmode::ConstantOffset(c)
    }
}

impl PrettyPrint for SyntheticAmode {
    fn pretty_print(&self, _size: u8) -> String {
        match self {
            // See note in `Amode` regarding constant size of `8`.
            SyntheticAmode::Real(addr) => addr.pretty_print(8),
            &SyntheticAmode::IncomingArg { offset } => {
                format!("rbp(stack args max - {offset})")
            }
            SyntheticAmode::SlotOffset { simm32 } => {
                format!("rsp({} + virtual offset)", *simm32)
            }
            SyntheticAmode::ConstantOffset(c) => format!("const({})", c.as_u32()),
        }
    }
}

/// An operand which is either an integer Register, a value in Memory or an Immediate.  This can
/// denote an 8, 16, 32 or 64 bit value.  For the Immediate form, in the 8- and 16-bit case, only
/// the lower 8 or 16 bits of `simm32` is relevant.  In the 64-bit case, the value denoted by
/// `simm32` is its sign-extension out to 64 bits.
#[derive(Clone, Debug)]
pub enum RegMemImm {
    /// A register operand.
    Reg {
        /// The underlying register.
        reg: Reg,
    },
    /// A memory operand.
    Mem {
        /// The memory address.
        addr: SyntheticAmode,
    },
    /// An immediate operand.
    Imm {
        /// The immediate value.
        simm32: u32,
    },
}

impl RegMemImm {
    /// Create a register operand.
    pub fn reg(reg: Reg) -> Self {
        debug_assert!(reg.class() == RegClass::Int || reg.class() == RegClass::Float);
        Self::Reg { reg }
    }

    /// Create a memory operand.
    pub fn mem(addr: impl Into<SyntheticAmode>) -> Self {
        Self::Mem { addr: addr.into() }
    }

    /// Create an immediate operand.
    pub fn imm(simm32: u32) -> Self {
        Self::Imm { simm32 }
    }

    /// Add the regs mentioned by `self` to `collector`.
    pub(crate) fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            Self::Reg { reg } => collector.reg_use(reg),
            Self::Mem { addr } => addr.get_operands(collector),
            Self::Imm { .. } => {}
        }
    }
}

impl From<RegMem> for RegMemImm {
    fn from(rm: RegMem) -> RegMemImm {
        match rm {
            RegMem::Reg { reg } => RegMemImm::Reg { reg },
            RegMem::Mem { addr } => RegMemImm::Mem { addr },
        }
    }
}

impl From<Reg> for RegMemImm {
    fn from(reg: Reg) -> Self {
        RegMemImm::Reg { reg }
    }
}

impl PrettyPrint for RegMemImm {
    fn pretty_print(&self, size: u8) -> String {
        match self {
            Self::Reg { reg } => pretty_print_reg(*reg, size),
            Self::Mem { addr } => addr.pretty_print(size),
            Self::Imm { simm32 } => format!("${}", *simm32 as i32),
        }
    }
}

/// An operand which is either an integer Register or a value in Memory.  This can denote an 8, 16,
/// 32, 64, or 128 bit value.
#[derive(Clone, Debug)]
pub enum RegMem {
    /// A register operand.
    Reg {
        /// The underlying register.
        reg: Reg,
    },
    /// A memory operand.
    Mem {
        /// The memory address.
        addr: SyntheticAmode,
    },
}

impl RegMem {
    /// Create a register operand.
    pub fn reg(reg: Reg) -> Self {
        debug_assert!(reg.class() == RegClass::Int || reg.class() == RegClass::Float);
        Self::Reg { reg }
    }

    /// Create a memory operand.
    pub fn mem(addr: impl Into<SyntheticAmode>) -> Self {
        Self::Mem { addr: addr.into() }
    }
    /// Asserts that in register mode, the reg class is the one that's expected.
    pub(crate) fn assert_regclass_is(&self, expected_reg_class: RegClass) {
        if let Self::Reg { reg } = self {
            debug_assert_eq!(reg.class(), expected_reg_class);
        }
    }
    /// Add the regs mentioned by `self` to `collector`.
    pub(crate) fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        match self {
            RegMem::Reg { reg } => collector.reg_use(reg),
            RegMem::Mem { addr, .. } => addr.get_operands(collector),
        }
    }
}

impl From<Reg> for RegMem {
    fn from(reg: Reg) -> RegMem {
        RegMem::Reg { reg }
    }
}

impl From<Writable<Reg>> for RegMem {
    fn from(r: Writable<Reg>) -> Self {
        RegMem::reg(r.to_reg())
    }
}

impl PrettyPrint for RegMem {
    fn pretty_print(&self, size: u8) -> String {
        match self {
            RegMem::Reg { reg } => pretty_print_reg(*reg, size),
            RegMem::Mem { addr, .. } => addr.pretty_print(size),
        }
    }
}

#[derive(Debug)]
pub(crate) enum InstructionSet {
    SSE,
    SSE2,
    CMPXCHG16b,
    SSE3,
    SSSE3,
    SSE41,
    SSE42,
    Popcnt,
    Lzcnt,
    BMI1,
    BMI2,
    FMA,
    AVX,
    AVX2,
    AVX512BITALG,
    AVX512DQ,
    AVX512F,
    AVX512VBMI,
    AVX512VL,
}

/// This defines the ways a value can be extended: either signed- or zero-extension, or none for
/// types that are not extended. Contrast with [ExtMode], which defines the widths from and to which
/// values can be extended.
#[derive(Clone, PartialEq)]
pub enum ExtKind {
    /// No extension.
    None,
    /// Sign-extend.
    SignExtend,
    /// Zero-extend.
    ZeroExtend,
}

/// These indicate ways of extending (widening) a value, using the Intel
/// naming: B(yte) = u8, W(ord) = u16, L(ong)word = u32, Q(uad)word = u64
#[derive(Clone, PartialEq)]
pub enum ExtMode {
    /// Byte -> Longword.
    BL,
    /// Byte -> Quadword.
    BQ,
    /// Word -> Longword.
    WL,
    /// Word -> Quadword.
    WQ,
    /// Longword -> Quadword.
    LQ,
}

impl ExtMode {
    /// Calculate the `ExtMode` from passed bit lengths of the from/to types.
    pub(crate) fn new(from_bits: u16, to_bits: u16) -> Option<ExtMode> {
        match (from_bits, to_bits) {
            (1, 8) | (1, 16) | (1, 32) | (8, 16) | (8, 32) => Some(ExtMode::BL),
            (1, 64) | (8, 64) => Some(ExtMode::BQ),
            (16, 32) => Some(ExtMode::WL),
            (16, 64) => Some(ExtMode::WQ),
            (32, 64) => Some(ExtMode::LQ),
            _ => None,
        }
    }
}

impl fmt::Debug for ExtMode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            ExtMode::BL => "bl",
            ExtMode::BQ => "bq",
            ExtMode::WL => "wl",
            ExtMode::WQ => "wq",
            ExtMode::LQ => "lq",
        };
        write!(fmt, "{name}")
    }
}

impl fmt::Display for ExtMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// These indicate condition code tests.  Not all are represented since not all are useful in
/// compiler-generated code.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CC {
    ///  overflow
    O = 0,
    /// no overflow
    NO = 1,

    /// < unsigned
    B = 2,
    /// >= unsigned
    NB = 3,

    /// zero
    Z = 4,
    /// not-zero
    NZ = 5,

    /// <= unsigned
    BE = 6,
    /// > unsigned
    NBE = 7,

    /// negative
    S = 8,
    /// not-negative
    NS = 9,

    /// < signed
    L = 12,
    /// >= signed
    NL = 13,

    /// <= signed
    LE = 14,
    /// > signed
    NLE = 15,

    /// parity
    P = 10,

    /// not parity
    NP = 11,
}

impl CC {
    pub(crate) fn from_intcc(intcc: IntCC) -> Self {
        match intcc {
            IntCC::Equal => CC::Z,
            IntCC::NotEqual => CC::NZ,
            IntCC::SignedGreaterThanOrEqual => CC::NL,
            IntCC::SignedGreaterThan => CC::NLE,
            IntCC::SignedLessThanOrEqual => CC::LE,
            IntCC::SignedLessThan => CC::L,
            IntCC::UnsignedGreaterThanOrEqual => CC::NB,
            IntCC::UnsignedGreaterThan => CC::NBE,
            IntCC::UnsignedLessThanOrEqual => CC::BE,
            IntCC::UnsignedLessThan => CC::B,
        }
    }

    pub(crate) fn invert(&self) -> Self {
        match self {
            CC::O => CC::NO,
            CC::NO => CC::O,

            CC::B => CC::NB,
            CC::NB => CC::B,

            CC::Z => CC::NZ,
            CC::NZ => CC::Z,

            CC::BE => CC::NBE,
            CC::NBE => CC::BE,

            CC::S => CC::NS,
            CC::NS => CC::S,

            CC::L => CC::NL,
            CC::NL => CC::L,

            CC::LE => CC::NLE,
            CC::NLE => CC::LE,

            CC::P => CC::NP,
            CC::NP => CC::P,
        }
    }

    pub(crate) fn get_enc(self) -> u8 {
        self as u8
    }
}

impl fmt::Debug for CC {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            CC::O => "o",
            CC::NO => "no",
            CC::B => "b",
            CC::NB => "nb",
            CC::Z => "z",
            CC::NZ => "nz",
            CC::BE => "be",
            CC::NBE => "nbe",
            CC::S => "s",
            CC::NS => "ns",
            CC::L => "l",
            CC::NL => "nl",
            CC::LE => "le",
            CC::NLE => "nle",
            CC::P => "p",
            CC::NP => "np",
        };
        write!(fmt, "{name}")
    }
}

impl fmt::Display for CC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Encode the ways that floats can be compared. This is used in float comparisons such as `cmpps`,
/// e.g.; it is distinguished from other float comparisons (e.g. `ucomiss`) in that those use EFLAGS
/// whereas [FcmpImm] is used as an immediate.
#[derive(Clone, Copy)]
pub enum FcmpImm {
    /// Equal comparison.
    Equal = 0x00,
    /// Less than comparison.
    LessThan = 0x01,
    /// Less than or equal comparison.
    LessThanOrEqual = 0x02,
    /// Unordered.
    Unordered = 0x03,
    /// Not equal comparison.
    NotEqual = 0x04,
    /// Unordered of greater than or equal comparison.
    UnorderedOrGreaterThanOrEqual = 0x05,
    /// Unordered or greater than comparison.
    UnorderedOrGreaterThan = 0x06,
    /// Ordered.
    Ordered = 0x07,
}

impl FcmpImm {
    pub(crate) fn encode(self) -> u8 {
        self as u8
    }
}

impl From<FloatCC> for FcmpImm {
    fn from(cond: FloatCC) -> Self {
        match cond {
            FloatCC::Equal => FcmpImm::Equal,
            FloatCC::LessThan => FcmpImm::LessThan,
            FloatCC::LessThanOrEqual => FcmpImm::LessThanOrEqual,
            FloatCC::Unordered => FcmpImm::Unordered,
            FloatCC::NotEqual => FcmpImm::NotEqual,
            FloatCC::UnorderedOrGreaterThanOrEqual => FcmpImm::UnorderedOrGreaterThanOrEqual,
            FloatCC::UnorderedOrGreaterThan => FcmpImm::UnorderedOrGreaterThan,
            FloatCC::Ordered => FcmpImm::Ordered,
            _ => panic!("unable to create comparison predicate for {cond}"),
        }
    }
}

/// Encode the rounding modes used as part of the Rounding Control field.
/// Note, these rounding immediates only consider the rounding control field
/// (i.e. the rounding mode) which only take up the first two bits when encoded.
/// However the rounding immediate which this field helps make up, also includes
/// bits 3 and 4 which define the rounding select and precision mask respectively.
/// These two bits are not defined here and are implicitly set to zero when encoded.
#[derive(Clone, Copy)]
pub enum RoundImm {
    /// Round to nearest mode.
    RoundNearest = 0x00,
    /// Round down mode.
    RoundDown = 0x01,
    /// Round up mode.
    RoundUp = 0x02,
    /// Round to zero mode.
    RoundZero = 0x03,
}

impl RoundImm {
    pub(crate) fn encode(self) -> u8 {
        self as u8
    }
}

/// An operand's size in bits.
#[derive(Clone, Copy, PartialEq)]
pub enum OperandSize {
    /// 8-bit.
    Size8,
    /// 16-bit.
    Size16,
    /// 32-bit.
    Size32,
    /// 64-bit.
    Size64,
}

impl OperandSize {
    pub(crate) fn from_bytes(num_bytes: u32) -> Self {
        match num_bytes {
            1 => OperandSize::Size8,
            2 => OperandSize::Size16,
            4 => OperandSize::Size32,
            8 => OperandSize::Size64,
            _ => unreachable!("Invalid OperandSize: {}", num_bytes),
        }
    }

    // Computes the OperandSize for a given type.
    // For vectors, the OperandSize of the lanes is returned.
    pub(crate) fn from_ty(ty: Type) -> Self {
        Self::from_bytes(ty.lane_type().bytes())
    }

    // Check that the value of self is one of the allowed sizes.
    pub(crate) fn is_one_of(&self, sizes: &[Self]) -> bool {
        sizes.iter().any(|val| *self == *val)
    }

    pub(crate) fn to_bytes(&self) -> u8 {
        match self {
            Self::Size8 => 1,
            Self::Size16 => 2,
            Self::Size32 => 4,
            Self::Size64 => 8,
        }
    }

    pub(crate) fn to_bits(&self) -> u8 {
        self.to_bytes() * 8
    }
}
