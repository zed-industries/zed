//! Memory operation flags.

use super::TrapCode;
use core::fmt;
use core::num::NonZeroU8;
use core::str::FromStr;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// Endianness of a memory access.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Endianness {
    /// Little-endian
    Little,
    /// Big-endian
    Big,
}

/// Which disjoint region of aliasing memory is accessed in this memory
/// operation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
#[expect(missing_docs, reason = "self-describing variants")]
#[rustfmt::skip]
pub enum AliasRegion {
    // None = 0b00;
    Heap    = 0b01,
    Table   = 0b10,
    Vmctx   = 0b11,
}

impl AliasRegion {
    const fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            0b00 => None,
            0b01 => Some(Self::Heap),
            0b10 => Some(Self::Table),
            0b11 => Some(Self::Vmctx),
            _ => panic!("invalid alias region bits"),
        }
    }

    const fn to_bits(region: Option<Self>) -> u8 {
        match region {
            None => 0b00,
            Some(r) => r as u8,
        }
    }
}

/// Flags for memory operations like load/store.
///
/// Each of these flags introduce a limited form of undefined behavior. The flags each enable
/// certain optimizations that need to make additional assumptions. Generally, the semantics of a
/// program does not change when a flag is removed, but adding a flag will.
///
/// In addition, the flags determine the endianness of the memory access.  By default,
/// any memory access uses the native endianness determined by the target ISA.  This can
/// be overridden for individual accesses by explicitly specifying little- or big-endian
/// semantics via the flags.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct MemFlags {
    // Initialized to all zeros to have all flags have their default value.
    // This is interpreted through various methods below. Currently the bits of
    // this are defined as:
    //
    // * 0 - aligned flag
    // * 1 - readonly flag
    // * 2 - little endian flag
    // * 3 - big endian flag
    // * 4 - checked flag
    // * 5/6 - alias region
    // * 7/8/9/10/11/12/13/14 - trap code
    // * 15 - can_move flag
    //
    // Current properties upheld are:
    //
    // * only one of little/big endian is set
    // * only one alias region can be set - once set it cannot be changed
    bits: u16,
}

/// Guaranteed to use "natural alignment" for the given type. This
/// may enable better instruction selection.
const BIT_ALIGNED: u16 = 1 << 0;

/// A load that reads data in memory that does not change for the
/// duration of the function's execution. This may enable
/// additional optimizations to be performed.
const BIT_READONLY: u16 = 1 << 1;

/// Load multi-byte values from memory in a little-endian format.
const BIT_LITTLE_ENDIAN: u16 = 1 << 2;

/// Load multi-byte values from memory in a big-endian format.
const BIT_BIG_ENDIAN: u16 = 1 << 3;

/// Check this load or store for safety when using the
/// proof-carrying-code framework. The address must have a
/// `PointsTo` fact attached with a sufficiently large valid range
/// for the accessed size.
const BIT_CHECKED: u16 = 1 << 4;

/// Used for alias analysis, indicates which disjoint part of the abstract state
/// is being accessed.
const MASK_ALIAS_REGION: u16 = 0b11 << ALIAS_REGION_OFFSET;
const ALIAS_REGION_OFFSET: u16 = 5;

/// Trap code, if any, for this memory operation.
const MASK_TRAP_CODE: u16 = 0b1111_1111 << TRAP_CODE_OFFSET;
const TRAP_CODE_OFFSET: u16 = 7;

/// Whether this memory operation may be freely moved by the optimizer so long
/// as its data dependencies are satisfied. That is, by setting this flag, the
/// producer is guaranteeing that this memory operation's safety is not guarded
/// by outside-the-data-flow-graph properties, like implicit bounds-checking
/// control dependencies.
const BIT_CAN_MOVE: u16 = 1 << 15;

impl MemFlags {
    /// Create a new empty set of flags.
    pub const fn new() -> Self {
        Self { bits: 0 }.with_trap_code(Some(TrapCode::HEAP_OUT_OF_BOUNDS))
    }

    /// Create a set of flags representing an access from a "trusted" address, meaning it's
    /// known to be aligned and non-trapping.
    pub const fn trusted() -> Self {
        Self::new().with_notrap().with_aligned()
    }

    /// Read a flag bit.
    const fn read_bit(self, bit: u16) -> bool {
        self.bits & bit != 0
    }

    /// Return a new `MemFlags` with this flag bit set.
    const fn with_bit(mut self, bit: u16) -> Self {
        self.bits |= bit;
        self
    }

    /// Reads the alias region that this memory operation works with.
    pub const fn alias_region(self) -> Option<AliasRegion> {
        AliasRegion::from_bits(((self.bits & MASK_ALIAS_REGION) >> ALIAS_REGION_OFFSET) as u8)
    }

    /// Sets the alias region that this works on to the specified `region`.
    pub const fn with_alias_region(mut self, region: Option<AliasRegion>) -> Self {
        let bits = AliasRegion::to_bits(region);
        self.bits &= !MASK_ALIAS_REGION;
        self.bits |= (bits as u16) << ALIAS_REGION_OFFSET;
        self
    }

    /// Sets the alias region that this works on to the specified `region`.
    pub fn set_alias_region(&mut self, region: Option<AliasRegion>) {
        *self = self.with_alias_region(region);
    }

    /// Set a flag bit by name.
    ///
    /// Returns true if the flag was found and set, false for an unknown flag
    /// name.
    ///
    /// # Errors
    ///
    /// Returns an error message if the `name` is known but couldn't be applied
    /// due to it being a semantic error.
    pub fn set_by_name(&mut self, name: &str) -> Result<bool, &'static str> {
        *self = match name {
            "notrap" => self.with_trap_code(None),
            "aligned" => self.with_aligned(),
            "readonly" => self.with_readonly(),
            "little" => {
                if self.read_bit(BIT_BIG_ENDIAN) {
                    return Err("cannot set both big and little endian bits");
                }
                self.with_endianness(Endianness::Little)
            }
            "big" => {
                if self.read_bit(BIT_LITTLE_ENDIAN) {
                    return Err("cannot set both big and little endian bits");
                }
                self.with_endianness(Endianness::Big)
            }
            "heap" => {
                if self.alias_region().is_some() {
                    return Err("cannot set more than one alias region");
                }
                self.with_alias_region(Some(AliasRegion::Heap))
            }
            "table" => {
                if self.alias_region().is_some() {
                    return Err("cannot set more than one alias region");
                }
                self.with_alias_region(Some(AliasRegion::Table))
            }
            "vmctx" => {
                if self.alias_region().is_some() {
                    return Err("cannot set more than one alias region");
                }
                self.with_alias_region(Some(AliasRegion::Vmctx))
            }
            "checked" => self.with_checked(),
            "can_move" => self.with_can_move(),

            other => match TrapCode::from_str(other) {
                Ok(code) => self.with_trap_code(Some(code)),
                Err(()) => return Ok(false),
            },
        };
        Ok(true)
    }

    /// Return endianness of the memory access.  This will return the endianness
    /// explicitly specified by the flags if any, and will default to the native
    /// endianness otherwise.  The native endianness has to be provided by the
    /// caller since it is not explicitly encoded in CLIF IR -- this allows a
    /// front end to create IR without having to know the target endianness.
    pub const fn endianness(self, native_endianness: Endianness) -> Endianness {
        if self.read_bit(BIT_LITTLE_ENDIAN) {
            Endianness::Little
        } else if self.read_bit(BIT_BIG_ENDIAN) {
            Endianness::Big
        } else {
            native_endianness
        }
    }

    /// Return endianness of the memory access, if explicitly specified.
    ///
    /// If the endianness is not explicitly specified, this will return `None`,
    /// which means "native endianness".
    pub const fn explicit_endianness(self) -> Option<Endianness> {
        if self.read_bit(BIT_LITTLE_ENDIAN) {
            Some(Endianness::Little)
        } else if self.read_bit(BIT_BIG_ENDIAN) {
            Some(Endianness::Big)
        } else {
            None
        }
    }

    /// Set endianness of the memory access.
    pub fn set_endianness(&mut self, endianness: Endianness) {
        *self = self.with_endianness(endianness);
    }

    /// Set endianness of the memory access, returning new flags.
    pub const fn with_endianness(self, endianness: Endianness) -> Self {
        let res = match endianness {
            Endianness::Little => self.with_bit(BIT_LITTLE_ENDIAN),
            Endianness::Big => self.with_bit(BIT_BIG_ENDIAN),
        };
        assert!(!(res.read_bit(BIT_LITTLE_ENDIAN) && res.read_bit(BIT_BIG_ENDIAN)));
        res
    }

    /// Test if this memory operation cannot trap.
    ///
    /// By default `MemFlags` will assume that any load/store can trap and is
    /// associated with a `TrapCode::HeapOutOfBounds` code. If the trap code is
    /// configured to `None` though then this method will return `true` and
    /// indicates that the memory operation will not trap.
    ///
    /// If this returns `true` then the memory is *accessible*, which means
    /// that accesses will not trap. This makes it possible to delete an unused
    /// load or a dead store instruction.
    ///
    /// This flag does *not* mean that the associated instruction can be
    /// code-motioned to arbitrary places in the function so long as its data
    /// dependencies are met. This only means that, given its current location
    /// in the function, it will never trap. See the `can_move` method for more
    /// details.
    pub const fn notrap(self) -> bool {
        self.trap_code().is_none()
    }

    /// Sets the trap code for this `MemFlags` to `None`.
    pub fn set_notrap(&mut self) {
        *self = self.with_notrap();
    }

    /// Sets the trap code for this `MemFlags` to `None`, returning the new
    /// flags.
    pub const fn with_notrap(self) -> Self {
        self.with_trap_code(None)
    }

    /// Is this memory operation safe to move so long as its data dependencies
    /// remain satisfied?
    ///
    /// If this is `true`, then it is okay to code motion this instruction to
    /// arbitrary locations, in the function, including across blocks and
    /// conditional branches, so long as data dependencies (and trap ordering,
    /// if any) are upheld.
    ///
    /// If this is `false`, then this memory operation's safety potentially
    /// relies upon invariants that are not reflected in its data dependencies,
    /// and therefore it is not safe to code motion this operation. For example,
    /// this operation could be in a block that is dominated by a control-flow
    /// bounds check, which is not reflected in its operands, and it would be
    /// unsafe to code motion it above the bounds check, even if its data
    /// dependencies would still be satisfied.
    pub const fn can_move(self) -> bool {
        self.read_bit(BIT_CAN_MOVE)
    }

    /// Set the `can_move` flag.
    pub const fn set_can_move(&mut self) {
        *self = self.with_can_move();
    }

    /// Set the `can_move` flag, returning new flags.
    pub const fn with_can_move(self) -> Self {
        self.with_bit(BIT_CAN_MOVE)
    }

    /// Test if the `aligned` flag is set.
    ///
    /// By default, Cranelift memory instructions work with any unaligned effective address. If the
    /// `aligned` flag is set, the instruction is permitted to trap or return a wrong result if the
    /// effective address is misaligned.
    pub const fn aligned(self) -> bool {
        self.read_bit(BIT_ALIGNED)
    }

    /// Set the `aligned` flag.
    pub fn set_aligned(&mut self) {
        *self = self.with_aligned();
    }

    /// Set the `aligned` flag, returning new flags.
    pub const fn with_aligned(self) -> Self {
        self.with_bit(BIT_ALIGNED)
    }

    /// Test if the `readonly` flag is set.
    ///
    /// Loads with this flag have no memory dependencies.
    /// This results in undefined behavior if the dereferenced memory is mutated at any time
    /// between when the function is called and when it is exited.
    pub const fn readonly(self) -> bool {
        self.read_bit(BIT_READONLY)
    }

    /// Set the `readonly` flag.
    pub fn set_readonly(&mut self) {
        *self = self.with_readonly();
    }

    /// Set the `readonly` flag, returning new flags.
    pub const fn with_readonly(self) -> Self {
        self.with_bit(BIT_READONLY)
    }

    /// Test if the `checked` bit is set.
    ///
    /// Loads and stores with this flag are verified to access
    /// pointers only with a validated `PointsTo` fact attached, and
    /// with that fact validated, when using the proof-carrying-code
    /// framework. If initial facts on program inputs are correct
    /// (i.e., correctly denote the shape and types of data structures
    /// in memory), and if PCC validates the compiled output, then all
    /// `checked`-marked memory accesses are guaranteed (up to the
    /// checker's correctness) to access valid memory. This can be
    /// used to ensure memory safety and sandboxing.
    pub const fn checked(self) -> bool {
        self.read_bit(BIT_CHECKED)
    }

    /// Set the `checked` bit.
    pub fn set_checked(&mut self) {
        *self = self.with_checked();
    }

    /// Set the `checked` bit, returning new flags.
    pub const fn with_checked(self) -> Self {
        self.with_bit(BIT_CHECKED)
    }

    /// Get the trap code to report if this memory access traps.
    ///
    /// A `None` trap code indicates that this memory access does not trap.
    pub const fn trap_code(self) -> Option<TrapCode> {
        let byte = ((self.bits & MASK_TRAP_CODE) >> TRAP_CODE_OFFSET) as u8;
        match NonZeroU8::new(byte) {
            Some(code) => Some(TrapCode::from_raw(code)),
            None => None,
        }
    }

    /// Configures these flags with the specified trap code `code`.
    ///
    /// A trap code indicates that this memory operation cannot be optimized
    /// away and it must "stay where it is" in the programs. Traps are
    /// considered side effects, for example, and have meaning through the trap
    /// code that is communicated and which instruction trapped.
    pub const fn with_trap_code(mut self, code: Option<TrapCode>) -> Self {
        let bits = match code {
            Some(code) => code.as_raw().get() as u16,
            None => 0,
        };
        self.bits &= !MASK_TRAP_CODE;
        self.bits |= bits << TRAP_CODE_OFFSET;
        self
    }
}

impl fmt::Display for MemFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.trap_code() {
            None => write!(f, " notrap")?,
            // This is the default trap code, so don't print anything extra
            // for this.
            Some(TrapCode::HEAP_OUT_OF_BOUNDS) => {}
            Some(t) => write!(f, " {t}")?,
        }
        if self.aligned() {
            write!(f, " aligned")?;
        }
        if self.readonly() {
            write!(f, " readonly")?;
        }
        if self.can_move() {
            write!(f, " can_move")?;
        }
        if self.read_bit(BIT_BIG_ENDIAN) {
            write!(f, " big")?;
        }
        if self.read_bit(BIT_LITTLE_ENDIAN) {
            write!(f, " little")?;
        }
        if self.checked() {
            write!(f, " checked")?;
        }
        match self.alias_region() {
            None => {}
            Some(AliasRegion::Heap) => write!(f, " heap")?,
            Some(AliasRegion::Table) => write!(f, " table")?,
            Some(AliasRegion::Vmctx) => write!(f, " vmctx")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_traps() {
        for trap in TrapCode::non_user_traps().iter().copied() {
            let flags = MemFlags::new().with_trap_code(Some(trap));
            assert_eq!(flags.trap_code(), Some(trap));
        }
        let flags = MemFlags::new().with_trap_code(None);
        assert_eq!(flags.trap_code(), None);
    }

    #[test]
    fn cannot_set_big_and_little() {
        let mut big = MemFlags::new().with_endianness(Endianness::Big);
        assert!(big.set_by_name("little").is_err());

        let mut little = MemFlags::new().with_endianness(Endianness::Little);
        assert!(little.set_by_name("big").is_err());
    }

    #[test]
    fn only_one_region() {
        let mut big = MemFlags::new().with_alias_region(Some(AliasRegion::Heap));
        assert!(big.set_by_name("table").is_err());
        assert!(big.set_by_name("vmctx").is_err());

        let mut big = MemFlags::new().with_alias_region(Some(AliasRegion::Table));
        assert!(big.set_by_name("heap").is_err());
        assert!(big.set_by_name("vmctx").is_err());

        let mut big = MemFlags::new().with_alias_region(Some(AliasRegion::Vmctx));
        assert!(big.set_by_name("heap").is_err());
        assert!(big.set_by_name("table").is_err());
    }
}
