use alloc::{string::String, vec::Vec};
#[cfg(feature = "counters")]
use core::sync::atomic::{AtomicIsize, Ordering};
use core::{fmt, ops::Range};

/// An internal counter for debugging purposes
///
/// Internally represented as an atomic isize if the `counters` feature is enabled,
/// or compiles to nothing otherwise.
pub struct InternalCounter {
    #[cfg(feature = "counters")]
    value: AtomicIsize,
}

impl InternalCounter {
    /// Creates a counter with value 0.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        InternalCounter {
            #[cfg(feature = "counters")]
            value: AtomicIsize::new(0),
        }
    }

    /// Get the counter's value.
    #[cfg(feature = "counters")]
    #[inline]
    pub fn read(&self) -> isize {
        self.value.load(Ordering::Relaxed)
    }

    /// Get the counter's value.
    ///
    /// Always returns 0 if the `counters` feature is not enabled.
    #[cfg(not(feature = "counters"))]
    #[inline]
    #[must_use]
    pub fn read(&self) -> isize {
        0
    }

    /// Get and reset the counter's value.
    ///
    /// Always returns 0 if the `counters` feature is not enabled.
    #[cfg(feature = "counters")]
    #[inline]
    pub fn take(&self) -> isize {
        self.value.swap(0, Ordering::Relaxed)
    }

    /// Get and reset the counter's value.
    ///
    /// Always returns 0 if the `counters` feature is not enabled.
    #[cfg(not(feature = "counters"))]
    #[inline]
    #[must_use]
    pub fn take(&self) -> isize {
        0
    }

    /// Increment the counter by the provided amount.
    #[inline]
    pub fn add(&self, _val: isize) {
        #[cfg(feature = "counters")]
        self.value.fetch_add(_val, Ordering::Relaxed);
    }

    /// Decrement the counter by the provided amount.
    #[inline]
    pub fn sub(&self, _val: isize) {
        #[cfg(feature = "counters")]
        self.value.fetch_add(-_val, Ordering::Relaxed);
    }

    /// Sets the counter to the provided value.
    #[inline]
    pub fn set(&self, _val: isize) {
        #[cfg(feature = "counters")]
        self.value.store(_val, Ordering::Relaxed);
    }
}

impl Clone for InternalCounter {
    fn clone(&self) -> Self {
        InternalCounter {
            #[cfg(feature = "counters")]
            value: AtomicIsize::new(self.read()),
        }
    }
}

impl Default for InternalCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for InternalCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.read().fmt(f)
    }
}

/// `wgpu-hal`'s part of [`InternalCounters`].
#[allow(missing_docs)]
#[derive(Clone, Default)]
pub struct HalCounters {
    // API objects
    pub buffers: InternalCounter,
    pub textures: InternalCounter,
    pub texture_views: InternalCounter,
    pub bind_groups: InternalCounter,
    pub bind_group_layouts: InternalCounter,
    pub render_pipelines: InternalCounter,
    pub compute_pipelines: InternalCounter,
    pub pipeline_layouts: InternalCounter,
    pub samplers: InternalCounter,
    pub command_encoders: InternalCounter,
    pub shader_modules: InternalCounter,
    pub query_sets: InternalCounter,
    pub fences: InternalCounter,

    // Resources
    /// Amount of allocated gpu memory attributed to buffers, in bytes.
    pub buffer_memory: InternalCounter,
    /// Amount of allocated gpu memory attributed to textures, in bytes.
    pub texture_memory: InternalCounter,
    /// Amount of allocated gpu memory attributed to acceleration structures, in bytes.
    pub acceleration_structure_memory: InternalCounter,
    /// Number of gpu memory allocations.
    pub memory_allocations: InternalCounter,
}

/// `wgpu-core`'s part of [`InternalCounters`].
#[derive(Clone, Default)]
pub struct CoreCounters {
    // TODO    #[cfg(features=)]
}

/// All internal counters, exposed for debugging purposes.
///
/// Obtain this from
/// [`Device::get_internal_counters()`](../wgpu/struct.Device.html#method.get_internal_counters).
#[derive(Clone, Default)]
pub struct InternalCounters {
    /// `wgpu-core` counters.
    pub core: CoreCounters,
    /// `wgpu-hal` counters.
    pub hal: HalCounters,
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
    /// Sum of the memory reserved by all memory blocks including unallocated regions, in bytes.
    // XXX: Rename to total_capacity_bytes following the rename at https://github.com/Traverse-Research/gpu-allocator/pull/266?
    pub total_reserved_bytes: u64,
}

impl fmt::Debug for AllocationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = if !self.name.is_empty() {
            self.name.as_str()
        } else {
            "--"
        };
        write!(f, "{name:?}: {}", FmtBytes(self.size))
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
                    FmtBytes(self.total_allocated_bytes),
                    FmtBytes(self.total_reserved_bytes)
                ),
            )
            .field("blocks", &self.blocks.len())
            .field("allocations", &self.allocations.len())
            .field("largest", &allocations.as_slice())
            .finish()
    }
}

struct FmtBytes(u64);

impl fmt::Display for FmtBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const SUFFIX: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
        let mut idx = 0;
        let mut amount = self.0 as f64;
        loop {
            if amount < 1024.0 || idx == SUFFIX.len() - 1 {
                return write!(f, "{:.2} {}", amount, SUFFIX[idx]);
            }

            amount /= 1024.0;
            idx += 1;
        }
    }
}
