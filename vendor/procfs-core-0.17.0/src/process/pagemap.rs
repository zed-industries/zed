use bitflags::bitflags;
use std::{fmt, mem::size_of};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

const fn genmask(high: usize, low: usize) -> u64 {
    let mask_bits = size_of::<u64>() * 8;
    (!0 - (1 << low) + 1) & (!0 >> (mask_bits - 1 - high))
}

// source: include/linux/swap.h
const MAX_SWAPFILES_SHIFT: usize = 5;

// source: fs/proc/task_mmu.c
bitflags! {
    /// Represents the fields and flags in a page table entry for a swapped page.
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
    pub struct SwapPageFlags: u64 {
        /// Swap type if swapped
        #[doc(hidden)]
        const SWAP_TYPE = genmask(MAX_SWAPFILES_SHIFT - 1, 0);
        /// Swap offset if swapped
        #[doc(hidden)]
        const SWAP_OFFSET = genmask(54, MAX_SWAPFILES_SHIFT);
        /// PTE is soft-dirty
        const SOFT_DIRTY = 1 << 55;
        /// Page is exclusively mapped
        const MMAP_EXCLUSIVE = 1 << 56;
        /// Page is file-page or shared-anon
        const FILE = 1 << 61;
        /// Page is swapped
        #[doc(hidden)]
        const SWAP = 1 << 62;
        /// Page is present
        const PRESENT = 1 << 63;
    }
}

impl SwapPageFlags {
    /// Returns the swap type recorded in this entry.
    pub fn get_swap_type(&self) -> u64 {
        (*self & Self::SWAP_TYPE).bits()
    }

    /// Returns the swap offset recorded in this entry.
    pub fn get_swap_offset(&self) -> u64 {
        (*self & Self::SWAP_OFFSET).bits() >> MAX_SWAPFILES_SHIFT
    }
}

bitflags! {
    /// Represents the fields and flags in a page table entry for a memory page.
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
    pub struct MemoryPageFlags: u64 {
        /// Page frame number if present
        #[doc(hidden)]
        const PFN = genmask(54, 0);
        /// PTE is soft-dirty
        const SOFT_DIRTY = 1 << 55;
        /// Page is exclusively mapped
        const MMAP_EXCLUSIVE = 1 << 56;
        /// Page is file-page or shared-anon
        const FILE = 1 << 61;
        /// Page is swapped
        #[doc(hidden)]
        const SWAP = 1 << 62;
        /// Page is present
        const PRESENT = 1 << 63;
    }
}

impl MemoryPageFlags {
    /// Returns the page frame number recorded in this entry.
    pub fn get_page_frame_number(&self) -> Pfn {
        Pfn((*self & Self::PFN).bits())
    }
}

/// A Page Frame Number, representing a 4 kiB physical memory page
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pfn(pub u64);

impl fmt::UpperHex for Pfn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::UpperHex::fmt(&val, f)
    }
}

impl fmt::LowerHex for Pfn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::LowerHex::fmt(&val, f)
    }
}

/// Represents a page table entry in `/proc/<pid>/pagemap`.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageInfo {
    /// Entry referring to a memory page
    MemoryPage(MemoryPageFlags),
    /// Entry referring to a swapped page
    SwapPage(SwapPageFlags),
}

impl PageInfo {
    pub fn parse_info(info: u64) -> Self {
        let flags = MemoryPageFlags::from_bits_retain(info);

        if flags.contains(MemoryPageFlags::SWAP) {
            Self::SwapPage(SwapPageFlags::from_bits_retain(info))
        } else {
            Self::MemoryPage(flags)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genmask() {
        let mask = genmask(3, 1);
        assert_eq!(mask, 0b1110);

        let mask = genmask(3, 0);
        assert_eq!(mask, 0b1111);

        let mask = genmask(63, 62);
        assert_eq!(mask, 0b11 << 62);
    }

    #[test]
    fn test_page_info() {
        let pagemap_entry: u64 = 0b1000000110000000000000000000000000000000000000000000000000000011;
        let info = PageInfo::parse_info(pagemap_entry);
        if let PageInfo::MemoryPage(memory_flags) = info {
            assert!(memory_flags
                .contains(MemoryPageFlags::PRESENT | MemoryPageFlags::MMAP_EXCLUSIVE | MemoryPageFlags::SOFT_DIRTY));
            assert_eq!(memory_flags.get_page_frame_number(), Pfn(0b11));
        } else {
            panic!("Wrong SWAP decoding");
        }

        let pagemap_entry: u64 = 0b1100000110000000000000000000000000000000000000000000000001100010;
        let info = PageInfo::parse_info(pagemap_entry);
        if let PageInfo::SwapPage(swap_flags) = info {
            assert!(
                swap_flags.contains(SwapPageFlags::PRESENT | SwapPageFlags::MMAP_EXCLUSIVE | SwapPageFlags::SOFT_DIRTY)
            );
            assert_eq!(swap_flags.get_swap_type(), 0b10);
            assert_eq!(swap_flags.get_swap_offset(), 0b11);
        } else {
            panic!("Wrong SWAP decoding");
        }
    }
}
