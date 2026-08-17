use crate::types::{DescriptorPoolCreateFlags, DescriptorTotalCount};

/// Memory exhausted error.
#[derive(Debug)]
pub enum CreatePoolError {
    /// Device memory exhausted.
    OutOfDeviceMemory,

    /// Host memory exhausted.
    OutOfHostMemory,

    /// A descriptor pool creation has failed due to fragmentation.
    Fragmentation,
}

/// Memory exhausted error.
#[derive(Debug)]
pub enum DeviceAllocationError {
    /// Device memory exhausted.
    OutOfDeviceMemory,

    /// Host memory exhausted.
    OutOfHostMemory,

    /// Failed to allocate memory from pool.
    OutOfPoolMemory,

    /// Pool allocation failed due to fragmentation of pool's memory.
    FragmentedPool,
}

/// Abstract device that can create pools of type `P` and allocate sets `S` with layout `L`.
pub trait DescriptorDevice<L, P, S> {
    /// Creates a new descriptor pool.
    ///
    /// # Safety
    ///
    /// Actually safe.
    /// TODO: Remove `unsafe` with next breaking change.
    unsafe fn create_descriptor_pool(
        &self,
        descriptor_count: &DescriptorTotalCount,
        max_sets: u32,
        flags: DescriptorPoolCreateFlags,
    ) -> Result<P, CreatePoolError>;

    /// Destroys descriptor pool.
    ///
    /// # Safety
    ///
    /// Pool must be created from this device.
    /// All descriptor sets allocated from this pool become invalid.
    unsafe fn destroy_descriptor_pool(&self, pool: P);

    /// Allocates descriptor sets.
    ///
    /// # Safety
    ///
    /// Pool must be created from this device.
    unsafe fn alloc_descriptor_sets<'a>(
        &self,
        pool: &mut P,
        layouts: impl ExactSizeIterator<Item = &'a L>,
        sets: &mut impl Extend<S>,
    ) -> Result<(), DeviceAllocationError>
    where
        L: 'a;

    /// Deallocates descriptor sets.
    ///
    /// # Safety
    ///
    /// Sets must be allocated from specified pool and not deallocated before.
    unsafe fn dealloc_descriptor_sets(&self, pool: &mut P, sets: impl Iterator<Item = S>);
}
