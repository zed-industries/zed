/*
 * The following license applies to this file, which derives many
 * details (register and constraint definitions, for example) from the
 * files `BacktrackingAllocator.h`, `BacktrackingAllocator.cpp`,
 * `LIR.h`, and possibly definitions in other related files in
 * `js/src/jit/`:
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![allow(dead_code)]
#![allow(clippy::all)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

// Even when trace logging is disabled, the trace macro has a significant
// performance cost so we disable it in release builds.
macro_rules! trace {
    ($($tt:tt)*) => {
        if cfg!(feature = "trace-log") {
            ::log::trace!($($tt)*);
        }
    };
}

macro_rules! trace_enabled {
    () => {
        cfg!(feature = "trace-log") && ::log::log_enabled!(::log::Level::Trace)
    };
}

use alloc::rc::Rc;
use allocator_api2::vec::Vec as Vec2;
use core::ops::Deref as _;
use core::{hash::BuildHasherDefault, iter::FromIterator};
use rustc_hash::FxHasher;
type FxHashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<FxHasher>>;
type FxHashSet<V> = hashbrown::HashSet<V, BuildHasherDefault<FxHasher>>;

pub(crate) mod cfg;
pub(crate) mod domtree;
pub(crate) mod fastalloc;
pub mod indexset;
pub(crate) mod ion;
pub(crate) mod moves;
pub(crate) mod postorder;
pub mod ssa;

#[macro_use]
mod index;

pub use self::ion::data_structures::Ctx;
use alloc::vec::Vec;
pub use index::{Block, Inst, InstRange};

pub mod checker;

#[cfg(feature = "fuzzing")]
pub mod fuzzing;

#[cfg(feature = "enable-serde")]
pub mod serialize;

#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};

/// Register classes.
///
/// Every value has a "register class", which is like a type at the
/// register-allocator level. Every register must belong to only one
/// class; i.e., they are disjoint.
///
/// For tight bit-packing throughout our data structures, we support
/// only three classes, "int", "float" and "vector". Usually two will
/// be enough on modern machines, as they have one class of general-purpose
/// integer registers of machine width (e.g. 64 bits), and another
/// class of float/vector registers used both for FP and for vector
/// operations. Additionally for machines with totally separate vector
/// registers a third class is provided.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum RegClass {
    Int = 0,
    Float = 1,
    Vector = 2,
}

/// A physical register. Contains a physical register number and a class.
///
/// The `hw_enc` field contains the physical register number and is in
/// a logically separate index space per class; in other words, Int
/// register 0 is different than Float register 0.
///
/// Because of bit-packed encodings throughout the implementation,
/// `hw_enc` must fit in 6 bits, i.e., at most 64 registers per class.
///
/// The value returned by `index()`, in contrast, is in a single index
/// space shared by all classes, in order to enable uniform reasoning
/// about physical registers. This is done by putting the class bit at
/// the MSB, or equivalently, declaring that indices 0..=63 are the 64
/// integer registers and indices 64..=127 are the 64 float registers.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct PReg {
    bits: u8,
}

impl PReg {
    pub const MAX_BITS: usize = 6;
    pub const MAX: usize = (1 << Self::MAX_BITS) - 1;
    pub const NUM_INDEX: usize = 1 << (Self::MAX_BITS + 2); // including RegClass bits

    /// Create a new PReg. The `hw_enc` range is 6 bits.
    #[inline(always)]
    pub const fn new(hw_enc: usize, class: RegClass) -> Self {
        debug_assert!(hw_enc <= PReg::MAX);
        PReg {
            bits: ((class as u8) << Self::MAX_BITS) | (hw_enc as u8),
        }
    }

    /// The physical register number, as encoded by the ISA for the particular register class.
    #[inline(always)]
    pub const fn hw_enc(self) -> usize {
        self.bits as usize & Self::MAX
    }

    /// The register class.
    #[inline(always)]
    pub const fn class(self) -> RegClass {
        match (self.bits >> Self::MAX_BITS) & 0b11 {
            0 => RegClass::Int,
            1 => RegClass::Float,
            2 => RegClass::Vector,
            _ => unreachable!(),
        }
    }

    /// Get an index into the (not necessarily contiguous) index space of
    /// all physical registers. Allows one to maintain an array of data for
    /// all PRegs and index it efficiently.
    #[inline(always)]
    pub const fn index(self) -> usize {
        self.bits as usize
    }

    /// Construct a PReg from the value returned from `.index()`.
    #[inline(always)]
    pub const fn from_index(index: usize) -> Self {
        PReg {
            bits: (index & (Self::NUM_INDEX - 1)) as u8,
        }
    }

    /// Return the "invalid PReg", which can be used to initialize
    /// data structures.
    #[inline(always)]
    pub const fn invalid() -> Self {
        PReg::new(Self::MAX, RegClass::Int)
    }
}

impl Default for PReg {
    fn default() -> Self {
        Self::invalid()
    }
}

impl core::fmt::Debug for PReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "PReg(hw = {}, class = {:?}, index = {})",
            self.hw_enc(),
            self.class(),
            self.index()
        )
    }
}

impl core::fmt::Display for PReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let class = match self.class() {
            RegClass::Int => "i",
            RegClass::Float => "f",
            RegClass::Vector => "v",
        };
        write!(f, "p{}{}", self.hw_enc(), class)
    }
}

/// A type for internal bit arrays.
type Bits = u64;

/// A physical register set. Used to represent clobbers
/// efficiently.
///
/// The set is `Copy` and is guaranteed to have constant, and small,
/// size, as it is based on a bitset internally.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct PRegSet {
    bits: [Bits; Self::LEN],
}

impl PRegSet {
    /// The number of bits per element in the internal bit array.
    const BITS: usize = core::mem::size_of::<Bits>() * 8;

    /// Length of the internal bit array.
    const LEN: usize = (PReg::NUM_INDEX + Self::BITS - 1) / Self::BITS;

    /// Create an empty set.
    pub const fn empty() -> Self {
        Self {
            bits: [0; Self::LEN],
        }
    }

    /// Splits the given register index into parts to access the internal bit array.
    const fn split_index(reg: PReg) -> (usize, usize) {
        let index = reg.index();
        (index >> Self::BITS.ilog2(), index & (Self::BITS - 1))
    }

    /// Returns whether the given register is part of the set.
    pub fn contains(&self, reg: PReg) -> bool {
        let (index, bit) = Self::split_index(reg);
        self.bits[index] & (1 << bit) != 0
    }

    /// Add a physical register (PReg) to the set, returning the new value.
    pub const fn with(self, reg: PReg) -> Self {
        let (index, bit) = Self::split_index(reg);
        let mut out = self;
        out.bits[index] |= 1 << bit;
        out
    }

    /// Add a physical register (PReg) to the set.
    pub fn add(&mut self, reg: PReg) {
        let (index, bit) = Self::split_index(reg);
        self.bits[index] |= 1 << bit;
    }

    /// Remove a physical register (PReg) from the set.
    pub fn remove(&mut self, reg: PReg) {
        let (index, bit) = Self::split_index(reg);
        self.bits[index] &= !(1 << bit);
    }

    /// Add all of the registers in one set to this one, mutating in
    /// place.
    pub fn union_from(&mut self, other: PRegSet) {
        for i in 0..self.bits.len() {
            self.bits[i] |= other.bits[i];
        }
    }

    pub fn intersect_from(&mut self, other: PRegSet) {
        for i in 0..self.bits.len() {
            self.bits[i] &= other.bits[i];
        }
    }

    pub fn invert(&self) -> PRegSet {
        let mut set = self.bits;
        for i in 0..self.bits.len() {
            set[i] = !self.bits[i];
        }
        PRegSet { bits: set }
    }

    pub fn is_empty(&self, regclass: RegClass) -> bool {
        self.bits[regclass as usize] == 0
    }
}

impl core::ops::BitAnd<PRegSet> for PRegSet {
    type Output = PRegSet;

    fn bitand(self, rhs: PRegSet) -> Self::Output {
        let mut out = self;
        out.intersect_from(rhs);
        out
    }
}

impl core::ops::BitOr<PRegSet> for PRegSet {
    type Output = PRegSet;

    fn bitor(self, rhs: PRegSet) -> Self::Output {
        let mut out = self;
        out.union_from(rhs);
        out
    }
}

impl IntoIterator for PRegSet {
    type Item = PReg;
    type IntoIter = PRegSetIter;
    fn into_iter(self) -> PRegSetIter {
        PRegSetIter {
            bits: self.bits,
            cur: 0,
        }
    }
}

pub struct PRegSetIter {
    bits: [Bits; PRegSet::LEN],
    cur: usize,
}

impl Iterator for PRegSetIter {
    type Item = PReg;
    fn next(&mut self) -> Option<PReg> {
        loop {
            let bits = self.bits.get_mut(self.cur)?;
            if *bits != 0 {
                let bit = bits.trailing_zeros();
                *bits &= !(1 << bit);
                let index = bit as usize + self.cur * PRegSet::BITS;
                return Some(PReg::from_index(index));
            }
            self.cur += 1;
        }
    }
}

impl From<&MachineEnv> for PRegSet {
    fn from(env: &MachineEnv) -> Self {
        let mut res = Self::default();

        for class in env.preferred_regs_by_class.iter() {
            for preg in class {
                res.add(*preg)
            }
        }

        for class in env.non_preferred_regs_by_class.iter() {
            for preg in class {
                res.add(*preg)
            }
        }

        res
    }
}

impl FromIterator<PReg> for PRegSet {
    fn from_iter<T: IntoIterator<Item = PReg>>(iter: T) -> Self {
        let mut set = Self::default();
        for preg in iter {
            set.add(preg);
        }
        set
    }
}

impl core::fmt::Display for PRegSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{")?;
        for preg in self.into_iter() {
            write!(f, "{preg}, ")?;
        }
        write!(f, "}}")
    }
}

/// A virtual register. Contains a virtual register number and a
/// class.
///
/// A virtual register ("vreg") corresponds to an SSA value. All
/// dataflow in the input program is specified via flow through a
/// virtual register; even uses of specially-constrained locations,
/// such as fixed physical registers, are done by using vregs, because
/// we need the vreg's live range in order to track the use of that
/// location.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct VReg {
    bits: u32,
}

impl VReg {
    pub const MAX_BITS: usize = 21;
    pub const MAX: usize = (1 << Self::MAX_BITS) - 1;

    #[inline(always)]
    pub const fn new(virt_reg: usize, class: RegClass) -> Self {
        debug_assert!(virt_reg <= VReg::MAX);
        VReg {
            bits: ((virt_reg as u32) << 2) | (class as u8 as u32),
        }
    }

    #[inline(always)]
    pub const fn vreg(self) -> usize {
        let vreg = (self.bits >> 2) as usize;
        vreg
    }

    #[inline(always)]
    pub const fn class(self) -> RegClass {
        match self.bits & 0b11 {
            0 => RegClass::Int,
            1 => RegClass::Float,
            2 => RegClass::Vector,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub const fn invalid() -> Self {
        VReg::new(Self::MAX, RegClass::Int)
    }

    #[inline(always)]
    pub const fn bits(self) -> usize {
        self.bits as usize
    }
}

impl From<u32> for VReg {
    fn from(value: u32) -> Self {
        Self { bits: value }
    }
}

impl core::fmt::Debug for VReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "VReg(vreg = {}, class = {:?})",
            self.vreg(),
            self.class()
        )
    }
}

impl core::fmt::Display for VReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "v{}", self.vreg())
    }
}

/// A spillslot is a space in the stackframe used by the allocator to
/// temporarily store a value.
///
/// The allocator is responsible for allocating indices in this space,
/// and will specify how many spillslots have been used when the
/// allocation is completed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct SpillSlot {
    bits: u32,
}

impl SpillSlot {
    /// The maximum spillslot index.
    pub const MAX: usize = (1 << 24) - 1;

    /// Create a new SpillSlot.
    #[inline(always)]
    pub fn new(slot: usize) -> Self {
        debug_assert!(slot <= Self::MAX);
        SpillSlot { bits: slot as u32 }
    }

    /// Get the spillslot index for this spillslot.
    #[inline(always)]
    pub fn index(self) -> usize {
        (self.bits & 0x00ffffff) as usize
    }

    /// Get the spillslot `offset` slots away.
    #[inline(always)]
    pub fn plus(self, offset: usize) -> Self {
        SpillSlot::new(self.index() + offset)
    }

    /// Get the invalid spillslot, used for initializing data structures.
    #[inline(always)]
    pub fn invalid() -> Self {
        SpillSlot { bits: 0xffff_ffff }
    }

    /// Is this the invalid spillslot?
    #[inline(always)]
    pub fn is_invalid(self) -> bool {
        self == Self::invalid()
    }

    /// Is this a valid spillslot (not `SpillSlot::invalid()`)?
    #[inline(always)]
    pub fn is_valid(self) -> bool {
        self != Self::invalid()
    }
}

impl core::fmt::Display for SpillSlot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "stack{}", self.index())
    }
}

/// An `OperandConstraint` specifies where a vreg's value must be
/// placed at a particular reference to that vreg via an
/// `Operand`. The constraint may be loose -- "any register of a given
/// class", for example -- or very specific, such as "this particular
/// physical register". The allocator's result will always satisfy all
/// given constraints; however, if the input has a combination of
/// constraints that are impossible to satisfy, then allocation may
/// fail or the allocator may panic (providing impossible constraints
/// is usually a programming error in the client, rather than a
/// function of bad input).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum OperandConstraint {
    /// Any location is fine (register or stack slot).
    Any,
    /// Operand must be in a register. Register is read-only for Uses.
    Reg,
    /// Operand must be in a fixed register.
    FixedReg(PReg),
    /// On defs only: reuse a use's register.
    Reuse(usize),
}

impl core::fmt::Display for OperandConstraint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::Reg => write!(f, "reg"),
            Self::FixedReg(preg) => write!(f, "fixed({})", preg),
            Self::Reuse(idx) => write!(f, "reuse({})", idx),
        }
    }
}

/// The "kind" of the operand: whether it reads a vreg (Use) or writes
/// a vreg (Def).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum OperandKind {
    Def = 0,
    Use = 1,
}

/// The "position" of the operand: where it has its read/write
/// effects. These are positions "in" the instruction, and "early" and
/// "late" are relative to the instruction's main effect or
/// computation. In other words, the allocator assumes that the
/// instruction (i) performs all reads and writes of "early" operands,
/// (ii) does its work, and (iii) performs all reads and writes of its
/// "late" operands.
///
/// A "write" (def) at "early" or a "read" (use) at "late" may be
/// slightly nonsensical, given the above, if the read is necessary
/// for the computation or the write is a result of it. A way to think
/// of it is that the value (even if a result of execution) *could*
/// have been read or written at the given location without causing
/// any register-usage conflicts. In other words, these write-early or
/// use-late operands ensure that the particular allocations are valid
/// for longer than usual and that a register is not reused between
/// the use (normally complete at "Early") and the def (normally
/// starting at "Late"). See `Operand` for more.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum OperandPos {
    Early = 0,
    Late = 1,
}

/// An `Operand` encodes everything about a mention of a register in
/// an instruction: virtual register number, and any constraint that
/// applies to the register at this program point.
///
/// An Operand may be a use or def (this corresponds to `LUse` and
/// `LAllocation` in Ion).
///
/// Generally, regalloc2 considers operands to have their effects at
/// one of two points that exist in an instruction: "Early" or
/// "Late". All operands at a given program-point are assigned
/// non-conflicting locations based on their constraints. Each operand
/// has a "kind", one of use/def/mod, corresponding to
/// read/write/read-write, respectively.
///
/// Usually, an instruction's inputs will be "early uses" and outputs
/// will be "late defs", though there are valid use-cases for other
/// combinations too. For example, a single "instruction" seen by the
/// regalloc that lowers into multiple machine instructions and reads
/// some of its inputs after it starts to write outputs must either
/// make those input(s) "late uses" or those output(s) "early defs" so
/// that the conflict (overlap) is properly accounted for. See
/// comments on the constructors below for more.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Operand {
    /// Bit-pack into 32 bits.
    ///
    /// constraint:7 kind:1 pos:1 class:2 vreg:21
    ///
    /// where `constraint` is an `OperandConstraint`, `kind` is an
    /// `OperandKind`, `pos` is an `OperandPos`, `class` is a
    /// `RegClass`, and `vreg` is a vreg index.
    ///
    /// The constraints are encoded as follows:
    /// - 1xxxxxx => FixedReg(preg)
    /// - 01xxxxx => Reuse(index)
    /// - 0000000 => Any
    /// - 0000001 => Reg
    /// - 0000010 => Stack
    /// - _ => Unused for now
    bits: u32,
}

impl Operand {
    /// Construct a new operand.
    #[inline(always)]
    pub fn new(
        vreg: VReg,
        constraint: OperandConstraint,
        kind: OperandKind,
        pos: OperandPos,
    ) -> Self {
        let constraint_field = match constraint {
            OperandConstraint::Any => 0,
            OperandConstraint::Reg => 1,
            OperandConstraint::FixedReg(preg) => {
                debug_assert_eq!(preg.class(), vreg.class());
                0b1000000 | preg.hw_enc() as u32
            }
            OperandConstraint::Reuse(which) => {
                debug_assert!(which <= 31);
                0b0100000 | which as u32
            }
        };
        let class_field = vreg.class() as u8 as u32;
        let pos_field = pos as u8 as u32;
        let kind_field = kind as u8 as u32;
        Operand {
            bits: vreg.vreg() as u32
                | (class_field << 21)
                | (pos_field << 23)
                | (kind_field << 24)
                | (constraint_field << 25),
        }
    }

    /// Create an `Operand` that designates a use of a VReg that must
    /// be in a register, and that is used at the "before" point,
    /// i.e., can be overwritten by a result.
    #[inline(always)]
    pub fn reg_use(vreg: VReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::Reg,
            OperandKind::Use,
            OperandPos::Early,
        )
    }

    /// Create an `Operand` that designates a use of a VReg that must
    /// be in a register, and that is used up until the "after" point,
    /// i.e., must not conflict with any results.
    #[inline(always)]
    pub fn reg_use_at_end(vreg: VReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::Reg,
            OperandKind::Use,
            OperandPos::Late,
        )
    }

    /// Create an `Operand` that designates a definition of a VReg
    /// that must be in a register, and that occurs at the "after"
    /// point, i.e. may reuse a register that carried a use into this
    /// instruction.
    #[inline(always)]
    pub fn reg_def(vreg: VReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::Reg,
            OperandKind::Def,
            OperandPos::Late,
        )
    }

    /// Create an `Operand` that designates a definition of a VReg
    /// that must be in a register, and that occurs early at the
    /// "before" point, i.e., must not conflict with any input to the
    /// instruction.
    ///
    /// Note that the register allocator will ensure that such an
    /// early-def operand is live throughout the instruction, i.e., also
    /// at the after-point. Hence it will also avoid conflicts with all
    /// outputs to the instruction. As such, early defs are appropriate
    /// for use as "temporary registers" that an instruction can use
    /// throughout its execution separately from the inputs and outputs.
    #[inline(always)]
    pub fn reg_def_at_start(vreg: VReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::Reg,
            OperandKind::Def,
            OperandPos::Early,
        )
    }

    /// Create an `Operand` that designates a def (and use) of a
    /// temporary *within* the instruction. This register is assumed
    /// to be written by the instruction, and will not conflict with
    /// any input or output, but should not be used after the
    /// instruction completes.
    ///
    /// Note that within a single instruction, the dedicated scratch
    /// register (as specified in the `MachineEnv`) is also always
    /// available for use. The register allocator may use the register
    /// *between* instructions in order to implement certain sequences
    /// of moves, but will never hold a value live in the scratch
    /// register across an instruction.
    #[inline(always)]
    pub fn reg_temp(vreg: VReg) -> Self {
        // For now a temp is equivalent to a def-at-start operand,
        // which gives the desired semantics but does not enforce the
        // "not reused later" constraint.
        Operand::new(
            vreg,
            OperandConstraint::Reg,
            OperandKind::Def,
            OperandPos::Early,
        )
    }

    /// Create an `Operand` that designates a def of a vreg that must
    /// reuse the register assigned to an input to the
    /// instruction. The input is identified by `idx` (is the `idx`th
    /// `Operand` for the instruction) and must be constraint to a
    /// register, i.e., be the result of `Operand::reg_use(vreg)`.
    #[inline(always)]
    pub fn reg_reuse_def(vreg: VReg, idx: usize) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::Reuse(idx),
            OperandKind::Def,
            OperandPos::Late,
        )
    }

    /// Create an `Operand` that designates a use of a vreg and
    /// ensures that it is placed in the given, fixed PReg at the
    /// use. It is guaranteed that the `Allocation` resulting for this
    /// operand will be `preg`.
    #[inline(always)]
    pub fn reg_fixed_use(vreg: VReg, preg: PReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::FixedReg(preg),
            OperandKind::Use,
            OperandPos::Early,
        )
    }

    /// Create an `Operand` that designates a def of a vreg and
    /// ensures that it is placed in the given, fixed PReg at the
    /// def. It is guaranteed that the `Allocation` resulting for this
    /// operand will be `preg`.
    #[inline(always)]
    pub fn reg_fixed_def(vreg: VReg, preg: PReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::FixedReg(preg),
            OperandKind::Def,
            OperandPos::Late,
        )
    }

    /// Same as `reg_fixed_use` but at `OperandPos::Late`.
    #[inline(always)]
    pub fn reg_fixed_use_at_end(vreg: VReg, preg: PReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::FixedReg(preg),
            OperandKind::Use,
            OperandPos::Late,
        )
    }

    /// Same as `reg_fixed_def` but at `OperandPos::Early`.
    #[inline(always)]
    pub fn reg_fixed_def_at_start(vreg: VReg, preg: PReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::FixedReg(preg),
            OperandKind::Def,
            OperandPos::Early,
        )
    }

    /// Create an `Operand` that designates a use of a vreg and places
    /// no constraints on its location (i.e., it can be allocated into
    /// either a register or on the stack).
    #[inline(always)]
    pub fn any_use(vreg: VReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::Any,
            OperandKind::Use,
            OperandPos::Early,
        )
    }

    /// Create an `Operand` that designates a def of a vreg and places
    /// no constraints on its location (i.e., it can be allocated into
    /// either a register or on the stack).
    #[inline(always)]
    pub fn any_def(vreg: VReg) -> Self {
        Operand::new(
            vreg,
            OperandConstraint::Any,
            OperandKind::Def,
            OperandPos::Late,
        )
    }

    /// Create an `Operand` that always results in an assignment to the
    /// given fixed `preg`, *without* tracking liveranges in that
    /// `preg`. Must only be used for non-allocatable registers.
    #[inline(always)]
    pub fn fixed_nonallocatable(preg: PReg) -> Self {
        Operand::new(
            VReg::new(VReg::MAX, preg.class()),
            OperandConstraint::FixedReg(preg),
            OperandKind::Use,
            OperandPos::Early,
        )
    }

    /// Get the virtual register designated by an operand. Every
    /// operand must name some virtual register, even if it constrains
    /// the operand to a fixed physical register as well; the vregs
    /// are used to track dataflow.
    #[inline(always)]
    pub fn vreg(self) -> VReg {
        let vreg_idx = ((self.bits as usize) & VReg::MAX) as usize;
        VReg::new(vreg_idx, self.class())
    }

    /// Get the register class used by this operand.
    #[inline(always)]
    pub fn class(self) -> RegClass {
        let class_field = (self.bits >> 21) & 3;
        match class_field {
            0 => RegClass::Int,
            1 => RegClass::Float,
            2 => RegClass::Vector,
            _ => unreachable!(),
        }
    }

    /// Get the "kind" of this operand: a definition (write) or a use
    /// (read).
    #[inline(always)]
    pub fn kind(self) -> OperandKind {
        let kind_field = (self.bits >> 24) & 1;
        match kind_field {
            0 => OperandKind::Def,
            1 => OperandKind::Use,
            _ => unreachable!(),
        }
    }

    /// Get the "position" of this operand, i.e., where its read
    /// and/or write occurs: either before the instruction executes,
    /// or after it does. Ordinarily, uses occur at "before" and defs
    /// at "after", though there are cases where this is not true.
    #[inline(always)]
    pub fn pos(self) -> OperandPos {
        let pos_field = (self.bits >> 23) & 1;
        match pos_field {
            0 => OperandPos::Early,
            1 => OperandPos::Late,
            _ => unreachable!(),
        }
    }

    /// Get the "constraint" of this operand, i.e., what requirements
    /// its allocation must fulfill.
    #[inline(always)]
    pub fn constraint(self) -> OperandConstraint {
        let constraint_field = ((self.bits >> 25) as usize) & 127;
        if constraint_field & 0b1000000 != 0 {
            OperandConstraint::FixedReg(PReg::new(constraint_field & 0b0111111, self.class()))
        } else if constraint_field & 0b0100000 != 0 {
            OperandConstraint::Reuse(constraint_field & 0b0011111)
        } else {
            match constraint_field {
                0 => OperandConstraint::Any,
                1 => OperandConstraint::Reg,
                _ => unreachable!(),
            }
        }
    }

    /// If this operand is for a fixed non-allocatable register (see
    /// [`Operand::fixed_nonallocatable`]), then returns the physical register that it will
    /// be assigned to.
    #[inline(always)]
    pub fn as_fixed_nonallocatable(self) -> Option<PReg> {
        match self.constraint() {
            OperandConstraint::FixedReg(preg) if self.vreg().vreg() == VReg::MAX => Some(preg),
            _ => None,
        }
    }

    /// Get the raw 32-bit encoding of this operand's fields.
    #[inline(always)]
    pub fn bits(self) -> u32 {
        self.bits
    }

    /// Construct an `Operand` from the raw 32-bit encoding returned
    /// from `bits()`.
    #[inline(always)]
    pub fn from_bits(bits: u32) -> Self {
        debug_assert!(bits >> 29 <= 4);
        Operand { bits }
    }
}

impl core::fmt::Debug for Operand {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for Operand {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(preg) = self.as_fixed_nonallocatable() {
            return write!(f, "Fixed: {preg}");
        }
        match (self.kind(), self.pos()) {
            (OperandKind::Def, OperandPos::Late) | (OperandKind::Use, OperandPos::Early) => {
                write!(f, "{:?}", self.kind())?;
            }
            _ => {
                write!(f, "{:?}@{:?}", self.kind(), self.pos())?;
            }
        }
        write!(
            f,
            ": {}{} {}",
            self.vreg(),
            match self.class() {
                RegClass::Int => "i",
                RegClass::Float => "f",
                RegClass::Vector => "v",
            },
            self.constraint()
        )
    }
}

/// An Allocation represents the end result of regalloc for an
/// Operand.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Allocation {
    /// Bit-pack in 32 bits.
    ///
    /// kind:3 unused:1 index:28
    bits: u32,
}

impl core::fmt::Debug for Allocation {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for Allocation {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.kind() {
            AllocationKind::None => write!(f, "none"),
            AllocationKind::Reg => write!(f, "{}", self.as_reg().unwrap()),
            AllocationKind::Stack => write!(f, "{}", self.as_stack().unwrap()),
        }
    }
}

impl Allocation {
    /// Construct a new Allocation.
    #[inline(always)]
    pub(crate) fn new(kind: AllocationKind, index: usize) -> Self {
        debug_assert!(index < (1 << 28));
        Self {
            bits: ((kind as u8 as u32) << 29) | (index as u32),
        }
    }

    /// Get the "none" allocation, which is distinct from the other
    /// possibilities and is used to initialize data structures.
    #[inline(always)]
    pub fn none() -> Allocation {
        Allocation::new(AllocationKind::None, 0)
    }

    /// Create an allocation into a register.
    #[inline(always)]
    pub fn reg(preg: PReg) -> Allocation {
        Allocation::new(AllocationKind::Reg, preg.index())
    }

    /// Create an allocation into a spillslot.
    #[inline(always)]
    pub fn stack(slot: SpillSlot) -> Allocation {
        Allocation::new(AllocationKind::Stack, slot.bits as usize)
    }

    /// Get the allocation's "kind": none, register, or stack (spillslot).
    #[inline(always)]
    pub fn kind(self) -> AllocationKind {
        match (self.bits >> 29) & 7 {
            0 => AllocationKind::None,
            1 => AllocationKind::Reg,
            2 => AllocationKind::Stack,
            _ => unreachable!(),
        }
    }

    /// Is the allocation "none"?
    #[inline(always)]
    pub fn is_none(self) -> bool {
        self.kind() == AllocationKind::None
    }

    /// Is the allocation not "none"?
    #[inline(always)]
    pub fn is_some(self) -> bool {
        self.kind() != AllocationKind::None
    }

    /// Is the allocation a register?
    #[inline(always)]
    pub fn is_reg(self) -> bool {
        self.kind() == AllocationKind::Reg
    }

    /// Is the allocation on the stack (a spillslot)?
    #[inline(always)]
    pub fn is_stack(self) -> bool {
        self.kind() == AllocationKind::Stack
    }

    /// Get the index of the spillslot or register. If register, this
    /// is an index that can be used by `PReg::from_index()`.
    #[inline(always)]
    pub fn index(self) -> usize {
        (self.bits & ((1 << 28) - 1)) as usize
    }

    /// Get the allocation as a physical register, if any.
    #[inline(always)]
    pub fn as_reg(self) -> Option<PReg> {
        if self.kind() == AllocationKind::Reg {
            Some(PReg::from_index(self.index()))
        } else {
            None
        }
    }

    /// Get the allocation as a spillslot, if any.
    #[inline(always)]
    pub fn as_stack(self) -> Option<SpillSlot> {
        if self.kind() == AllocationKind::Stack {
            Some(SpillSlot {
                bits: self.index() as u32,
            })
        } else {
            None
        }
    }

    /// Get the raw bits for the packed encoding of this allocation.
    #[inline(always)]
    pub fn bits(self) -> u32 {
        self.bits
    }

    /// Construct an allocation from its packed encoding.
    #[inline(always)]
    pub fn from_bits(bits: u32) -> Self {
        debug_assert!(bits >> 29 >= 5);
        Self { bits }
    }
}

/// An allocation is one of two "kinds" (or "none"): register or
/// spillslot/stack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum AllocationKind {
    None = 0,
    Reg = 1,
    Stack = 2,
}

/// A trait defined by the regalloc client to provide access to its
/// machine-instruction / CFG representation.
///
/// (This trait's design is inspired by, and derives heavily from, the
/// trait of the same name in regalloc.rs.)
///
/// The ids used in [`Block`] and [`VReg`] must start their numbering
/// at zero and be sequential.
pub trait Function {
    // -------------
    // CFG traversal
    // -------------

    /// How many instructions are there?
    fn num_insts(&self) -> usize;

    /// How many blocks are there?
    fn num_blocks(&self) -> usize;

    /// Get the index of the entry block.
    fn entry_block(&self) -> Block;

    /// Provide the range of instruction indices contained in each block.
    fn block_insns(&self, block: Block) -> InstRange;

    /// Get CFG successors for a given block.
    fn block_succs(&self, block: Block) -> &[Block];

    /// Get the CFG predecessors for a given block.
    fn block_preds(&self, block: Block) -> &[Block];

    /// Get the block parameters for a given block.
    fn block_params(&self, block: Block) -> &[VReg];

    /// Determine whether an instruction is a return instruction.
    fn is_ret(&self, insn: Inst) -> bool;

    /// Determine whether an instruction is the end-of-block
    /// branch.
    fn is_branch(&self, insn: Inst) -> bool;

    /// If `insn` is a branch at the end of `block`, returns the
    /// outgoing blockparam arguments for the given successor. The
    /// number of arguments must match the number incoming blockparams
    /// for each respective successor block.
    fn branch_blockparams(&self, block: Block, insn: Inst, succ_idx: usize) -> &[VReg];

    // --------------------------
    // Instruction register slots
    // --------------------------

    /// Get the Operands for an instruction.
    fn inst_operands(&self, insn: Inst) -> &[Operand];

    /// Get the clobbers for an instruction; these are the registers
    /// that, after the instruction has executed, hold values that are
    /// arbitrary, separately from the usual outputs to the
    /// instruction. It is invalid to read a register that has been
    /// clobbered; the register allocator is free to assume that
    /// clobbered registers are filled with garbage and available for
    /// reuse. It will avoid storing any value in a clobbered register
    /// that must be live across the instruction.
    ///
    /// Another way of seeing this is that a clobber is equivalent to
    /// a "late def" of a fresh vreg that is not used anywhere else
    /// in the program, with a fixed-register constraint that places
    /// it in a given PReg chosen by the client prior to regalloc.
    ///
    /// Every register written by an instruction must either
    /// correspond to (be assigned to) an Operand of kind `Def`, or
    /// else must be a "clobber".
    ///
    /// This can be used to, for example, describe ABI-specified
    /// registers that are not preserved by a call instruction, or
    /// fixed physical registers written by an instruction but not
    /// used as a vreg output, or fixed physical registers used as
    /// temps within an instruction out of necessity.
    ///
    /// Note that clobbers and defs and late-uses must not collide: it
    /// is illegal to name the same physical register both as a clobber
    /// and in a fixed-register constraint on a def or late use.
    /// Internally, clobbers are modeled as defs (of throwaway vregs) at
    /// the instruction's late-point constrained to the named register.
    fn inst_clobbers(&self, insn: Inst) -> PRegSet;

    /// Get the number of `VReg` in use in this function.
    fn num_vregs(&self) -> usize;

    /// Get the VRegs for which we should generate value-location
    /// metadata for debugging purposes. This can be used to generate
    /// e.g. DWARF with valid prgram-point ranges for each value
    /// expression in a way that is more efficient than a post-hoc
    /// analysis of the allocator's output.
    ///
    /// Each tuple is (vreg, inclusive_start, exclusive_end,
    /// label). In the `Output` there will be (label, inclusive_start,
    /// exclusive_end, alloc)` tuples. The ranges may not exactly
    /// match -- specifically, the returned metadata may cover only a
    /// subset of the requested ranges -- if the value is not live for
    /// the entire requested ranges.
    ///
    /// The instruction indices imply a program point just *before*
    /// the instruction.
    ///
    /// Precondition: we require this slice to be sorted by vreg.
    fn debug_value_labels(&self) -> &[(VReg, Inst, Inst, u32)] {
        &[]
    }

    // --------------
    // Spills/reloads
    // --------------

    /// How many logical spill slots does the given regclass require?  E.g., on
    /// a 64-bit machine, spill slots may nominally be 64-bit words, but a
    /// 128-bit vector value will require two slots.  The regalloc will always
    /// align on this size.
    ///
    /// (This trait method's design and doc text derives from
    /// regalloc.rs' trait of the same name.)
    fn spillslot_size(&self, regclass: RegClass) -> usize;

    /// When providing a spillslot number for a multi-slot spillslot,
    /// do we provide the first or the last? This is usually related
    /// to which direction the stack grows and different clients may
    /// have different preferences.
    fn multi_spillslot_named_by_last_slot(&self) -> bool {
        false
    }

    // -----------
    // Misc config
    // -----------

    /// Allow a single instruction to define a vreg multiple times. If
    /// allowed, the semantics are as if the definition occurs only
    /// once, and all defs will get the same alloc. This flexibility is
    /// meant to allow the embedder to more easily aggregate operands
    /// together in macro/pseudoinstructions, or e.g. add additional
    /// clobbered vregs without taking care to deduplicate. This may be
    /// particularly useful when referring to physical registers via
    /// pinned vregs. It is optional functionality because a strict mode
    /// (at most one def per vreg) is also useful for finding bugs in
    /// other applications.
    fn allow_multiple_vreg_defs(&self) -> bool {
        false
    }
}

/// A position before or after an instruction at which we can make an
/// edit.
///
/// Note that this differs from `OperandPos` in that the former
/// describes specifically a constraint on an operand, while this
/// describes a program point. `OperandPos` could grow more options in
/// the future, for example if we decide that an "early write" or
/// "late read" phase makes sense, while `InstPosition` will always
/// describe these two insertion points.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum InstPosition {
    Before = 0,
    After = 1,
}

/// A program point: a single point before or after a given instruction.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ProgPoint {
    bits: u32,
}

impl core::fmt::Debug for ProgPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "progpoint{}{}",
            self.inst().index(),
            match self.pos() {
                InstPosition::Before => "-pre",
                InstPosition::After => "-post",
            }
        )
    }
}

impl ProgPoint {
    /// Create a new ProgPoint before or after the given instruction.
    #[inline(always)]
    pub fn new(inst: Inst, pos: InstPosition) -> Self {
        let bits = ((inst.0 as u32) << 1) | (pos as u8 as u32);
        Self { bits }
    }

    /// Create a new ProgPoint before the given instruction.
    #[inline(always)]
    pub fn before(inst: Inst) -> Self {
        Self::new(inst, InstPosition::Before)
    }

    /// Create a new ProgPoint after the given instruction.
    #[inline(always)]
    pub fn after(inst: Inst) -> Self {
        Self::new(inst, InstPosition::After)
    }

    /// Get the instruction that this ProgPoint is before or after.
    #[inline(always)]
    pub fn inst(self) -> Inst {
        // Cast to i32 to do an arithmetic right-shift, which will
        // preserve an `Inst::invalid()` (which is -1, or all-ones).
        Inst::new(((self.bits as i32) >> 1) as usize)
    }

    /// Get the "position" (Before or After) relative to the
    /// instruction.
    #[inline(always)]
    pub fn pos(self) -> InstPosition {
        match self.bits & 1 {
            0 => InstPosition::Before,
            1 => InstPosition::After,
            _ => unreachable!(),
        }
    }

    /// Get the "next" program point: for After, this is the Before of
    /// the next instruction, while for Before, this is After of the
    /// same instruction.
    #[inline(always)]
    pub fn next(self) -> ProgPoint {
        Self {
            bits: self.bits + 1,
        }
    }

    /// Get the "previous" program point, the inverse of `.next()`
    /// above.
    #[inline(always)]
    pub fn prev(self) -> ProgPoint {
        Self {
            bits: self.bits - 1,
        }
    }

    /// Convert to a raw encoding in 32 bits.
    #[inline(always)]
    pub fn to_index(self) -> u32 {
        self.bits
    }

    /// Construct from the raw 32-bit encoding.
    #[inline(always)]
    pub fn from_index(index: u32) -> Self {
        Self { bits: index }
    }

    #[inline(always)]
    pub fn invalid() -> Self {
        Self::before(Inst::new(usize::MAX))
    }
}

/// An instruction to insert into the program to perform some data movement.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum Edit {
    /// Move one allocation to another. Each allocation may be a
    /// register or a stack slot (spillslot). However, stack-to-stack
    /// moves will never be generated.
    ///
    /// `Move` edits will be generated even if src and dst allocation
    /// are the same if the vreg changes; this allows proper metadata
    /// tracking even when moves are elided.
    Move { from: Allocation, to: Allocation },
}

/// Wrapper around either an original instruction or an inserted edit.
#[derive(Clone, Debug)]
pub enum InstOrEdit<'a> {
    Inst(Inst),
    Edit(&'a Edit),
}

/// Iterator over the instructions and edits in a block.
pub struct OutputIter<'a> {
    /// List of edits starting at the first for the current block.
    edits: &'a [(ProgPoint, Edit)],

    /// Remaining instructions in the current block.
    inst_range: InstRange,
}

impl<'a> Iterator for OutputIter<'a> {
    type Item = InstOrEdit<'a>;

    fn next(&mut self) -> Option<InstOrEdit<'a>> {
        // There can't be any edits after the last instruction in a block, so
        // we don't need to worry about that case.
        if self.inst_range.len() == 0 {
            return None;
        }

        // Return any edits that happen before the next instruction first.
        let next_inst = self.inst_range.first();
        if let Some((edit, remaining_edits)) = self.edits.split_first() {
            if edit.0 <= ProgPoint::before(next_inst) {
                self.edits = remaining_edits;
                return Some(InstOrEdit::Edit(&edit.1));
            }
        }

        self.inst_range = self.inst_range.rest();
        Some(InstOrEdit::Inst(next_inst))
    }
}

/// A machine environment tells the register allocator which registers
/// are available to allocate and what register may be used as a
/// scratch register for each class, and some other miscellaneous info
/// as well.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct MachineEnv {
    /// Preferred physical registers for each class. These are the
    /// registers that will be allocated first, if free.
    ///
    /// If an explicit scratch register is provided in `scratch_by_class` then
    /// it must not appear in this list.
    pub preferred_regs_by_class: [Vec<PReg>; 3],

    /// Non-preferred physical registers for each class. These are the
    /// registers that will be allocated if a preferred register is
    /// not available; using one of these is considered suboptimal,
    /// but still better than spilling.
    ///
    /// If an explicit scratch register is provided in `scratch_by_class` then
    /// it must not appear in this list.
    pub non_preferred_regs_by_class: [Vec<PReg>; 3],

    /// Optional dedicated scratch register per class. This is needed to perform
    /// moves between registers when cyclic move patterns occur. The
    /// register should not be placed in either the preferred or
    /// non-preferred list (i.e., it is not otherwise allocatable).
    ///
    /// Note that the register allocator will freely use this register
    /// between instructions, but *within* the machine code generated
    /// by a single (regalloc-level) instruction, the client is free
    /// to use the scratch register. E.g., if one "instruction" causes
    /// the emission of two machine-code instructions, this lowering
    /// can use the scratch register between them.
    ///
    /// If a scratch register is not provided then the register allocator will
    /// automatically allocate one as needed, spilling a value to the stack if
    /// necessary.
    pub scratch_by_class: [Option<PReg>; 3],

    /// Some `PReg`s can be designated as locations on the stack rather than
    /// actual registers. These can be used to tell the register allocator about
    /// pre-defined stack slots used for function arguments and return values.
    ///
    /// `PReg`s in this list cannot be used as an allocatable or scratch
    /// register.
    pub fixed_stack_slots: Vec<PReg>,
}

/// The output of the register allocator.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Output {
    /// How many spillslots are needed in the frame?
    pub num_spillslots: usize,

    /// Edits (insertions or removals). Guaranteed to be sorted by
    /// program point.
    pub edits: Vec<(ProgPoint, Edit)>,

    /// Allocations for each operand. Mapping from instruction to
    /// allocations provided by `inst_alloc_offsets` below.
    pub allocs: Vec<Allocation>,

    /// Allocation offset in `allocs` for each instruction.
    pub inst_alloc_offsets: Vec<u32>,

    /// Debug info: a labeled value (as applied to vregs by
    /// `Function::debug_value_labels()` on the input side) is located
    /// in the given allocation from the first program point
    /// (inclusive) to the second (exclusive). Guaranteed to be sorted
    /// by label and program point, and the ranges are guaranteed to
    /// be disjoint.
    pub debug_locations: Vec<(u32, ProgPoint, ProgPoint, Allocation)>,

    /// Internal stats from the allocator.
    pub stats: ion::Stats,
}

impl Output {
    /// Get the allocations assigned to a given instruction.
    pub fn inst_allocs(&self, inst: Inst) -> &[Allocation] {
        let start = self.inst_alloc_offsets[inst.index()] as usize;
        let end = if inst.index() + 1 == self.inst_alloc_offsets.len() {
            self.allocs.len()
        } else {
            self.inst_alloc_offsets[inst.index() + 1] as usize
        };
        &self.allocs[start..end]
    }

    /// Returns an iterator over the instructions and edits in a block, in
    /// order.
    pub fn block_insts_and_edits(&self, func: &impl Function, block: Block) -> OutputIter<'_> {
        let inst_range = func.block_insns(block);

        let edit_idx = self
            .edits
            .binary_search_by(|&(pos, _)| {
                // This predicate effectively searches for a point *just* before
                // the first ProgPoint. This never returns Ordering::Equal, but
                // binary_search_by returns the index of where it would have
                // been inserted in Err.
                if pos < ProgPoint::before(inst_range.first()) {
                    core::cmp::Ordering::Less
                } else {
                    core::cmp::Ordering::Greater
                }
            })
            .unwrap_err();

        let edits = &self.edits[edit_idx..];
        OutputIter { inst_range, edits }
    }
}

/// An error that prevents allocation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum RegAllocError {
    /// Critical edge is not split between given blocks.
    CritEdge(Block, Block),
    /// Invalid SSA for given vreg at given inst: multiple defs or
    /// illegal use. `inst` may be `Inst::invalid()` if this concerns
    /// a block param.
    SSA(VReg, Inst),
    /// Invalid basic block: does not end in branch/ret, or contains a
    /// branch/ret in the middle, or the VReg ids do not start at zero
    /// or aren't numbered sequentially.
    BB(Block),
    /// Invalid branch: operand count does not match sum of block
    /// params of successor blocks, or the block ids do not start at
    /// zero or aren't numbered sequentially.
    Branch(Inst),
    /// A VReg is live-in on entry; this is not allowed.
    EntryLivein,
    /// A branch has non-blockparam arg(s) and at least one of the
    /// successor blocks has more than one predecessor, forcing
    /// edge-moves before this branch. This is disallowed because it
    /// places a use after the edge moves occur; insert an edge block
    /// to avoid the situation.
    DisallowedBranchArg(Inst),
    /// Too many pinned VRegs + Reg-constrained Operands are live at
    /// once, making allocation impossible.
    TooManyLiveRegs,
    /// Too many operands on a single instruction (beyond limit of
    /// 2^16 - 1).
    TooManyOperands,
}

impl core::fmt::Display for RegAllocError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RegAllocError {}

/// Run the allocator.
pub fn run<F: Function>(
    func: &F,
    env: &MachineEnv,
    options: &RegallocOptions,
) -> Result<Output, RegAllocError> {
    match options.algorithm {
        Algorithm::Ion => {
            let mut ctx = Ctx::default();
            run_with_ctx(func, env, options, &mut ctx)?;
            Ok(ctx.output)
        }
        Algorithm::Fastalloc => {
            fastalloc::run(func, env, options.verbose_log, options.validate_ssa)
        }
    }
}

/// Run the allocator with reusable context.
///
/// Return value points to `ctx.output` that can be alternatively `std::mem::take`n.
pub fn run_with_ctx<'a, F: Function>(
    func: &F,
    env: &MachineEnv,
    options: &RegallocOptions,
    ctx: &'a mut Ctx,
) -> Result<&'a Output, RegAllocError> {
    match options.algorithm {
        Algorithm::Ion => ion::run(func, env, ctx, options.verbose_log, options.validate_ssa)?,
        Algorithm::Fastalloc => {
            ctx.output = fastalloc::run(func, env, options.verbose_log, options.validate_ssa)?
        }
    }
    Ok(&ctx.output)
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Algorithm {
    #[default]
    Ion,
    Fastalloc,
}

/// Options for allocation.
#[derive(Clone, Copy, Debug, Default)]
pub struct RegallocOptions {
    /// Add extra verbosity to debug logs.
    pub verbose_log: bool,

    /// Run the SSA validator before allocating registers.
    pub validate_ssa: bool,

    /// The register allocation algorithm to be used.
    pub algorithm: Algorithm,
}

pub(crate) trait VecExt<T> {
    /// Fills `self` with `value` up to `len` and return the mutable slice to the values.
    fn repopulate(&mut self, len: usize, value: T) -> &mut [T]
    where
        T: Clone;
    /// Clears the `self` and returns a mutable reference to it.
    fn cleared(&mut self) -> &mut Self;
    /// Makes sure `self` is empty and has at least `cap` capacity.
    fn preallocate(&mut self, cap: usize) -> &mut Self;
}

impl<T> VecExt<T> for Vec<T> {
    fn repopulate(&mut self, len: usize, value: T) -> &mut [T]
    where
        T: Clone,
    {
        self.clear();
        self.resize(len, value);
        self
    }

    fn cleared(&mut self) -> &mut Self {
        self.clear();
        self
    }

    fn preallocate(&mut self, cap: usize) -> &mut Self {
        self.clear();
        self.reserve(cap);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Bump(Rc<bumpalo::Bump>);

impl Bump {
    pub(crate) fn get_mut(&mut self) -> Option<&mut bumpalo::Bump> {
        Rc::get_mut(&mut self.0)
    }
}

// Simply delegating because `Rc<bumpalo::Bump>` does not implement `Allocator`.
unsafe impl allocator_api2::alloc::Allocator for Bump {
    fn allocate(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        self.0.deref().allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        self.0.deref().deallocate(ptr, layout);
    }

    fn allocate_zeroed(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        self.0.deref().allocate_zeroed(layout)
    }

    unsafe fn grow(
        &self,
        ptr: core::ptr::NonNull<u8>,
        old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        self.0.deref().grow(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: core::ptr::NonNull<u8>,
        old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        self.0.deref().grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn shrink(
        &self,
        ptr: core::ptr::NonNull<u8>,
        old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        self.0.deref().shrink(ptr, old_layout, new_layout)
    }
}
