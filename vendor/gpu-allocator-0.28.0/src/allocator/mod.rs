#[cfg(feature = "std")]
use alloc::sync::Arc;
use alloc::{fmt, string::String, vec::Vec};
use core::ops::Range;
#[cfg(feature = "std")]
use std::backtrace::Backtrace;

use log::*;

use crate::result::*;

pub(crate) mod dedicated_block_allocator;
pub(crate) use dedicated_block_allocator::DedicatedBlockAllocator;

pub(crate) mod free_list_allocator;
pub(crate) use free_list_allocator::FreeListAllocator;

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub(crate) enum AllocationType {
    Free,
    Linear,
    NonLinear,
}

impl AllocationType {
    #[cfg(feature = "visualizer")]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Free => "Free",
            Self::Linear => "Linear",
            Self::NonLinear => "Non-Linear",
        }
    }
}

/// Describes an allocation in the [`AllocatorReport`].
#[derive(Clone)]
pub struct AllocationReport {
    /// The name provided to the `allocate()` function.
    pub name: String,
    /// The offset in bytes of the allocation in its memory block.
    pub offset: u64,
    /// The size in bytes of the allocation.
    pub size: u64,
    #[cfg(feature = "visualizer")]
    pub(crate) backtrace: Arc<Backtrace>,
}

/// Describes a memory block in the [`AllocatorReport`].
#[derive(Clone)]
pub struct MemoryBlockReport {
    /// The size in bytes of this memory block.
    pub size: u64,
    /// The range of allocations in [`AllocatorReport::allocations`] that are associated
    /// to this memory block.
    pub allocations: Range<usize>,
}

/// A report that can be generated for informational purposes using `Allocator::generate_report()`.
#[derive(Clone)]
pub struct AllocatorReport {
    /// All live allocations, sub-allocated from memory blocks.
    pub allocations: Vec<AllocationReport>,
    /// All memory blocks.
    pub blocks: Vec<MemoryBlockReport>,
    /// Sum of the memory used by all allocations, in bytes.
    pub total_allocated_bytes: u64,
    /// Sum of the memory capacity of all memory blocks including unallocated regions, in bytes.
    pub total_capacity_bytes: u64,
}

impl fmt::Debug for AllocationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = if !self.name.is_empty() {
            self.name.as_str()
        } else {
            "--"
        };
        write!(f, "{name:?}: {}", fmt_bytes(self.size))
    }
}

impl fmt::Debug for AllocatorReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut allocations = self.allocations.clone();
        allocations.sort_by_key(|alloc| core::cmp::Reverse(alloc.size));

        let max_num_allocations_to_print = f.precision().unwrap_or(usize::MAX);
        allocations.truncate(max_num_allocations_to_print);

        f.debug_struct("AllocatorReport")
            .field(
                "summary",
                &core::format_args!(
                    "{} / {}",
                    fmt_bytes(self.total_allocated_bytes),
                    fmt_bytes(self.total_capacity_bytes)
                ),
            )
            .field("blocks", &self.blocks.len())
            .field("allocations", &self.allocations.len())
            .field("largest", &allocations.as_slice())
            .finish()
    }
}

#[cfg(feature = "visualizer")]
pub(crate) trait SubAllocatorBase: crate::visualizer::SubAllocatorVisualizer {}
#[cfg(not(feature = "visualizer"))]
pub(crate) trait SubAllocatorBase {}

pub(crate) trait SubAllocator: SubAllocatorBase + fmt::Debug + Sync + Send {
    fn allocate(
        &mut self,
        size: u64,
        alignment: u64,
        allocation_type: AllocationType,
        granularity: u64,
        name: &str,
        #[cfg(feature = "std")] backtrace: Arc<Backtrace>,
    ) -> Result<(u64, core::num::NonZeroU64)>;

    fn free(&mut self, chunk_id: Option<core::num::NonZeroU64>) -> Result<()>;

    fn rename_allocation(
        &mut self,
        chunk_id: Option<core::num::NonZeroU64>,
        name: &str,
    ) -> Result<()>;

    fn report_memory_leaks(
        &self,
        log_level: Level,
        memory_type_index: usize,
        memory_block_index: usize,
    );

    fn report_allocations(&self) -> Vec<AllocationReport>;

    /// Returns [`true`] if this allocator allows sub-allocating multiple allocations, [`false`] if
    /// it is designed to only represent dedicated allocations.
    #[must_use]
    fn supports_general_allocations(&self) -> bool;
    #[must_use]
    fn allocated(&self) -> u64;

    /// Helper function: reports if the suballocator is empty (meaning, having no allocations).
    #[must_use]
    fn is_empty(&self) -> bool {
        self.allocated() == 0
    }
}

pub(crate) fn fmt_bytes(mut amount: u64) -> String {
    const SUFFIX: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    let mut idx = 0;
    let mut print_amount = amount as f64;
    loop {
        if amount < 1024 {
            return format!("{:.2} {}", print_amount, SUFFIX[idx]);
        }

        print_amount = amount as f64 / 1024.0;
        amount /= 1024;
        idx += 1;
    }
}
