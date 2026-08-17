//! Definitions for registers, operands, etc. Provides a thin
//! interface over the register allocator so that we can more easily
//! swap it out or shim it when necessary.

use alloc::{string::String, vec::Vec};
use core::{fmt::Debug, hash::Hash};
use regalloc2::{Operand, OperandConstraint, OperandKind, OperandPos, PReg, PRegSet, VReg};

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// The first 192 vregs (64 int, 64 float, 64 vec) are "pinned" to
/// physical registers. These must not be passed into the regalloc,
/// but they are used to represent physical registers in the same
/// `Reg` type post-regalloc.
const PINNED_VREGS: usize = 192;

/// Convert a `VReg` to its pinned `PReg`, if any.
pub fn pinned_vreg_to_preg(vreg: VReg) -> Option<PReg> {
    if vreg.vreg() < PINNED_VREGS {
        Some(PReg::from_index(vreg.vreg()))
    } else {
        None
    }
}

/// Convert a `PReg` to its pinned `VReg`.
pub const fn preg_to_pinned_vreg(preg: PReg) -> VReg {
    VReg::new(preg.index(), preg.class())
}

/// Give the first available vreg for generated code (i.e., after all
/// pinned vregs).
pub fn first_user_vreg_index() -> usize {
    // This is just the constant defined above, but we keep the
    // constant private and expose only this helper function with the
    // specific name in order to ensure other parts of the code don't
    // open-code and depend on the index-space scheme.
    PINNED_VREGS
}

/// A register named in an instruction. This register can be a virtual
/// register, a fixed physical register, or a named spillslot (after
/// regalloc). It does not have any constraints applied to it: those
/// can be added later in `MachInst::get_operands()` when the `Reg`s
/// are converted to `Operand`s.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Reg(u32);

const REG_SPILLSLOT_BIT: u32 = 0x8000_0000;
const REG_SPILLSLOT_MASK: u32 = !REG_SPILLSLOT_BIT;

impl Reg {
    /// Const constructor: create a new Reg from a regalloc2 VReg.
    pub const fn from_virtual_reg(vreg: regalloc2::VReg) -> Reg {
        Reg(vreg.bits() as u32)
    }

    /// Const constructor: create a new Reg from a regalloc2 PReg.
    pub const fn from_real_reg(preg: regalloc2::PReg) -> Reg {
        Reg(preg_to_pinned_vreg(preg).bits() as u32)
    }

    /// Get the physical register (`RealReg`), if this register is
    /// one.
    pub fn to_real_reg(self) -> Option<RealReg> {
        pinned_vreg_to_preg(self.0.into()).map(RealReg)
    }

    /// Get the virtual (non-physical) register, if this register is
    /// one.
    pub fn to_virtual_reg(self) -> Option<VirtualReg> {
        if self.to_spillslot().is_some() {
            None
        } else if pinned_vreg_to_preg(self.0.into()).is_none() {
            Some(VirtualReg(self.0.into()))
        } else {
            None
        }
    }

    /// Get the spillslot, if this register is one.
    pub fn to_spillslot(self) -> Option<SpillSlot> {
        if (self.0 & REG_SPILLSLOT_BIT) != 0 {
            Some(SpillSlot::new((self.0 & REG_SPILLSLOT_MASK) as usize))
        } else {
            None
        }
    }

    /// Get the class of this register.
    pub fn class(self) -> RegClass {
        assert!(!self.to_spillslot().is_some());
        VReg::from(self.0).class()
    }

    /// Is this a real (physical) reg?
    pub fn is_real(self) -> bool {
        self.to_real_reg().is_some()
    }

    /// Is this a virtual reg?
    pub fn is_virtual(self) -> bool {
        self.to_virtual_reg().is_some()
    }

    /// Is this a spillslot?
    pub fn is_spillslot(self) -> bool {
        self.to_spillslot().is_some()
    }
}

impl std::fmt::Debug for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if VReg::from(self.0) == VReg::invalid() {
            write!(f, "<invalid>")
        } else if let Some(spillslot) = self.to_spillslot() {
            write!(f, "{spillslot}")
        } else if let Some(rreg) = self.to_real_reg() {
            let preg: PReg = rreg.into();
            write!(f, "{preg}")
        } else if let Some(vreg) = self.to_virtual_reg() {
            let vreg: VReg = vreg.into();
            write!(f, "{vreg}")
        } else {
            unreachable!()
        }
    }
}

impl AsMut<Reg> for Reg {
    fn as_mut(&mut self) -> &mut Reg {
        self
    }
}

/// A real (physical) register. This corresponds to one of the target
/// ISA's named registers and can be used as an instruction operand.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct RealReg(PReg);

impl RealReg {
    /// Get the class of this register.
    pub fn class(self) -> RegClass {
        self.0.class()
    }

    /// The physical register number.
    pub fn hw_enc(self) -> u8 {
        self.0.hw_enc() as u8
    }
}

impl std::fmt::Debug for RealReg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Reg::from(*self).fmt(f)
    }
}

/// A virtual register. This can be allocated into a real (physical)
/// register of the appropriate register class, but which one is not
/// specified. Virtual registers are used when generating `MachInst`s,
/// before register allocation occurs, in order to allow us to name as
/// many register-carried values as necessary.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct VirtualReg(VReg);

impl VirtualReg {
    /// Get the class of this register.
    pub fn class(self) -> RegClass {
        self.0.class()
    }

    pub fn index(self) -> usize {
        self.0.vreg()
    }
}

impl std::fmt::Debug for VirtualReg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Reg::from(*self).fmt(f)
    }
}

/// A type wrapper that indicates a register type is writable. The
/// underlying register can be extracted, and the type wrapper can be
/// built using an arbitrary register. Hence, this type-level wrapper
/// is not strictly a guarantee. However, "casting" to a writable
/// register is an explicit operation for which we can
/// audit. Ordinarily, internal APIs in the compiler backend should
/// take a `Writable<Reg>` whenever the register is written, and the
/// usual, frictionless way to get one of these is to allocate a new
/// temporary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Writable<T> {
    reg: T,
}

impl<T> Writable<T> {
    /// Explicitly construct a `Writable<T>` from a `T`. As noted in
    /// the documentation for `Writable`, this is not hidden or
    /// disallowed from the outside; anyone can perform the "cast";
    /// but it is explicit so that we can audit the use sites.
    pub const fn from_reg(reg: T) -> Writable<T> {
        Writable { reg }
    }

    /// Get the underlying register, which can be read.
    pub fn to_reg(self) -> T {
        self.reg
    }

    /// Get a mutable borrow of the underlying register.
    pub fn reg_mut(&mut self) -> &mut T {
        &mut self.reg
    }

    /// Map the underlying register to another value or type.
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Writable<U> {
        Writable { reg: f(self.reg) }
    }
}

// Proxy on assembler trait to the underlying register type.
impl<R: cranelift_assembler_x64::AsReg> cranelift_assembler_x64::AsReg for Writable<R> {
    fn enc(&self) -> u8 {
        self.reg.enc()
    }

    fn to_string(&self, size: Option<cranelift_assembler_x64::gpr::Size>) -> String {
        self.reg.to_string(size)
    }

    fn new(_: u8) -> Self {
        panic!("disallow creation of new assembler registers")
    }
}

// Conversions between regalloc2 types (VReg, PReg) and our types
// (VirtualReg, RealReg, Reg).

impl std::convert::From<regalloc2::VReg> for Reg {
    fn from(vreg: regalloc2::VReg) -> Reg {
        Reg(vreg.bits() as u32)
    }
}

impl std::convert::From<regalloc2::VReg> for VirtualReg {
    fn from(vreg: regalloc2::VReg) -> VirtualReg {
        debug_assert!(pinned_vreg_to_preg(vreg).is_none());
        VirtualReg(vreg)
    }
}

impl std::convert::From<Reg> for regalloc2::VReg {
    /// Extract the underlying `regalloc2::VReg`. Note that physical
    /// registers also map to particular (special) VRegs, so this
    /// method can be used either on virtual or physical `Reg`s.
    fn from(reg: Reg) -> regalloc2::VReg {
        reg.0.into()
    }
}
impl std::convert::From<&Reg> for regalloc2::VReg {
    fn from(reg: &Reg) -> regalloc2::VReg {
        reg.0.into()
    }
}

impl std::convert::From<VirtualReg> for regalloc2::VReg {
    fn from(reg: VirtualReg) -> regalloc2::VReg {
        reg.0
    }
}

impl std::convert::From<RealReg> for regalloc2::VReg {
    fn from(reg: RealReg) -> regalloc2::VReg {
        // This representation is redundant: the class is implied in the vreg
        // index as well as being in the vreg class field.
        VReg::new(reg.0.index(), reg.0.class())
    }
}

impl std::convert::From<RealReg> for regalloc2::PReg {
    fn from(reg: RealReg) -> regalloc2::PReg {
        reg.0
    }
}

impl std::convert::From<regalloc2::PReg> for RealReg {
    fn from(preg: regalloc2::PReg) -> RealReg {
        RealReg(preg)
    }
}

impl std::convert::From<regalloc2::PReg> for Reg {
    fn from(preg: regalloc2::PReg) -> Reg {
        RealReg(preg).into()
    }
}

impl std::convert::From<RealReg> for Reg {
    fn from(reg: RealReg) -> Reg {
        Reg(VReg::from(reg).bits() as u32)
    }
}

impl std::convert::From<VirtualReg> for Reg {
    fn from(reg: VirtualReg) -> Reg {
        Reg(reg.0.bits() as u32)
    }
}

/// A spill slot.
pub type SpillSlot = regalloc2::SpillSlot;

impl std::convert::From<regalloc2::SpillSlot> for Reg {
    fn from(spillslot: regalloc2::SpillSlot) -> Reg {
        Reg(REG_SPILLSLOT_BIT | spillslot.index() as u32)
    }
}

/// A register class. Each register in the ISA has one class, and the
/// classes are disjoint. Most modern ISAs will have just two classes:
/// the integer/general-purpose registers (GPRs), and the float/vector
/// registers (typically used for both).
///
/// Note that unlike some other compiler backend/register allocator
/// designs, we do not allow for overlapping classes, i.e. registers
/// that belong to more than one class, because doing so makes the
/// allocation problem significantly more complex. Instead, when a
/// register can be addressed under different names for different
/// sizes (for example), the backend author should pick classes that
/// denote some fundamental allocation unit that encompasses the whole
/// register. For example, always allocate 128-bit vector registers
/// `v0`..`vN`, even though `f32` and `f64` values may use only the
/// low 32/64 bits of those registers and name them differently.
pub type RegClass = regalloc2::RegClass;

/// An OperandCollector is a wrapper around a Vec of Operands
/// (flattened array for a whole sequence of instructions) that
/// gathers operands from a single instruction and provides the range
/// in the flattened array.
#[derive(Debug)]
pub struct OperandCollector<'a, F: Fn(VReg) -> VReg> {
    operands: &'a mut Vec<Operand>,
    clobbers: PRegSet,

    /// The subset of physical registers that are allocatable.
    allocatable: PRegSet,

    renamer: F,
}

impl<'a, F: Fn(VReg) -> VReg> OperandCollector<'a, F> {
    /// Start gathering operands into one flattened operand array.
    pub fn new(operands: &'a mut Vec<Operand>, allocatable: PRegSet, renamer: F) -> Self {
        Self {
            operands,
            clobbers: PRegSet::default(),
            allocatable,
            renamer,
        }
    }

    /// Finish the operand collection and return the tuple giving the
    /// range of indices in the flattened operand array, and the
    /// clobber set.
    pub fn finish(self) -> (usize, PRegSet) {
        let end = self.operands.len();
        (end, self.clobbers)
    }
}

pub trait OperandVisitor {
    fn add_operand(
        &mut self,
        reg: &mut Reg,
        constraint: OperandConstraint,
        kind: OperandKind,
        pos: OperandPos,
    );

    fn debug_assert_is_allocatable_preg(&self, _reg: PReg, _expected: bool) {}

    /// Add a register clobber set. This is a set of registers that
    /// are written by the instruction, so must be reserved (not used)
    /// for the whole instruction, but are not used afterward.
    fn reg_clobbers(&mut self, _regs: PRegSet) {}
}

pub trait OperandVisitorImpl: OperandVisitor {
    /// Add a use of a fixed, nonallocatable physical register.
    fn reg_fixed_nonallocatable(&mut self, preg: PReg) {
        self.debug_assert_is_allocatable_preg(preg, false);
        // Since this operand does not participate in register allocation,
        // there's nothing to do here.
    }

    /// Add a register use, at the start of the instruction (`Before`
    /// position).
    fn reg_use(&mut self, reg: &mut impl AsMut<Reg>) {
        self.reg_maybe_fixed(reg.as_mut(), OperandKind::Use, OperandPos::Early);
    }

    /// Add a register use, at the end of the instruction (`After` position).
    fn reg_late_use(&mut self, reg: &mut impl AsMut<Reg>) {
        self.reg_maybe_fixed(reg.as_mut(), OperandKind::Use, OperandPos::Late);
    }

    /// Add a register def, at the end of the instruction (`After`
    /// position). Use only when this def will be written after all
    /// uses are read.
    fn reg_def(&mut self, reg: &mut Writable<impl AsMut<Reg>>) {
        self.reg_maybe_fixed(reg.reg.as_mut(), OperandKind::Def, OperandPos::Late);
    }

    /// Add a register "early def", which logically occurs at the
    /// beginning of the instruction, alongside all uses. Use this
    /// when the def may be written before all uses are read; the
    /// regalloc will ensure that it does not overwrite any uses.
    fn reg_early_def(&mut self, reg: &mut Writable<impl AsMut<Reg>>) {
        self.reg_maybe_fixed(reg.reg.as_mut(), OperandKind::Def, OperandPos::Early);
    }

    /// Add a register "fixed use", which ties a vreg to a particular
    /// RealReg at the end of the instruction.
    fn reg_fixed_late_use(&mut self, reg: &mut impl AsMut<Reg>, rreg: Reg) {
        self.reg_fixed(reg.as_mut(), rreg, OperandKind::Use, OperandPos::Late);
    }

    /// Add a register "fixed use", which ties a vreg to a particular
    /// RealReg at this point.
    fn reg_fixed_use(&mut self, reg: &mut impl AsMut<Reg>, rreg: Reg) {
        self.reg_fixed(reg.as_mut(), rreg, OperandKind::Use, OperandPos::Early);
    }

    /// Add a register "fixed def", which ties a vreg to a particular
    /// RealReg at this point.
    fn reg_fixed_def(&mut self, reg: &mut Writable<impl AsMut<Reg>>, rreg: Reg) {
        self.reg_fixed(reg.reg.as_mut(), rreg, OperandKind::Def, OperandPos::Late);
    }

    /// Add an operand tying a virtual register to a physical register.
    fn reg_fixed(&mut self, reg: &mut Reg, rreg: Reg, kind: OperandKind, pos: OperandPos) {
        debug_assert!(reg.is_virtual());
        let rreg = rreg.to_real_reg().expect("fixed reg is not a RealReg");
        self.debug_assert_is_allocatable_preg(rreg.into(), true);
        let constraint = OperandConstraint::FixedReg(rreg.into());
        self.add_operand(reg, constraint, kind, pos);
    }

    /// Add an operand which might already be a physical register.
    fn reg_maybe_fixed(&mut self, reg: &mut Reg, kind: OperandKind, pos: OperandPos) {
        if let Some(rreg) = reg.to_real_reg() {
            self.reg_fixed_nonallocatable(rreg.into());
        } else {
            debug_assert!(reg.is_virtual());
            self.add_operand(reg, OperandConstraint::Reg, kind, pos);
        }
    }

    /// Add a register def that reuses an earlier use-operand's
    /// allocation. The index of that earlier operand (relative to the
    /// current instruction's start of operands) must be known.
    fn reg_reuse_def(&mut self, reg: &mut Writable<impl AsMut<Reg>>, idx: usize) {
        let reg = reg.reg.as_mut();
        if let Some(rreg) = reg.to_real_reg() {
            // In some cases we see real register arguments to a reg_reuse_def
            // constraint. We assume the creator knows what they're doing
            // here, though we do also require that the real register be a
            // fixed-nonallocatable register.
            self.reg_fixed_nonallocatable(rreg.into());
        } else {
            debug_assert!(reg.is_virtual());
            // The operand we're reusing must not be fixed-nonallocatable, as
            // that would imply that the register has been allocated to a
            // virtual register.
            let constraint = OperandConstraint::Reuse(idx);
            self.add_operand(reg, constraint, OperandKind::Def, OperandPos::Late);
        }
    }

    /// Add a def that can be allocated to either a register or a
    /// spillslot, at the end of the instruction (`After`
    /// position). Use only when this def will be written after all
    /// uses are read.
    fn any_def(&mut self, reg: &mut Writable<impl AsMut<Reg>>) {
        self.add_operand(
            reg.reg.as_mut(),
            OperandConstraint::Any,
            OperandKind::Def,
            OperandPos::Late,
        );
    }

    /// Add a use that can be allocated to either a register or a
    /// spillslot, at the end of the instruction (`After` position).
    fn any_late_use(&mut self, reg: &mut impl AsMut<Reg>) {
        self.add_operand(
            reg.as_mut(),
            OperandConstraint::Any,
            OperandKind::Use,
            OperandPos::Late,
        );
    }
}

impl<T: OperandVisitor> OperandVisitorImpl for T {}

impl<'a, F: Fn(VReg) -> VReg> OperandVisitor for OperandCollector<'a, F> {
    fn add_operand(
        &mut self,
        reg: &mut Reg,
        constraint: OperandConstraint,
        kind: OperandKind,
        pos: OperandPos,
    ) {
        debug_assert!(!reg.is_spillslot());
        reg.0 = (self.renamer)(VReg::from(reg.0)).bits() as u32;
        self.operands
            .push(Operand::new(VReg::from(reg.0), constraint, kind, pos));
    }

    fn debug_assert_is_allocatable_preg(&self, reg: PReg, expected: bool) {
        debug_assert_eq!(
            self.allocatable.contains(reg),
            expected,
            "{reg:?} should{} be allocatable",
            if expected { "" } else { " not" }
        );
    }

    fn reg_clobbers(&mut self, regs: PRegSet) {
        self.clobbers.union_from(regs);
    }
}

impl<T: FnMut(&mut Reg, OperandConstraint, OperandKind, OperandPos)> OperandVisitor for T {
    fn add_operand(
        &mut self,
        reg: &mut Reg,
        constraint: OperandConstraint,
        kind: OperandKind,
        pos: OperandPos,
    ) {
        self(reg, constraint, kind, pos)
    }
}

/// Pretty-print part of a disassembly, with knowledge of
/// operand/instruction size, and optionally with regalloc
/// results. This can be used, for example, to print either `rax` or
/// `eax` for the register by those names on x86-64, depending on a
/// 64- or 32-bit context.
pub trait PrettyPrint {
    fn pretty_print(&self, size_bytes: u8) -> String;

    fn pretty_print_default(&self) -> String {
        self.pretty_print(0)
    }
}
