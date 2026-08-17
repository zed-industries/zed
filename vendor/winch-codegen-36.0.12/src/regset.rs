use crate::isa::reg::{Reg, RegClass};

/// A bit set to track register availability.
pub(crate) struct RegSet {
    /// Bitset to track general purpose register availability.
    gpr: RegBitSet,
    /// Bitset to track floating-point register availability.
    fpr: RegBitSet,
}

use std::ops::{Index, IndexMut};

impl Index<RegClass> for RegSet {
    type Output = RegBitSet;

    fn index(&self, class: RegClass) -> &Self::Output {
        match class {
            RegClass::Int => &self.gpr,
            RegClass::Float => &self.fpr,
            c => unreachable!("Unexpected register class {:?}", c),
        }
    }
}

impl IndexMut<RegClass> for RegSet {
    fn index_mut(&mut self, class: RegClass) -> &mut Self::Output {
        match class {
            RegClass::Int => &mut self.gpr,
            RegClass::Float => &mut self.fpr,
            c => unreachable!("Unexpected register class {:?}", c),
        }
    }
}

/// Bitset for a particular register class.
pub struct RegBitSet {
    /// The register class.
    class: RegClass,
    /// The set of allocatable
    allocatable: u64,
    /// The set of non-alloctable registers.
    non_allocatable: u64,
    /// The max number of registers.
    /// Invariant:
    /// When allocating or freeing a register the encoding (index) of the
    /// register must be less than the max property.
    max: usize,
}

impl RegBitSet {
    /// Creates an integer register class bitset.
    pub fn int(allocatable: u64, non_allocatable: u64, max: usize) -> Self {
        // Assert that one set is the complement of the other.
        debug_assert!(allocatable & non_allocatable == 0);
        Self {
            class: RegClass::Int,
            allocatable,
            non_allocatable,
            max,
        }
    }

    /// Creates a float register class bitset.
    pub fn float(allocatable: u64, non_allocatable: u64, max: usize) -> Self {
        // Assert that one set is the complement of the other.
        debug_assert!(allocatable & non_allocatable == 0);
        Self {
            class: RegClass::Float,
            allocatable,
            non_allocatable,
            max,
        }
    }
}

impl RegSet {
    /// Create a new register set.
    pub fn new(gpr: RegBitSet, fpr: RegBitSet) -> Self {
        debug_assert!(gpr.class == RegClass::Int);
        debug_assert!(fpr.class == RegClass::Float);

        Self { gpr, fpr }
    }

    /// Allocate the next available register of the given class,
    /// returning `None` if there are no more registers available.
    pub fn reg_for_class(&mut self, class: RegClass) -> Option<Reg> {
        self.available(class).then(|| {
            let bitset = &self[class];
            let index = bitset.allocatable.trailing_zeros();
            self.allocate(class, index.into());
            Reg::from(class, index as usize)
        })
    }

    /// Request a specific register.
    pub fn reg(&mut self, reg: Reg) -> Option<Reg> {
        let index = reg.hw_enc();
        self.named_reg_available(reg).then(|| {
            self.allocate(reg.class(), index.try_into().unwrap());
            reg
        })
    }

    /// Marks the specified register as available, utilizing the
    /// register class to determine the bitset that requires updating.
    pub fn free(&mut self, reg: Reg) {
        let bitset = &self[reg.class()];
        let index = reg.hw_enc();
        assert!(index < bitset.max);
        let index = u64::try_from(index).unwrap();
        if !self.is_non_allocatable(reg.class(), index) {
            self[reg.class()].allocatable |= 1 << index;
        }
    }

    /// Returns true if the specified register is allocatable.
    pub fn named_reg_available(&self, reg: Reg) -> bool {
        let bitset = &self[reg.class()];
        assert!(reg.hw_enc() < bitset.max);
        let index = 1 << reg.hw_enc();

        (!bitset.allocatable & index) == 0
            || self.is_non_allocatable(reg.class(), reg.hw_enc().try_into().unwrap())
    }

    fn available(&self, class: RegClass) -> bool {
        let bitset = &self[class];
        bitset.allocatable != 0
    }

    fn allocate(&mut self, class: RegClass, index: u64) {
        if !self.is_non_allocatable(class, index) {
            self[class].allocatable &= !(1 << index);
        }
    }

    fn is_non_allocatable(&self, class: RegClass, index: u64) -> bool {
        let bitset = &self[class];
        let non_allocatable = bitset.non_allocatable;
        non_allocatable != 0 && !non_allocatable & (1 << index) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::{Reg, RegBitSet, RegClass, RegSet};

    const UNIVERSE: u64 = (1 << 16) - 1;
    const MAX: usize = 16;

    #[test]
    fn test_any_gpr() {
        let bitset = RegBitSet::int(UNIVERSE, !UNIVERSE, MAX);
        let zero = RegBitSet::float(0, 0, MAX);
        let mut set = RegSet::new(bitset, zero);
        for _ in 0..16 {
            let gpr = set.reg_for_class(RegClass::Int);
            assert!(gpr.is_some())
        }

        assert!(!set.available(RegClass::Int));
        assert!(set.reg_for_class(RegClass::Int).is_none())
    }

    #[test]
    fn test_gpr() {
        let non_allocatable: u64 = 1 << 5;
        let all = UNIVERSE & !non_allocatable;
        let non_alloc = Reg::int(5);
        let alloc = Reg::int(2);
        let bitset = RegBitSet::int(all, non_allocatable, MAX);
        let zero = RegBitSet::float(0, 0, MAX);
        let mut set = RegSet::new(bitset, zero);
        // Requesting a non alloctable register returns the register
        // and doesn't allocate it.
        assert!(set.reg(non_alloc).is_some());
        assert!(set.reg(non_alloc).is_some());
        // Requesting an allocatable register twice returns none the
        // second time.
        assert!(set.reg(alloc).is_some());
        assert!(set.reg(alloc).is_none());
    }

    #[test]
    fn test_free_reg() {
        let set = RegBitSet::int(UNIVERSE, !UNIVERSE, MAX);
        let zero = RegBitSet::float(0, 0, MAX);
        let mut set = RegSet::new(set, zero);
        let gpr = set.reg_for_class(RegClass::Int).unwrap();
        set.free(gpr);
        assert!(set.reg(gpr).is_some());
    }
}
